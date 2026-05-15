use std::fs::{self, File};
use std::io::BufWriter;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use hound::{WavSpec, WavWriter};
use screencapturekit::prelude::*;
use screencapturekit::stream::output_type::SCStreamOutputType;

const SAMPLE_RATE: u32 = 48_000;

type SharedWriter = Arc<Mutex<Option<WavWriter<BufWriter<File>>>>>;

pub struct Recorder {
    stream: SCStream,
    mic_writer: SharedWriter,
    sys_writer: SharedWriter,
    paused: Arc<AtomicBool>,
    dir: PathBuf,
}

pub struct RecorderState(pub Mutex<Option<Recorder>>);

fn timestamp() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let s = secs % 60;
    let m = (secs / 60) % 60;
    let h = (secs / 3600) % 24;
    let days = secs / 86400;
    let (y, mo, d) = epoch_days_to_date(days);
    format!("{:04}-{:02}-{:02}T{:02}-{:02}-{:02}Z", y, mo, d, h, m, s)
}

fn epoch_days_to_date(days: u64) -> (u64, u64, u64) {
    let mut remaining = days;
    let mut year = 1970u64;
    loop {
        let dy = if is_leap(year) { 366 } else { 365 };
        if remaining < dy { break; }
        remaining -= dy;
        year += 1;
    }
    let months = [31, if is_leap(year) { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut month = 1u64;
    for &dm in &months {
        if remaining < dm { break; }
        remaining -= dm;
        month += 1;
    }
    (year, month, remaining + 1)
}

fn is_leap(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn make_writer(path: &PathBuf) -> Result<WavWriter<BufWriter<File>>, String> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    WavWriter::create(path, spec).map_err(|e| e.to_string())
}

fn write_samples(writer: &SharedWriter, paused: &Arc<AtomicBool>, sample: CMSampleBuffer) {
    if paused.load(Ordering::Relaxed) { return; }
    let Some(list) = sample.audio_buffer_list() else { return; };

    let mut guard = match writer.lock() {
        Ok(g) => g,
        Err(p) => p.into_inner(),
    };
    let Some(w) = guard.as_mut() else { return; };

    for buf in list.iter() {
        let bytes = buf.data();
        // ScreenCaptureKit отдаёт Float32, non-interleaved: 4 байта на сэмпл.
        for chunk in bytes.chunks_exact(4) {
            let f = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            let clamped = f.clamp(-1.0, 1.0);
            let i = (clamped * i16::MAX as f32) as i16;
            let _ = w.write_sample(i);
        }
        // Один AudioBuffer == один канал. Берём только первый (моно).
        break;
    }
}

pub fn start(state: &RecorderState) -> Result<PathBuf, String> {
    let mut slot = state.0.lock().map_err(|e| e.to_string())?;
    if slot.is_some() {
        return Err("Recording already in progress".into());
    }

    let base = std::env::temp_dir().join(format!("speechext-{}", timestamp()));
    fs::create_dir_all(&base).map_err(|e| e.to_string())?;
    let mic_path = base.join("mic.wav");
    let sys_path = base.join("system.wav");

    let mic_writer: SharedWriter = Arc::new(Mutex::new(Some(make_writer(&mic_path)?)));
    let sys_writer: SharedWriter = Arc::new(Mutex::new(Some(make_writer(&sys_path)?)));
    let paused = Arc::new(AtomicBool::new(false));

    let content = SCShareableContent::get().map_err(|e| e.to_string())?;
    let display = content.displays().into_iter().next()
        .ok_or("No display available")?;
    let filter = SCContentFilter::create()
        .with_display(&display)
        .with_excluding_windows(&[])
        .build();
    let config = SCStreamConfiguration::new()
        .with_captures_audio(true)
        .with_captures_microphone(true)
        .with_sample_rate(SAMPLE_RATE as i32)
        .with_channel_count(1)
        .with_excludes_current_process_audio(true);

    let mut stream = SCStream::new(&filter, &config);

    let sys_w = sys_writer.clone();
    let sys_p = paused.clone();
    stream.add_output_handler(
        move |sample: CMSampleBuffer, _: SCStreamOutputType| {
            write_samples(&sys_w, &sys_p, sample);
        },
        SCStreamOutputType::Audio,
    );

    let mic_w = mic_writer.clone();
    let mic_p = paused.clone();
    stream.add_output_handler(
        move |sample: CMSampleBuffer, _: SCStreamOutputType| {
            write_samples(&mic_w, &mic_p, sample);
        },
        SCStreamOutputType::Microphone,
    );

    stream.start_capture().map_err(|e| e.to_string())?;

    *slot = Some(Recorder {
        stream,
        mic_writer,
        sys_writer,
        paused,
        dir: base.clone(),
    });

    Ok(base)
}

pub fn pause(state: &RecorderState) -> Result<(), String> {
    let slot = state.0.lock().map_err(|e| e.to_string())?;
    let rec = slot.as_ref().ok_or("Not recording")?;
    rec.paused.store(true, Ordering::Relaxed);
    Ok(())
}

pub fn resume(state: &RecorderState) -> Result<(), String> {
    let slot = state.0.lock().map_err(|e| e.to_string())?;
    let rec = slot.as_ref().ok_or("Not recording")?;
    rec.paused.store(false, Ordering::Relaxed);
    Ok(())
}

pub fn stop(state: &RecorderState) -> Result<PathBuf, String> {
    let mut slot = state.0.lock().map_err(|e| e.to_string())?;
    let rec = slot.take().ok_or("Not recording")?;

    rec.stream.stop_capture().map_err(|e| e.to_string())?;

    if let Ok(mut g) = rec.mic_writer.lock() {
        if let Some(w) = g.take() { w.finalize().map_err(|e| e.to_string())?; }
    }
    if let Ok(mut g) = rec.sys_writer.lock() {
        if let Some(w) = g.take() { w.finalize().map_err(|e| e.to_string())?; }
    }

    Ok(rec.dir)
}

#[tauri::command]
pub fn start_recording(state: tauri::State<RecorderState>) -> Result<String, String> {
    start(&state).map(|p| p.to_string_lossy().to_string())
}

#[tauri::command]
pub fn pause_recording(state: tauri::State<RecorderState>) -> Result<(), String> {
    pause(&state)
}

#[tauri::command]
pub fn resume_recording(state: tauri::State<RecorderState>) -> Result<(), String> {
    resume(&state)
}

#[tauri::command]
pub fn stop_recording(state: tauri::State<RecorderState>) -> Result<String, String> {
    stop(&state).map(|p| p.to_string_lossy().to_string())
}

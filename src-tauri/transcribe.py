import sys
import json
import os
from faster_whisper import WhisperModel

def main():
    if len(sys.argv) < 2:
        print(json.dumps({"error": "No files provided"}))
        sys.exit(1)

    file_paths = sys.argv[1:]

    for path in file_paths:
        if not os.path.isfile(path):
            print(json.dumps({"error": f"File not found: {path}"}))
            sys.exit(1)

    total = len(file_paths)
    model = WhisperModel("base", device="cpu", compute_type="int8")

    segments_all = []
    detected_language = None
    detected_language_prob = None

    for i, file_path in enumerate(file_paths):
        progress = {"progress": i + 1, "total": total, "file": os.path.basename(file_path)}
        print(json.dumps(progress), file=sys.stderr, flush=True)

        try:
            segments, info = model.transcribe(file_path, vad_filter=True, beam_size=1)
            text = " ".join(seg.text.strip() for seg in segments).strip()

            if detected_language is None:
                detected_language = info.language
                detected_language_prob = round(info.language_probability, 3)

            if text:
                segments_all.append({
                    "index": i,
                    "file": os.path.basename(file_path),
                    "text": text
                })

        except Exception as e:
            segments_all.append({
                "index": i,
                "file": os.path.basename(file_path),
                "text": "",
                "error": str(e)
            })

    full_text = " ".join(s["text"] for s in segments_all if s.get("text"))

    print(json.dumps({
        "language": detected_language,
        "language_probability": detected_language_prob,
        "total_files": total,
        "segments": segments_all,
        "full_text": full_text
    }, ensure_ascii=False))

if __name__ == "__main__":
    main()

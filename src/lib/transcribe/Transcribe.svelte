<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { listen } from "@tauri-apps/api/event";
  import { onDestroy } from "svelte";

  type Segment = { index: number; file: string; text: string; error?: string };
  type Result = {
    language: string;
    language_probability: number;
    total_files: number;
    segments: Segment[];
    full_text: string;
  };

  let { onsave } = $props<{ onsave: () => void }>();

  let files = $state<string[]>([]);
  let progress = $state<{ current: number; total: number; file: string } | null>(null);
  let result = $state<Result | null>(null);
  let error = $state<string | null>(null);
  let loading = $state(false);
  let saving = $state(false);
  let saved = $state(false);

  let folderPath = $derived(
    files.length > 0
      ? files[0].substring(0, files[0].lastIndexOf("/"))
      : null
  );

  let sortedNames = $derived(
    [...files]
      .sort((a, b) => {
        const num = (s: string) => {
          const name = s.split("/").pop() ?? "";
          const match = name.match(/^audio_(\d+)\.ogg$/);
          return match ? parseInt(match[1]) : Infinity;
        };
        return num(a) - num(b);
      })
      .map((f) => f.split("/").pop() ?? f)
  );

  const unlisten = listen<{ current: number; total: number; file: string }>(
    "transcribe-progress",
    (event) => {
      progress = event.payload;
    }
  );

  onDestroy(async () => {
    (await unlisten)();
  });

  async function pickFiles() {
    const selected = await open({
      multiple: true,
      filters: [{ name: "Audio", extensions: ["ogg", "wav", "mp3", "m4a", "flac"] }],
    });
    if (Array.isArray(selected) && selected.length > 0) {
      files = selected;
      result = null;
      error = null;
      progress = null;
      saved = false;
    }
  }

  async function transcribe() {
    if (files.length === 0) return;
    loading = true;
    error = null;
    result = null;
    progress = null;
    saved = false;

    try {
      const raw = await invoke<string>("transcribe_files", { files });
      const parsed = JSON.parse(raw);
      if (parsed.error) {
        error = parsed.error;
      } else {
        result = parsed;
      }
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
      progress = null;
    }
  }

  async function save() {
    if (!result) return;
    saving = true;
    try {
      const payload = JSON.stringify({ ...result, type: "audio-text" });
      await invoke<string>("save_result", { payload });
      saved = true;
      onsave();
    } catch (e) {
      error = String(e);
    } finally {
      saving = false;
    }
  }
</script>

<div>
  <div>
    <button onclick={pickFiles}>Choose path</button>
    {#if folderPath}
      <span>{folderPath}</span>
    {/if}
  </div>

  {#if files.length > 0}
    <p>Selected files: {files.length}</p>
    <ul>
      {#each sortedNames as name}
        <li>{name}</li>
      {/each}
    </ul>

    <button onclick={transcribe} disabled={loading}>
      {loading ? "Transcribing..." : "Transcribe"}
    </button>
  {/if}

  {#if progress}
    <p>{progress.current} / {progress.total} — {progress.file}</p>
    <progress value={progress.current} max={progress.total}></progress>
  {/if}

  {#if error}
    <p>Error: {error}</p>
  {/if}

  {#if result}
    <p>
      Language: {result.language}
      ({(result.language_probability * 100).toFixed(0)}%) —
      files: {result.total_files}
    </p>
    <p>{result.full_text}</p>

    <button onclick={save} disabled={saving || saved}>
      {saved ? "Saved" : saving ? "Saving..." : "Save"}
    </button>
  {/if}
</div>

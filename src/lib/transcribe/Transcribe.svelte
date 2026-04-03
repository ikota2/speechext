<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';

  let filePath = $state<string | null>(null);
  let result = $state<{ text: string; language: string; language_probability: number } | null>(null);
  let error = $state<string | null>(null);
  let loading = $state(false);

  async function pickFile() {
    const selected = await open({
      multiple: false,
      filters: [{ name: 'Audio', extensions: ['ogg', 'wav', 'mp3', 'm4a', 'flac'] }],
    });
    if (typeof selected === 'string') {
      filePath = selected;
      result = null;
      error = null;
    }
  }

  async function transcribe() {
    if (!filePath) return;
    loading = true;
    error = null;
    result = null;

    try {
      const raw = await invoke<string>('transcribe_file', { path: filePath });
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
    }
  }
</script>

<div>
  <button onclick={pickFile}>Выбрать файл</button>

  {#if filePath}
    <p>{filePath}</p>
    <button onclick={transcribe} disabled={loading}>
      {loading ? 'Транскрибирую...' : 'Транскрибировать'}
    </button>
  {/if}

  {#if error}
    <p>Ошибка: {error}</p>
  {/if}

  {#if result}
    <p>Язык: {result.language} ({(result.language_probability * 100).toFixed(0)}%)</p>
    <textarea readonly rows={8}>{result.text}</textarea>
  {/if}
</div>

<script lang="ts">
  import { onDestroy } from 'svelte';
  import { transcribeFiles, listenProgress } from '$lib/services/transcribe.js';
  import { resultStore } from '$lib/stores/result.svelte.js';
  import type { ProgressPayload } from '$lib/types.js';
  import FilePicker from './FilePicker.svelte';
  import ProgressBar from '$lib/ui/ProgressBar.svelte';

  let files = $state<string[]>([]);
  let language = $state('en');
  let progress = $state<ProgressPayload | null>(null);
  let error = $state<string | null>(null);
  let loading = $state(false);

  const unlisten = listenProgress((payload) => {
    progress = payload;
  });

  onDestroy(async () => {
    (await unlisten)();
  });

  async function transcribe() {
    if (files.length === 0) return;
    loading = true;
    error = null;
    progress = null;

    try {
      const result = await transcribeFiles(files, language);
      resultStore.setUnsaved({ ...result, type: 'audio-text' });
      files = [];
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
      progress = null;
    }
  }
</script>

<div>
  <FilePicker bind:files onchange={() => { error = null; }} />

  <select bind:value={language}>
    <option value="en">English</option>
    <option value="es">Spanish</option>
    <option value="ru">Russian</option>
  </select>

  {#if files.length > 0}
    <button onclick={transcribe} disabled={loading}>
      {loading ? 'Transcribing...' : 'Transcribe'}
    </button>
  {/if}

  {#if progress}
    <ProgressBar current={progress.current} total={progress.total} label={progress.file} />
  {/if}

  {#if error}
    <p>Error: {error}</p>
  {/if}
</div>

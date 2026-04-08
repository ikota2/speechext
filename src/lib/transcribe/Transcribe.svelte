<script lang="ts">
  import { onDestroy } from 'svelte';
  import { transcribeFiles, listenProgress } from '$lib/services/transcribe.js';
  import { saveResult } from '$lib/services/history.js';
  import { historyStore } from '$lib/stores/history.svelte.js';
  import type { TranscribeResult, ProgressPayload } from '$lib/types.js';
  import FilePicker from './FilePicker.svelte';
  import ProgressBar from '$lib/ui/ProgressBar.svelte';
  import ResultCard from '$lib/ui/ResultCard.svelte';

  let files = $state<string[]>([]);
  let progress = $state<ProgressPayload | null>(null);
  let result = $state<TranscribeResult | null>(null);
  let error = $state<string | null>(null);
  let loading = $state(false);
  let saving = $state(false);
  let saved = $state(false);

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
    result = null;
    progress = null;
    saved = false;

    try {
      result = await transcribeFiles(files);
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
      await saveResult({ ...result, type: 'audio-text' });
      saved = true;
      await historyStore.refresh();
    } catch (e) {
      error = String(e);
    } finally {
      saving = false;
    }
  }
</script>

<div>
  <FilePicker bind:files onchange={() => { result = null; error = null; saved = false; }} />

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

  {#if result}
    <ResultCard
        text={result.full_text}
        language={result.language}
        languageProbability={result.language_probability}
        totalFiles={result.total_files}
        onsave={save}
        {saving}
        {saved}
    />
  {/if}
</div>

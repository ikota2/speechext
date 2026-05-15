<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  type RecordStatus = 'idle' | 'recording' | 'paused';
  let status = $state<RecordStatus>('idle');
  let outputDir = $state<string | null>(null);
  let error = $state<string | null>(null);

  async function start() {
    try {
      outputDir = null;
      error = null;
      await invoke('start_recording');
      status = 'recording';
    } catch (e) { error = String(e); }
  }

  async function pause() {
    try {
      await invoke('pause_recording');
      status = 'paused';
    } catch (e) { error = String(e); }
  }

  async function resume() {
    try {
      await invoke('resume_recording');
      status = 'recording';
    } catch (e) { error = String(e); }
  }

  async function stop() {
    try {
      outputDir = await invoke<string>('stop_recording');
      status = 'idle';
    } catch (e) { error = String(e); }
  }
</script>

<div>
  {#if status === 'idle'}
    <button onclick={start}>Start</button>
  {:else if status === 'recording'}
    <button onclick={pause}>Pause</button>
    <button onclick={stop}>Stop</button>
  {:else if status === 'paused'}
    <button onclick={resume}>Resume</button>
    <button onclick={stop}>Stop</button>
  {/if}

  <p>Status: {status}</p>
  {#if outputDir}
    <p>Saved to: {outputDir}</p>
  {/if}
  {#if error}
    <p>Error: {error}</p>
  {/if}
</div>

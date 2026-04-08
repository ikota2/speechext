<script lang="ts">
import Record from '$lib/record/Record.svelte';
import Transcribe from '$lib/transcribe/Transcribe.svelte';
import History from '$lib/history/History.svelte';

type FullResult = {
  type: string;
  full_text: string;
  language: string;
  language_probability: number;
  total_files: number;
  title?: string;
};

let history = $state<{ refresh: () => void } | null>(null);
let selected = $state<FullResult | null>(null);
</script>


<div class="wrapper">
  <main>
    <div class="controls">
      <Transcribe onsave={() => history?.refresh()} />
      <Record />
    </div>
    <div class="result">
      {#if selected}
        <p>{selected.full_text}</p>
      {/if}
    </div>
  </main>
  <aside>
    <History bind:this={history} onselect={(r) => (selected = r)} />
  </aside>
</div>

<style>
  .wrapper {
		background-color: #e5eff2;
		height: 100dvh;
		width: 100vw;
		display: grid;
		grid-template-columns: 3fr 1fr;
  }

  .controls {
    display: flex;
    gap: 10px;
  }
</style>

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


{#if selected}
  <p>{selected.full_text}</p>
{/if}

<Transcribe onsave={() => history?.refresh()} />
<History bind:this={history} onselect={(r) => (selected = r)} />
<Record />

<style>
	:global(body) {
		margin: 0;
		padding: 0;
	}
</style>

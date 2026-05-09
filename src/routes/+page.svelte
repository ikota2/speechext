<script lang="ts">
  import Transcribe from '$lib/transcribe/Transcribe.svelte';
  import Record from '$lib/record/Record.svelte';
  import ResultCard from '$lib/ui/ResultCard.svelte';
  import { resultStore } from '$lib/stores/result.svelte.js';
</script>

<div class="controls">
  <Transcribe />
  <Record />
</div>

{#if resultStore.result}
  <ResultCard
      text={resultStore.result.full_text}
      onsave={() => resultStore.save()}
      saving={resultStore.saving}
      saved={resultStore.isSaved}
  />
{/if}

<style>
	.controls {
		display: flex;
		gap: 10px;
	}
</style>

<script lang="ts">
  import Transcribe from '$lib/transcribe/Transcribe.svelte';
  import Record from '$lib/record/Record.svelte';
  import { selectedStore } from '$lib/stores/selected.svelte.js';
</script>

<div class="controls">
  <Transcribe />
  <Record />
</div>

{#if selectedStore.result}
  <div class="result">
    <p>{selectedStore.result.full_text}</p>
  </div>
{/if}

<style>
	.controls {
		display: flex;
		gap: 10px;
	}
</style>

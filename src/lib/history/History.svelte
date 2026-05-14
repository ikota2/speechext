<script lang="ts">
  import { historyStore } from '$lib/stores/history.svelte.js';
  import { resultStore } from '$lib/stores/result.svelte.js';
  import { loadResult } from '$lib/services/history.js';

  async function select(filename: string) {
    const result = await loadResult(filename);
    resultStore.select(filename, result);
    console.log();
  }

  function formatDate(created_at: string) {
    const iso = created_at.replace(/T(\d{2})-(\d{2})-(\d{2})Z/, 'T$1:$2:$3Z');
    return new Date(iso).toLocaleString();
  }

  $effect(() => {
    historyStore.loadIfNeeded();
  });

  let audioText = $derived(historyStore.items.filter((i) => i.result_type === 'audio-text'));
  let micText = $derived(historyStore.items.filter((i) => i.result_type === 'mic-text'));
</script>

<div class="history">
  {#if audioText.length > 0}
    <p>Audio → Text</p>
    <ul>
      {#each audioText as item}
        <li>
          <button
            onclick={() => select(item.filename)}
            class:active={item.filename === resultStore.filename}
          >{item.title ?? formatDate(item.created_at)}</button>
        </li>
      {/each}
    </ul>
  {/if}

  {#if micText.length > 0}
    <p>Mic → Text</p>
    <ul>
      {#each micText as item}
        <li>
          <button
            onclick={() => select(item.filename)}
            class:active={item.filename === resultStore.filename}
          >{item.title ?? formatDate(item.created_at)}</button>
        </li>
      {/each}
    </ul>
  {/if}

  {#if historyStore.items.length === 0}
    <p>No saved results yet.</p>
  {/if}
</div>
<style>
 .history {
   padding: 0 25px;
   position: fixed;
   border-left: 1px solid darkslateblue;
   height: 100%;
 }
 ul {
   list-style: none;
 }
 button {
	 border-radius: 4px;
   border: none;
	 background-color: transparent;
   cursor: pointer;
	 font-size: 24px;
	 font-weight: 200;
 }
 button.active {
	 background-color: oklch(90% 0.015 220);
	 font-weight: 400;
 }
</style>

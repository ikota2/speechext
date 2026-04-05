<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  type ResultMeta = {
    filename: string;
    created_at: string;
    result_type: string;
    title?: string;
  };

  type FullResult = {
    type: string;
    full_text: string;
    language: string;
    language_probability: number;
    total_files: number;
    title?: string;
  };

  let { onselect } = $props<{ onselect: (result: FullResult) => void }>();

  let items = $state<ResultMeta[]>([]);

  export async function refresh() {
    items = await invoke<ResultMeta[]>("list_results");
  }

  async function select(filename: string) {
    const raw = await invoke<string>("load_result", { filename });
    const parsed: FullResult = JSON.parse(raw);
    onselect(parsed);
  }

  function formatDate(created_at: string) {
    // created_at is like "2026-04-04T13-30-00Z"
    const iso = created_at.replace(/T(\d{2})-(\d{2})-(\d{2})Z/, "T$1:$2:$3Z");
    return new Date(iso).toLocaleString();
  }

  $effect(() => {
    refresh();
  });

  let audioText = $derived(items.filter((i) => i.result_type === "audio-text"));
  let micText = $derived(items.filter((i) => i.result_type === "mic-text"));
</script>

<div>
  {#if audioText.length > 0}
    <p>Audio → Text</p>
    <ul>
      {#each audioText as item}
        <li>
          <button onclick={() => select(item.filename)}>
            {item.title ?? formatDate(item.created_at)}
          </button>
        </li>
      {/each}
    </ul>
  {/if}

  {#if micText.length > 0}
    <p>Mic → Text</p>
    <ul>
      {#each micText as item}
        <li>
          <button onclick={() => select(item.filename)}>
            {item.title ?? formatDate(item.created_at)}
          </button>
        </li>
      {/each}
    </ul>
  {/if}

  {#if items.length === 0}
    <p>No saved results yet.</p>
  {/if}
</div>

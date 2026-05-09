<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog';

  let { files = $bindable<string[]>([]), onchange }: {
    files?: string[];
    onchange?: () => void;
  } = $props();

  let folderPath = $derived(
    files.length > 0
      ? files[0].substring(0, files[0].lastIndexOf('/'))
      : null
  );

  let sortedNames = $derived(
    [...files]
      .sort((a, b) => {
        const num = (s: string) => {
          const name = s.split('/').pop() ?? '';
          const match = name.match(/^audio_(\d+)\.ogg$/);
          return match ? parseInt(match[1]) : Infinity;
        };
        return num(a) - num(b);
      })
      .map((f) => f.split('/').pop() ?? f)
  );

  async function pick() {
    const selected = await open({
      multiple: true,
      filters: [{ name: 'Audio', extensions: ['ogg', 'wav', 'mp3', 'm4a', 'flac'] }],
    });
    if (Array.isArray(selected) && selected.length > 0) {
      files = selected;
      onchange?.();
    }
  }
</script>

<div>
  <div>
    <button onclick={pick} class="button">Choose path</button>
    {#if folderPath}
      <span>{folderPath}</span>
    {/if}
  </div>
</div>

<style>
	.button {
		font-size: 16px;
	}
</style>

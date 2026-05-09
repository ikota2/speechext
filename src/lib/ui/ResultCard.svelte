<script lang="ts">
  let {
    text,
    onsave,
    saving = false,
    saved = false,
  }: {
    text: string;
    onsave: () => void;
    saving?: boolean;
    saved?: boolean;
  } = $props();
</script>

<div>
  <p>{text}</p>
  <button onclick={onsave} disabled={saving || saved} class="button">
    {saved ? 'Saved' : saving ? 'Saving...' : 'Save'}
  </button>
</div>

<style>
	.button {
		font-size: 16px;
	}
</style>

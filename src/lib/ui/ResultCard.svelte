<script lang="ts">
  let {
    text,
    language,
    languageProbability,
    totalFiles,
    onsave,
    saving = false,
    saved = false,
  }: {
    text: string;
    language: string;
    languageProbability: number;
    totalFiles: number;
    onsave: () => void;
    saving?: boolean;
    saved?: boolean;
  } = $props();
</script>

<div>
  <p>
    Language: {language}
    ({(languageProbability * 100).toFixed(0)}%) —
    files: {totalFiles}
  </p>
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

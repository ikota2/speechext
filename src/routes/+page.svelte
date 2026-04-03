<script lang="ts">
import Record from '$lib/record/Record.svelte';
import Transcribe from '$lib/transcribe/Transcribe.svelte';
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>

<Transcribe />
<Record />

<style>
	:global(body) {
		margin: 0;
		padding: 0;
	}
</style>

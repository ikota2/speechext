<script lang="ts">
  import favicon from '$lib/assets/favicon.svg';
  import History from '$lib/history/History.svelte';

  let { children } = $props();
</script>

<svelte:head>
  <link rel="icon" href={favicon} />
</svelte:head>

<div class="wrapper">
  <main>
    {@render children()}
  </main>
  <aside>
    <History />
  </aside>
</div>

<style>
	:global(*) {
		margin: 0;
		padding: 0;
	}
	.wrapper {
		background-color: #e5eff2;
		height: 100dvh;
		width: 100vw;
		display: grid;
		grid-template-columns: 3fr 1fr;
	}
</style>

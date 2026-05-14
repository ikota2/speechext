<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { createTabs } from './tabs.svelte.ts';

  let { items, tabs }: {
    items: { title: string; content: Snippet }[];
    tabs: ReturnType<typeof createTabs>;
  } = $props();
</script>

<div>
  <div class="titles">
    {#each items as item, i}
      <button onclick={() => tabs.set(i)} class="title" class:active={tabs.isActive(i)}>
        {item.title}
      </button>
    {/each}
  </div>
  {@render items[tabs.active].content()}
</div>

<style>
	.titles {
		display: flex;
		gap: 10px;
	}
  .title {
    cursor: pointer;
    font-size: 24px;
		font-weight: 200;
    background-color: transparent;
    border-left: 2px solid black;
    border-right: 2px solid black;
    border: 0;
  }
  .title:hover {
    background-color: #fae2eb;
    transition: 2ms ease-in;
  }
	.active {
		font-weight: 400;
	}
</style>

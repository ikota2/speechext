<script lang="ts">
  import Transcribe from '$lib/transcribe/Transcribe.svelte';
  import Record from '$lib/record/Record.svelte';
  import ResultCard from '$lib/ui/ResultCard.svelte';
  import {createTabs} from '$lib/ui/tabs.svelte.js';
  import { resultStore } from '$lib/stores/result.svelte.js';
  import Tabs from '$lib/ui/Tabs.svelte';

  const tabs = createTabs(2);
  const tabItems = [
    { title: 'Open', content: openTab },
    { title: 'Record', content: recordTab },
  ];
</script>

<Tabs
  {tabs}
  items={tabItems}
/>

{#snippet openTab()}
  <Transcribe />
{/snippet}

{#snippet recordTab()}
  <Record />
{/snippet}

{#if resultStore.result}
  <ResultCard
      text={resultStore.result.full_text}
      onsave={() => resultStore.save()}
      saving={resultStore.saving}
      saved={resultStore.isSaved}
  />
{/if}

<style>

</style>

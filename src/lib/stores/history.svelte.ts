import { listResults } from '$lib/services/history.js';
import type { ResultMeta } from '$lib/types.js';

let items = $state<ResultMeta[]>([]);
let loaded = $state(false);

export const historyStore = {
  get items() { return items; },

  async refresh() {
    items = await listResults();
    loaded = true;
  },

  async loadIfNeeded() {
    if (!loaded) await this.refresh();
  }
};

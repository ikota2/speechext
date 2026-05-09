import { saveResult } from '$lib/services/history.js';
import { historyStore } from '$lib/stores/history.svelte.js';
import type { SavedResult } from '$lib/types.js';

let result = $state<SavedResult | null>(null);
let filename = $state<string | null>(null);
let isSaved = $state(false);
let saving = $state(false);

export const resultStore = {
  get result() { return result; },
  get filename() { return filename; },
  get isSaved() { return isSaved; },
  get saving() { return saving; },

  setUnsaved(r: SavedResult) {
    result = r;
    filename = null;
    isSaved = false;
  },

  select(name: string, r: SavedResult) {
    result = r;
    filename = name;
    isSaved = true;
  },

  async save() {
    if (!result || isSaved) return;
    saving = true;
    try {
      await saveResult(result);
      await historyStore.refresh();
      isSaved = true;
    } finally {
      saving = false;
    }
  },

  clear() {
    result = null;
    filename = null;
    isSaved = false;
  }
};

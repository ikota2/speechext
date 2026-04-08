import type { SavedResult } from '$lib/types.js';

let result = $state<SavedResult | null>(null);

export const selectedStore = {
  get result() { return result; },
  select(r: SavedResult) { result = r; },
  clear() { result = null; }
};

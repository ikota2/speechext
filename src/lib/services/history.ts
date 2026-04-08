import { invoke } from '@tauri-apps/api/core';
import type { ResultMeta, SavedResult } from '$lib/types.js';

export async function saveResult(result: SavedResult): Promise<void> {
  const payload = JSON.stringify(result);
  await invoke<string>('save_result', { payload });
}

export async function listResults(): Promise<ResultMeta[]> {
  return invoke<ResultMeta[]>('list_results');
}

export async function loadResult(filename: string): Promise<SavedResult> {
  const raw = await invoke<string>('load_result', { filename });
  return JSON.parse(raw);
}

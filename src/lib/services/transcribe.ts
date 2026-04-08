import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { TranscribeResult, ProgressPayload } from '$lib/types.js';

export async function transcribeFiles(files: string[], language: string): Promise<TranscribeResult> {
  const raw = await invoke<string>('transcribe_files', { files, language });
  const parsed = JSON.parse(raw);
  if (parsed.error) throw new Error(parsed.error);
  return parsed;
}

export function listenProgress(callback: (payload: ProgressPayload) => void) {
  return listen<ProgressPayload>('transcribe-progress', (event) => {
    callback(event.payload);
  });
}

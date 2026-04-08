export type Segment = {
  index: number;
  file: string;
  text: string;
  error?: string;
};

export type TranscribeResult = {
  language: string;
  language_probability: number;
  total_files: number;
  segments: Segment[];
  full_text: string;
};

export type SavedResult = TranscribeResult & {
  type: string;
  title?: string;
};

export type ResultMeta = {
  filename: string;
  created_at: string;
  result_type: string;
  title?: string;
};

export type ProgressPayload = {
  current: number;
  total: number;
  file: string;
};

<script lang="ts">
  let {
    current,
    total,
    label = '',
  }: {
    current: number;
    total: number;
    label?: string;
  } = $props();
</script>

<div>
  <p>{current} / {total}{label ? ` — ${label}` : ''}</p>
  <progress value={current} max={total}></progress>
</div>

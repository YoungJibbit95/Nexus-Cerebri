<script lang="ts">
  let { name, size = 20 }: { name: string; size?: number } = $props();
  const paths: Record<string, string> = {
    planner: 'M4 4h16v16H4z M4 9h16 M9 9v11 M14 13h3 M14 17h3',
    trace: 'M5 4v12a3 3 0 0 0 3 3h11 M5 8h8 M13 5l3 3-3 3 M15 16l3 3-3 3',
    semantics: 'M12 3v18 M3 12h18 M5.6 5.6l12.8 12.8 M5.6 18.4 18.4 5.6',
    preferences: 'M4 6h16 M4 12h16 M4 18h16 M8 3v6 M16 9v6 M10 15v6',
    ml: 'M12 3 4 7v10l8 4 8-4V7z M4 7l8 5 8-5 M12 12v9',
    dataset: 'M4 5c0-4 16-4 16 0s-16 4-16 0 M4 5v14c0 4 16 4 16 0V5 M4 12c0 4 16 4 16 0',
    play: 'm8 5 11 7-11 7z',
    arrow: 'M5 12h14 M13 6l6 6-6 6',
    upload: 'M12 16V3 M7 8l5-5 5 5 M4 16v5h16v-5',
    download: 'M12 3v13 M7 11l5 5 5-5 M4 16v5h16v-5',
    terminal: 'm4 6 5 6-5 6 M12 18h8',
    check: 'm5 12 4 4L19 6',
    sun: 'M12 2v2 M12 20v2 M2 12h2 M20 12h2 M5 5l1 1 M18 18l1 1 M5 19l1-1 M18 6l1-1 M16 12a4 4 0 1 0-8 0 4 4 0 0 0 8 0',
    clock: 'M12 7v5l3 2 M21 12a9 9 0 1 0-18 0 9 9 0 0 0 18 0',
    code: 'm8 6-6 6 6 6 M16 6l6 6-6 6 M14 3l-4 18',
    shield: 'm12 3 8 3v6c0 5-8 9-8 9s-8-4-8-9V6z M8 12l3 3 5-6',
  };
</script>

<svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d={paths[name] ?? paths.code} /></svg>

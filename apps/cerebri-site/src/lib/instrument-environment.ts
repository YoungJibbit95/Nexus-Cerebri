// Chapter state changes only. Scroll pixels never drive environmental transforms.
export function instrumentEnvironment(node: HTMLElement) {
  // Initial hash targets settle after hydration, responsive scene layout and fonts.
  // A real navigation gesture cancels this one-time correction; scrolling stays native.
  const initialHash = location.hash;
  let anchorFrame = 0;
  let anchorCancelled = false;
  const anchorListeners = new AbortController();
  const cancelAnchor = () => { anchorCancelled = true; };
  if (initialHash) {
    for (const event of ['wheel', 'touchstart', 'pointerdown', 'keydown']) window.addEventListener(event, cancelAnchor, { passive: true, signal: anchorListeners.signal });
    void document.fonts.ready.then(() => {
      if (anchorCancelled) return;
      anchorFrame = requestAnimationFrame(() => {
        anchorFrame = requestAnimationFrame(() => {
          if (!anchorCancelled && location.hash === initialHash) {
            let id = initialHash.slice(1);
            try { id = decodeURIComponent(id); } catch { /* A malformed fragment has no decoded target. */ }
            const target = document.getElementById(id);
            if (target && node.contains(target)) target.scrollIntoView({ behavior: 'instant', block: 'start' });
          }
          anchorListeners.abort();
        });
      });
    });
  }
  const chapters = [...node.querySelectorAll<HTMLElement>('[data-world]')];
  const update = () => {
    const current = chapters.filter(chapter => chapter.getBoundingClientRect().top <= innerHeight * .5).at(-1);
    document.body.dataset.instrumentWorld = current?.dataset.world ?? 'space';
  };
  const observer = new IntersectionObserver(update, { rootMargin: '-20% 0px -45% 0px' });
  chapters.forEach(chapter => observer.observe(chapter));
  window.addEventListener('resize', update);
  update();
  return { destroy() { anchorCancelled = true; anchorListeners.abort(); cancelAnimationFrame(anchorFrame); observer.disconnect(); window.removeEventListener('resize', update); delete document.body.dataset.instrumentWorld; } };
}

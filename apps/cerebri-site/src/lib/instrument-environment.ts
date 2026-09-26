// Chapter state changes only. Scroll pixels never drive environmental transforms.
export function instrumentEnvironment(node: HTMLElement) {
  const chapters = [...node.querySelectorAll<HTMLElement>('[data-world]')];
  const update = () => {
    const current = chapters.filter(chapter => chapter.getBoundingClientRect().top <= innerHeight * .5).at(-1);
    document.body.dataset.instrumentWorld = current?.dataset.world ?? 'space';
  };
  const observer = new IntersectionObserver(update, { rootMargin: '-20% 0px -45% 0px' });
  chapters.forEach(chapter => observer.observe(chapter));
  window.addEventListener('resize', update);
  update();
  return { destroy() { observer.disconnect(); window.removeEventListener('resize', update); delete document.body.dataset.instrumentWorld; } };
}

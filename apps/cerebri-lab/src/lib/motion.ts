/** One cancellable, bounded post-result explanation. No planner runtime trace. */
export const STORY_LAST_STAGE = 5;
export const STORY_STEP_MS = 600;
export interface StoryScheduler { schedule: (callback: () => void, delay: number) => unknown; cancel: (handle: unknown) => void }
export function createStoryPlayback(onstage: (stage: number) => void, scheduler: StoryScheduler = {
  schedule: (callback, delay) => setTimeout(callback, delay), cancel: (handle) => clearTimeout(handle as ReturnType<typeof setTimeout>),
}) {
  let pending: unknown;
  let generation = 0;
  function cancel() { generation++; if (pending !== undefined) scheduler.cancel(pending); pending = undefined; }
  function finish() { cancel(); onstage(STORY_LAST_STAGE); }
  function play(reduced = false) {
    cancel();
    if (reduced) { onstage(STORY_LAST_STAGE); return; }
    const current = generation;
    let stage = 0;
    onstage(stage);
    function advance() {
      if (current !== generation) return;
      onstage(++stage);
      pending = stage < STORY_LAST_STAGE ? scheduler.schedule(advance, STORY_STEP_MS) : undefined;
    }
    pending = scheduler.schedule(advance, STORY_STEP_MS);
  }
  return { play, cancel, finish };
}

import { expect, test } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import { runtimeData } from '../src/lib/generated/runtime-data';
import { createHash } from 'node:crypto';
import { writeFile } from 'node:fs/promises';
import { visualBaselines } from './visual-baselines';

const prefix = process.env.CEREBRI_BASE_PATH ?? '';

test('explanation length cannot shift the native scroll scene boundaries', async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 1100 });
  await page.goto(prefix + '/');
  await page.evaluate(() => document.fonts.ready);
  const scene = page.locator('.observatory');
  const heights: number[] = [];
  for (let stage = 0; stage < 7; stage++) {
    await scene.locator('.scene-controls button').nth(stage).click();
    await expect(scene).toHaveAttribute('data-scene', String(stage));
    await expect(scene.locator('.scene-explanation').getByRole('heading')).toHaveCount(1);
    heights.push(await scene.evaluate(element => element.getBoundingClientRect().height));
  }
  expect(Math.max(...heights) - Math.min(...heights)).toBeLessThanOrEqual(1);
});

test('manual scene inspection survives fast scrolling, resizing and reduced motion', async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 1100 });
  await page.goto(prefix + '/');
  const scene = page.locator('.observatory');
  await scene.locator('.scene-controls button').nth(3).press('Enter');
  await expect(scene).toHaveAttribute('data-scene', '3');
  await page.evaluate(() => scrollTo({ top: document.body.scrollHeight, behavior: 'instant' }));
  await page.evaluate(() => scrollTo({ top: 0, behavior: 'instant' }));
  await page.setViewportSize({ width: 390, height: 700 });
  await expect(scene).toHaveAttribute('data-scene', '3');
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await expect(scene).toHaveAttribute('data-scene', '3');
  await expect(scene.locator('.coordinate-volume')).toHaveCSS('transition-duration', '0s');
  await scene.getByRole('button', { name: 'Follow scroll', exact: true }).click();
  await expect(scene).toHaveAttribute('data-scene', '6');
  await page.goto(prefix + '/#structured-descent');
  await expect(page.locator('#descent-title')).toBeInViewport();
  await page.reload();
  await expect(page.locator('#descent-title')).toBeInViewport();
});

test('rejection inspection retains candidate identity and the complete Rust witness', async ({ page }) => {
  await page.goto(prefix + '/');
  const witness = page.locator('.rejection-witness');
  const endpoints = await page.locator('.planning-instrument .capsule-origin').elementHandles();
  for (const [index, rejection] of runtimeData.plannerResult.conflicts.rejections.entries()) {
    await witness.getByRole('button').nth(index).press('Enter');
    const highlighted = page.locator('.planning-instrument .candidate-lanes li.inspected');
    await expect(highlighted).toHaveAttribute('data-start', rejection.start);
    await expect(highlighted).toHaveAttribute('data-state', 'rejected');
    await expect(witness.locator('.witness-reason')).toContainText(rejection.reasons[0].HardConstraint.reason);
    await witness.locator('details').evaluate(node => (node as HTMLDetailsElement).open = true);
    expect(JSON.parse(await witness.locator('pre code').innerText())).toEqual(rejection);
    expect(await endpoints[index].evaluate((node, start) => node === document.querySelector(`.planning-instrument [data-start="${start}"] .capsule-origin`), rejection.start)).toBe(true);
  }
  expect((await new AxeBuilder({ page }).include('.rejection-witness').analyze()).violations).toEqual([]);
});

for (const width of [1440, 390, 320]) test(`structured descent keeps endpoints and readable records at ${width}px`, async ({ page }) => {
  await page.setViewportSize({ width, height: 900 });
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.goto(prefix + '/#structured-descent');
  const descent = page.locator('.representation-descent');
  const endpoints = await descent.locator('.range-endpoint').elementHandles();
  const range = runtimeData.plannerResult.candidates[0].proposed.placements[0].range;
  for (const textSize of ['100%', '200%']) {
    await page.evaluate(size => document.documentElement.style.fontSize = size, textSize);
    for (const label of ['Interval', 'Named fields', 'Exact record']) {
      await descent.getByRole('button', { name: label, exact: true }).click();
      expect(await descent.locator('.range-endpoint').evaluateAll(nodes => nodes.map(node => node.getAttribute('data-value')))).toEqual([range.start, range.end]);
      expect(await endpoints[0].evaluate(node => node === document.querySelector('.representation-descent .range-endpoint.start'))).toBe(true);
      expect(await page.evaluate(() => document.documentElement.scrollWidth - innerWidth)).toBeLessThanOrEqual(1);
      const overlaps = await descent.locator('.range-endpoint').evaluateAll(nodes => {
        const a = nodes[0].getBoundingClientRect(), b = nodes[1].getBoundingClientRect();
        return a.left < b.right && a.right > b.left && a.top < b.bottom && a.bottom > b.top;
      });
      expect(overlaps).toBe(false);
    }
  }
  expect((await new AxeBuilder({ page }).include('.representation-descent').analyze()).violations).toEqual([]);
});

test('record landing performance and verify idle reduced-motion behavior', async ({ page }, info) => {
  await page.setViewportSize({ width: 1440, height: 1100 });
  await page.addInitScript(() => {
    const samples = { longTasks: [] as number[], layoutShifts: [] as number[] };
    Object.assign(window, { landingPerformance: samples });
    new PerformanceObserver(list => samples.longTasks.push(...list.getEntries().map(entry => entry.duration))).observe({ type: 'longtask', buffered: true });
    new PerformanceObserver(list => samples.layoutShifts.push(...list.getEntries().filter(entry => !(entry as PerformanceEntry & { hadRecentInput: boolean }).hadRecentInput).map(entry => (entry as PerformanceEntry & { value: number }).value))).observe({ type: 'layout-shift', buffered: true });
  });
  await page.goto(prefix + '/');
  for (const phase of [0, 1, 3, 4, 5, 6]) await page.locator('.scene-controls button').nth(phase).click();
  await page.locator('.rejection-witness').scrollIntoViewIfNeeded();
  await page.locator('#structured-descent').scrollIntoViewIfNeeded();
  const measurements = await page.evaluate(() => ({
    ...(window as Window & { landingPerformance?: object }).landingPerformance,
    paint: performance.getEntriesByType('paint').map(entry => ({ name: entry.name, ms: entry.startTime })),
    resources: performance.getEntriesByType('resource').map(entry => ({ name: entry.name.split('/').at(-1), bytes: (entry as PerformanceResourceTiming).transferSize }))
  }));
  const performancePath = info.outputPath('landing-performance.json');
  await writeFile(performancePath, JSON.stringify(measurements, null, 2));
  await info.attach('landing-performance.json', { path: performancePath, contentType: 'application/json' });
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await expect.poll(() => page.evaluate(() => document.getAnimations().filter(animation => animation.playState === 'running' && animation.effect?.getTiming().iterations === Infinity).length)).toBe(0);
});

for (const width of [1440, 390]) test(`@visual structured descent ${width}px`, async ({ page }) => {
  await page.setViewportSize({ width, height: 1000 });
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.goto(prefix + '/#structured-descent');
  await page.evaluate(() => document.fonts.ready);
  const name = `descent-${width}`;
  const screenshot = await page.locator('.representation-descent').screenshot({ path: test.info().outputPath(name + '.png'), animations: 'disabled', style: '.site-header{visibility:hidden!important}' });
  await test.info().attach(name + '.png', { body: screenshot, contentType: 'image/png' });
  expect(createHash('sha256').update(screenshot).digest('hex')).toBe(visualBaselines[process.platform]?.[name]);
});

import { expect, test, type Page } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import { createHash } from 'node:crypto';
import { visualBaselines } from './visual-baselines';

const prefix = process.env.CEREBRI_BASE_PATH ?? '';

async function scrub(page: Page, progress: number) {
  const track = page.locator('.observatory-track');
  await expect(track).not.toHaveClass(/static-view/);
  await track.evaluate((element, p) => {
    const surface = element.querySelector<HTMLElement>('.observatory')!;
    const top = Number.parseFloat(getComputedStyle(surface).top);
    scrollTo({ top: scrollY + element.getBoundingClientRect().top - top + p * (element.clientHeight - surface.clientHeight), behavior: 'instant' });
  }, progress);
  await expect.poll(async () => Math.abs(Number(await page.locator('.observatory').getAttribute('data-progress')) - progress)).toBeLessThan(.002);
}

for (const viewport of [{ width: 1440, height: 1000 }, { width: 390, height: 844 }]) {
  test(`brain continuously scrubs, holds and reassembles at ${viewport.width}px`, async ({ page }) => {
    await page.setViewportSize(viewport);
    await page.goto(prefix + '/');
    const brain = page.locator('.observatory .cerebri-brain');
    await expect(brain).toHaveAttribute('data-activation', 'ready');
    await expect(brain.locator('[data-module]')).toHaveCount(9);
    const skin = brain.locator('[data-outline="scope"]');
    const origin = await brain.locator('[data-candidate-id="C05"] .capsule-origin').elementHandle();
    await scrub(page, .21);
    const first = await skin.boundingBox();
    await scrub(page, .23);
    const next = await skin.boundingBox();
    expect(Math.abs(next!.x - first!.x) + Math.abs(next!.y - first!.y)).toBeGreaterThan(1);
    const held = await skin.getAttribute('transform');
    await page.waitForTimeout(180);
    expect(await skin.getAttribute('transform')).toBe(held);
    await scrub(page, .4);
    const scope = await skin.boundingBox();
    const known = await brain.locator('[data-outline="known-state"]').boundingBox();
    expect(scope!.y + scope!.height).toBeLessThan(known!.y);
    await scrub(page, .735);
    await scrub(page, 1);
    const spine = await brain.locator('.temporal-spine').boundingBox();
    expect(viewport.width > 700 ? spine!.width > spine!.height * 10 : spine!.height > spine!.width * 10).toBe(true);
    const read = (selector: string) => page.locator(selector).evaluateAll(nodes => nodes.map(n => [n.getAttribute('data-candidate-id'), n.getAttribute('data-start'), n.getAttribute('data-state')]));
    expect(await read('.observatory .brain-candidate')).toEqual(await read('.planning-instrument .candidate-lanes li'));
    await scrub(page, .21);
    const reversed = await skin.boundingBox();
    expect(Math.abs(reversed!.x - first!.x) + Math.abs(reversed!.y - first!.y)).toBeLessThan(1);
    expect(await origin!.evaluate(node => node === document.querySelector('.observatory [data-candidate-id="C05"] .capsule-origin'))).toBe(true);
    expect(await page.evaluate(() => document.documentElement.scrollWidth - innerWidth)).toBeLessThanOrEqual(1);
  });
}

test('brain retains its spine, shell and known-state shape through the planning morph', async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 1000 });
  await page.goto(prefix + '/');
  const nodes = await page.locator('.observatory .temporal-spine,.observatory [data-outline="scope"],.observatory [data-outline="known-state"]').elementHandles();
  await scrub(page, .82);
  const before = await page.locator('.observatory [data-outline="scope"]').getAttribute('d');
  await scrub(page, .91);
  const during = await page.locator('.observatory [data-outline="scope"]').getAttribute('d');
  await scrub(page, 1);
  const after = await page.locator('.observatory [data-outline="scope"]').getAttribute('d');
  expect(new Set([before, during, after]).size).toBe(3);
  for (const node of nodes) expect(await node.evaluate(n => n.isConnected)).toBe(true);
  const proposal = await page.locator('.observatory [data-outline="proposal"]').boundingBox();
  const selected = await page.locator('.observatory [data-candidate-id="C05"] .capsule-origin').boundingBox();
  expect(selected!.x).toBeGreaterThanOrEqual(proposal!.x - 1);
  expect(selected!.x).toBeLessThan(proposal!.x + proposal!.width);
  await scrub(page, .3);
  for (const node of nodes) expect(await node.evaluate(n => n.isConnected)).toBe(true);
});

test('manual inspection clears the desktop introduction and short phones retain the full silhouette', async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 1000 });
  await page.goto(prefix + '/');
  await page.locator('.scene-controls button').nth(6).click();
  await expect(page.locator('.hero-introduction')).not.toBeVisible();
  await expect(page.getByRole('link', { name: 'Deterministic Ordering', exact: false })).toBeVisible();
  await page.locator('.scene-controls button').nth(0).click();
  await expect(page.locator('.hero-introduction')).toBeVisible();
  for (const width of [320, 390]) {
    await page.setViewportSize({ width, height: 700 });
    await expect(page.locator('.observatory-track')).toHaveClass(/static-view/);
    for (const stage of [0, 1]) {
      await page.locator('.scene-controls button').nth(stage).click();
      const clipped = await page.locator('.observatory [data-outline]').evaluateAll(paths => paths.filter(path => {
        const box = path.getBoundingClientRect();
        return box.left < 0 || box.right > innerWidth;
      }).map(path => path.getAttribute('data-outline')));
      expect(clipped).toEqual([]);
    }
  }
});

test('reduced motion shows an assembled brain, an exploded overview and the complete handoff', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.goto(prefix + '/');
  await expect(page.locator('.reduced-brain-overview .cerebri-brain')).toHaveCount(2);
  await expect(page.locator('.observatory .cerebri-brain')).toHaveAttribute('data-progress', '1.0000');
  await expect(page.locator('.observatory-track')).toHaveClass(/static-view/);
  await expect(page.locator('.cerebri-hero').getByRole('img')).toHaveCount(1);
  await page.locator('.scene-controls button').nth(1).press('Enter');
  await expect(page.locator('.observatory')).toHaveAttribute('data-scene', '1');
  await page.getByRole('button', { name: 'Follow scroll', exact: true }).press('Enter');
  await expect(page.locator('.observatory')).toHaveAttribute('data-scene', '6');
  expect((await new AxeBuilder({ page }).include('.cerebri-hero').analyze()).violations).toEqual([]);
  expect(await page.evaluate(() => document.getAnimations().filter(a => a.playState === 'running' && a.effect?.getTiming().iterations === Infinity).length)).toBe(0);
  await page.getByRole('link', { name: 'Deterministic Ordering', exact: false }).press('Enter');
  await expect(page.locator('#candidate-comparison')).toBeInViewport();
});

for (const width of [1440, 390]) test(`@visual exploded Cerebri brain ${width}px`, async ({ page }) => {
  await page.setViewportSize({ width, height: width === 1440 ? 1000 : 844 });
  await page.goto(prefix + '/');
  await expect(page.locator('.observatory .cerebri-brain')).toHaveAttribute('data-activation', 'ready');
  await scrub(page, .4);
  const name = `brain-exploded-${width}`;
  const screenshot = await page.locator('.observatory').screenshot({ path: test.info().outputPath(name + '.png'), animations: 'disabled', style: '.site-header{visibility:hidden!important}' });
  await test.info().attach(name + '.png', { body: screenshot, contentType: 'image/png' });
  expect(createHash('sha256').update(screenshot).digest('hex')).toBe(visualBaselines[process.platform]?.[name]);
});

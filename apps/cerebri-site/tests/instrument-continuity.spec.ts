import { expect, test } from '@playwright/test';
import { runtimeData } from '../src/lib/generated/runtime-data';
import { createHash } from 'node:crypto';
import { visualBaselines } from './visual-baselines';
import AxeBuilder from '@axe-core/playwright';

const prefix = process.env.CEREBRI_BASE_PATH ?? '';

test('calibration retains endpoints, coordinates and Rust classifications when reversed', async ({ page }) => {
  await page.goto(prefix + '/');
  const hero = page.locator('.observatory');
  const endpoints = await hero.locator('.capsule-origin').elementHandles();
  const track = page.locator('.observatory-track');
  for (const phase of [2, 4, 6, 1, 0]) {
    await track.evaluate((el, target) => {
      const rect = el.getBoundingClientRect();
      window.scrollTo({ top: window.scrollY + rect.top - 160 + (target + .1) / 7 * (rect.height - 700), behavior: 'instant' });
    }, phase);
    await expect(hero).toHaveAttribute('data-scene', String(phase));
    if (phase === 6) {
      await expect.poll(async () => {
        const first = await hero.locator('.candidate-lanes li').first().boundingBox();
        const last = await hero.locator('.candidate-lanes li').last().boundingBox();
        return last!.y - first!.y;
      }).toBeGreaterThan(280);
    }
    expect(await endpoints[4].evaluate(el => el === document.querySelector('.observatory [data-candidate-id="C05"] .capsule-origin'))).toBe(true);
  }
  const read = (selector: string) => page.locator(selector).evaluateAll(nodes => nodes.map(n => [n.getAttribute('data-candidate-id'), n.getAttribute('data-start'), n.getAttribute('data-state')]));
  expect(await read('.observatory .candidate-lanes li')).toEqual(await read('.planning-instrument .candidate-lanes li'));
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await expect(hero).toHaveAttribute('data-scene', '6');
  await expect(hero.locator('.coordinate-volume')).toHaveCSS('transform', 'none');
});

test('candidate survives comparison and proposal, inspection stops at first difference', async ({ page }) => {
  await page.goto(prefix + '/');
  const plane = page.locator('.deterministic-comparator');
  const identity = await plane.locator('.winner .persistent-candidate').elementHandle();
  await plane.getByRole('button', { name: 'Candidate capsules', exact: true }).click();
  await plane.getByRole('button', { name: 'Inspect comparison', exact: true }).click();
  await expect(plane).toHaveAttribute('data-inspected', '3');
  const values = await plane.locator('.winner dd').allTextContents();
  expect(values).toEqual(['0s', String(runtimeData.plannerResult.candidates[0].ordering_key.mutation_count), '0s', '10:00 UTC', runtimeData.plannerResult.candidates[0].ordering_key.object_id]);
  await expect(plane.locator('[data-key-field="object_id"]')).toHaveClass(/later/);
  await plane.getByRole('button', { name: 'Resolve proposal', exact: true }).press('Enter');
  await expect(plane).toHaveAttribute('data-mode', '2');
  expect(await identity!.evaluate(el => el === document.querySelector('.decision-record.winner .persistent-candidate'))).toBe(true);
  await expect(plane.locator('.winner')).toContainText('C05');
  await expect(plane.locator('.winner')).toContainText('SELECTED PROPOSAL');
  await expect.poll(async () => {
    const proposal = await plane.locator('.winner .persistent-candidate').boundingBox();
    const authority = await plane.locator('.authority-plane').boundingBox();
    return authority!.x - (proposal!.x + proposal!.width);
  }).toBeGreaterThan(10);
  await plane.getByRole('button', { name: 'Inspect comparison', exact: true }).click();
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await expect(plane).toHaveAttribute('data-inspected', '3');
  await expect(plane.locator('.persistent-candidate').first()).toHaveCSS('transition-duration', '0s');
});

test('measurement moves the lattice while notation retains its physical endpoints', async ({ page }) => {
  await page.goto(prefix + '/');
  const surface = page.locator('.interval-inspection');
  const endpoint = await surface.locator('.compression-start i').elementHandle();
  await surface.getByRole('button', { name: 'Compress geometry to notation', exact: true }).click();
  await expect(surface.locator('.compression-start i')).toHaveCSS('border-radius', '0px');
  expect(await endpoint!.evaluate(el => el === document.querySelector('.compression-start i'))).toBe(true);
  const instant = page.locator('.exact-instant');
  const left = await instant.evaluate(el => getComputedStyle(el).left);
  await page.getByRole('button', { name: 'Reveal measurement lattice', exact: true }).click();
  await expect(page.locator('.measurement-lattice')).toHaveCSS('transform', 'none');
  expect(await instant.evaluate(el => getComputedStyle(el).left)).toBe(left);
  await page.locator('#math-shift').selectOption('-1');
  await expect(page.locator('.shift-trace')).toHaveCount(0);
  await expect(page.locator('.unanchored')).toBeVisible();
});

test('chapter environment follows navigation and releases its state on another route', async ({ page }) => {
  await page.goto(prefix + '/');
  await page.locator('.math-heading').scrollIntoViewIfNeeded();
  await expect(page.locator('body')).toHaveAttribute('data-instrument-world', 'measurement');
  await page.locator('.technical-inspection>summary').click();
  await page.locator('.projection-value').scrollIntoViewIfNeeded();
  await expect(page.locator('body')).toHaveAttribute('data-instrument-world', 'technical');
  await page.getByRole('link', { name: 'Explore Cerebri', exact: true }).click();
  await expect(page.locator('body')).not.toHaveAttribute('data-instrument-world');
});

test('decision modes retain accessible controls and readable semantics on mobile', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 900 });
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.goto(prefix + '/');
  for (const name of ['Candidate capsules', 'Inspect comparison', 'Resolve proposal']) {
    await page.getByRole('button', { name, exact: true }).click();
    const result = await new AxeBuilder({ page }).include('.deterministic-comparator').analyze();
    expect(result.violations).toEqual([]);
  }
});

for (const width of [320, 375, 390, 430, 768, 1280, 1440]) {
  test(`all candidate representations retain bounds at ${width}px`, async ({ page }) => {
    await page.setViewportSize({ width, height: 900 });
    await page.goto(prefix + '/');
    for (const motion of ['no-preference', 'reduce'] as const) {
      await page.emulateMedia({ reducedMotion: motion });
      for (const name of ['Candidate capsules', 'Inspect comparison', 'Resolve proposal']) {
        await page.getByRole('button', { name, exact: true }).click();
        expect(await page.evaluate(() => document.documentElement.scrollWidth - innerWidth)).toBeLessThanOrEqual(1);
      }
    }
  });
}

for (const width of [1440, 390]) {
  for (const mode of ['decision', 'proposal']) test(`@visual ${mode} plane ${width}px`, async ({ page }) => {
    await page.emulateMedia({ reducedMotion: 'reduce' });
    await page.setViewportSize({ width, height: 1000 });
    await page.goto(prefix + '/');
    await page.getByRole('button', { name: mode === 'decision' ? 'Inspect comparison' : 'Resolve proposal', exact: true }).click();
    const screenshot = await page.locator('.deterministic-comparator').screenshot({ path: test.info().outputPath(`${mode}-${width}.png`), animations: 'disabled', style: '.site-header,.coordinate-relay{visibility:hidden!important}' });
    await test.info().attach(`${mode}-${width}.png`, { body: screenshot, contentType: 'image/png' });
    expect(createHash('sha256').update(screenshot).digest('hex')).toBe(visualBaselines[process.platform]?.[`${mode}-${width}`]);
  });
  for (const phase of [0, 6]) test(`@visual calibration ${phase} at ${width}px`, async ({ page }) => {
    await page.setViewportSize({ width, height: 1100 });
    await page.goto(prefix + '/');
    await page.evaluate(() => document.fonts.ready);
    await page.locator('.observatory-track').evaluate((el, target) => {
      const rect = el.getBoundingClientRect();
      window.scrollTo({ top: scrollY + rect.top - 160 + (target + .1) / 7 * (rect.height - 700), behavior: 'instant' });
    }, phase);
    await expect(page.locator('.observatory')).toHaveAttribute('data-scene', String(phase));
    const name = `calibration-${phase}-${width}`;
    const screenshot = await page.locator('.observatory').screenshot({ path: test.info().outputPath(name + '.png'), animations: 'disabled', style: '.site-header{visibility:hidden!important}' });
    await test.info().attach(name + '.png', { body: screenshot, contentType: 'image/png' });
    expect(createHash('sha256').update(screenshot).digest('hex')).toBe(visualBaselines[process.platform]?.[name]);
  });
}

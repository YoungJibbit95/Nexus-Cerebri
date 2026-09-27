import AxeBuilder from '@axe-core/playwright';
import { expect, test, type Page } from '@playwright/test';
import { runtimeData } from '../src/lib/generated/runtime-data';
import { createHash } from 'node:crypto';
import { visualBaselines } from './visual-baselines';

const prefix = process.env.CEREBRI_BASE_PATH ?? '';
const fields = ['preference_distance_seconds', 'mutation_count', 'shifted_seconds', 'start', 'object_id'];
// Component captures exclude unrelated fixed page chrome; homepage coverage retains it.
const componentCaptureStyle = '.site-header, .skip-link { visibility: hidden !important; }';

async function settleMeasurementCapture(page: Page) {
  await page.evaluate(() => document.fonts.ready);
  await expect(page.locator('.math-inspection')).toHaveAttribute('data-motion', 'reduced');
  // Clicks scroll into this chapter before its IntersectionObserver updates the
  // fixed environment. Capture the settled chapter, not the previous background.
  await expect(page.locator('body')).toHaveAttribute('data-instrument-world', 'measurement');
  await page.evaluate(() => new Promise<void>(resolve =>
    requestAnimationFrame(() => requestAnimationFrame(() => resolve()))
  ));
}

test.beforeEach(async ({ page }) => {
  await page.goto(prefix + '/');
});

test('paired intervals preserve half-open boundaries and the exact intersection', async ({ page }) => {
  const surface = page.locator('.interval-inspection');
  await expect(surface).toContainText('These appointments fit back to back.');
  await expect(surface).toContainText('[start, end)');
  await expect(surface).toContainText('Each appointment includes its start, but no longer occupies time at its end.');
  await expect(surface.locator('.intersection-band')).toHaveCount(0);
  await surface.getByRole('button', { name: 'One-second overlap', exact: true }).click();
  await expect(surface).toContainText('Both occupy 10:00:00–10:00:01 UTC');
  await expect(surface.locator('.intersection-band')).toBeVisible();
  await surface.getByText('Show the overlap check', { exact: true }).click();
  await expect(surface).toContainText('Do these intervals overlap? true');
  await expect(surface).toContainText('NoSolution');
});

test('preference inspection uses a separate fixture without changing homepage evidence', async ({ page }) => {
  const surface = page.locator('.preferred-inspection');
  const expected = runtimeData.mathInspection.preferred;
  await expect(surface.locator('.source-link')).toHaveAttribute('href', /02-ranking-preferred-start.json$/);
  expect(await surface.locator('option').allTextContents()).toEqual(expected.candidates.map(c => c.start.slice(11, 19) + ' UTC'));
  await page.locator('#math-candidate').selectOption('0');
  await expect(surface).toContainText('A start at 10:45:00 is 0 seconds from the preferred 10:45:00 UTC');
  await page.locator('#math-candidate').selectOption('1');
  await expect(surface).toContainText('900 seconds');
  expect(runtimeData.request.preferences.preferences).toEqual([]);
  expect(runtimeData.plannerResult.candidates[0].ranking_features.preferred_start_distance_seconds).toBeNull();
  await expect(page.locator('.planning-instrument')).toContainText('NO PREFERRED TIME SUPPLIED');
});

test('measurement preserves exact instants and explicitly distinguishes absent placement', async ({ page }) => {
  const surface = page.locator('.quantization-inspection');
  await expect(surface).toContainText('Actual time difference: 0.8 s. Distance used for ranking: 0 s, recorded in whole seconds');
  await surface.getByText('Inspect exact instants and measurement', { exact: true }).click();
  await expect(surface).toContainText('1970-01-01T00:00:00.800Z');
  await surface.getByRole('button', { name: '-1 s', exact: true }).click();
  await expect(surface).toContainText('Distance used for ranking: 1 s, recorded in whole seconds');
  await page.locator('#math-shift').selectOption('-1');
  await expect(page.locator('.shift-inspection')).toHaveAttribute('data-original', 'absent');
  await expect(page.locator('.ghost-placement')).toHaveCount(0);
  await expect(page.locator('.shift-inspection')).toContainText('that does not mean an existing appointment stayed in place');
  await page.locator('#math-shift').selectOption('1');
  await expect(page.locator('.ghost-placement')).toBeVisible();
  await expect(page.locator('.shift-inspection')).toContainText('The start differs by 0.8 seconds');
  await expect(page.locator('.shift-reading>strong')).toHaveText('0 s');
});

test('technical inspection preserves missingness, exact tuple fields and Rust order', async ({ page }) => {
  await page.locator('.technical-inspection>summary').click();
  await expect(page.locator('[data-observation="absent"]')).toContainText('None');
  await expect(page.locator('[data-observation="zero"]')).toContainText('Some(0)');
  for (let index = 0; index < 2; index++) {
    const tuple = page.locator('.ordering-tuple').nth(index);
    expect(await tuple.locator('li').evaluateAll(nodes => nodes.map(n => n.getAttribute('data-key-field')))).toEqual(fields);
    expect(await tuple.locator('strong').allTextContents()).toEqual(Object.values(runtimeData.plannerResult.candidates[index].ordering_key).map(String));
  }
});

test('keyboard inspection and reversible normal-motion transformations preserve state across fast scrolling', async ({ page }) => {
  await page.emulateMedia({ reducedMotion: 'no-preference' });
  const surface = page.locator('.math-inspection');
  await expect(surface).toHaveAttribute('data-motion', 'full');
  const compress = surface.getByRole('button', { name: 'Show the interval notation', exact: true });
  await compress.focus();
  await page.keyboard.press('Enter');
  await expect(page.locator('.interval-inspection')).toHaveAttribute('data-compressed', 'true');
  await page.evaluate(() => window.scrollTo({ top: document.body.scrollHeight, behavior: 'instant' }));
  await page.evaluate(() => window.scrollTo({ top: 0, behavior: 'instant' }));
  await expect(page.locator('.interval-inspection')).toHaveAttribute('data-compressed', 'true');
  await surface.getByRole('button', { name: 'Show the timeline', exact: true }).focus();
  await page.keyboard.press('Space');
  await expect(page.locator('.interval-inspection')).toHaveAttribute('data-compressed', 'false');
  const technical = page.locator('.technical-inspection>summary');
  await technical.focus();
  await page.keyboard.press('Enter');
  const shiftTerm = page.locator('.tuple-dimensions button').nth(2);
  await shiftTerm.focus();
  await page.keyboard.press('Enter');
  await expect(page.locator('.dimension-reading')).toContainText('shifted_seconds carries the observation shift_seconds');
});

test('reduced motion shows measurement and correspondence directly and survives preference changes', async ({ page }) => {
  await page.emulateMedia({ reducedMotion: 'reduce' });
  const surface = page.locator('.math-inspection');
  await expect(surface).toHaveAttribute('data-motion', 'reduced');
  await expect(surface.locator('.notation-equivalent')).toBeVisible();
  await expect(surface.locator('.measurement-lattice')).toHaveCSS('transform', 'none');
  await expect(surface.locator('.measurement-lattice')).toHaveCSS('opacity', '0.55');
  await expect(surface.locator('.ghost-placement')).toBeVisible();
  await expect(surface.locator('.shift-candidate')).toBeVisible();
  await expect(surface.locator('.shift-candidate')).toHaveCSS('transition-duration', '0s');
  await page.emulateMedia({ reducedMotion: 'no-preference' });
  await expect(surface).toHaveAttribute('data-motion', 'full');
});

for (const width of [320, 375, 390, 430, 768, 1280, 1440]) {
  test(`mathematical inspection retains its complete content at ${width}px`, async ({ page }) => {
    await page.setViewportSize({ width, height: 900 });
    await page.locator('.technical-inspection>summary').click();
    await page.getByRole('button', { name: 'Compress dimensions into tuples', exact: true }).click();
    const surface = page.locator('.math-inspection');
    for (const selector of ['.interval-world', '.distance-world', '.measurement-world', '.shift-world', '.observation-projection', '.tuple-inspection']) {
      await expect(surface.locator(selector)).toBeVisible();
    }
    const overflow = await surface.evaluate(root => [...root.querySelectorAll('*')].filter(el => {
      const r = el.getBoundingClientRect();
      return r.width > 0 && (r.right > window.innerWidth + 1 || r.left < -1);
    }).map(el => el.className));
    expect(overflow).toEqual([]);
  });
}

test('inspection is accessible with all disclosures open and enlarged text', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 900 });
  await page.locator('.math-inspection details').evaluateAll(nodes => nodes.forEach(node => node.setAttribute('open', '')));
  await page.evaluate(() => document.documentElement.style.fontSize = '200%');
  const results = await new AxeBuilder({ page }).include('.math-inspection').analyze();
  expect(results.violations).toEqual([]);
  const overflow = await page.locator('.math-inspection').evaluate(el => el.scrollWidth - el.clientWidth);
  expect(overflow).toBeLessThanOrEqual(1);
});

for (const width of [1440, 768, 390]) {
  test(`@visual mathematical inspection ${width}px`, async ({ page }) => {
    await page.emulateMedia({ reducedMotion: 'reduce' });
    await page.setViewportSize({ width, height: 1000 });
    await page.locator('.interval-inspection').getByRole('button', { name: 'One-second overlap', exact: true }).click();
    await settleMeasurementCapture(page);
    const surface = page.locator('.math-inspection');
    const screenshot = await surface.screenshot({ path: test.info().outputPath(`math-${width}.png`), animations: 'disabled', style: componentCaptureStyle });
    const hash = createHash('sha256').update(screenshot).digest('hex');
    await test.info().attach(`math-${width}.png`, { body: screenshot, contentType: 'image/png' });
    expect(hash, `reviewed math baseline for ${process.platform} at ${width}px`).toBe(visualBaselines[process.platform]?.[`math-${width}`]);
  });

  test(`@visual technical inspection ${width}px`, async ({ page }) => {
    await page.emulateMedia({ reducedMotion: 'reduce' });
    await page.setViewportSize({ width, height: 1000 });
    await page.locator('.technical-inspection>summary').click();
    await page.getByRole('button', { name: 'Compress dimensions into tuples', exact: true }).click();
    const screenshot = await page.locator('.technical-inspection').screenshot({ path: test.info().outputPath(`technical-${width}.png`), animations: 'disabled', style: componentCaptureStyle });
    const hash = createHash('sha256').update(screenshot).digest('hex');
    await test.info().attach(`technical-${width}.png`, { body: screenshot, contentType: 'image/png' });
    expect(hash, `reviewed technical baseline for ${process.platform} at ${width}px`).toBe(visualBaselines[process.platform]?.[`technical-${width}`]);
  });
}

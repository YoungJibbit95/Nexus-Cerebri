# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: apps\cerebri-site\tests\math-inspection.spec.ts >> reduced motion shows measurement and correspondence directly and survives preference changes
- Location: apps\cerebri-site\tests\math-inspection.spec.ts:95:1

# Error details

```
Error: page.goto: Protocol error (Page.navigate): Cannot navigate to invalid URL
Call log:
  - navigating to "/Nexus-Cerebri/", waiting until "load"

```

# Test source

```ts
  1   | import AxeBuilder from '@axe-core/playwright';
  2   | import { expect, test } from '@playwright/test';
  3   | import { runtimeData } from '../src/lib/generated/runtime-data';
  4   | import { createHash } from 'node:crypto';
  5   | import { visualBaselines } from './visual-baselines';
  6   | 
  7   | const prefix = process.env.CEREBRI_BASE_PATH ?? '';
  8   | const fields = ['preference_distance_seconds', 'mutation_count', 'shifted_seconds', 'start', 'object_id'];
  9   | // Component captures exclude unrelated fixed page chrome; homepage coverage retains it.
  10  | const componentCaptureStyle = '.site-header, .skip-link { visibility: hidden !important; }';
  11  | 
  12  | test.beforeEach(async ({ page }) => {
> 13  |   await page.goto(prefix + '/');
      |              ^ Error: page.goto: Protocol error (Page.navigate): Cannot navigate to invalid URL
  14  | });
  15  | 
  16  | test('paired intervals preserve half-open boundaries and the exact intersection', async ({ page }) => {
  17  |   const surface = page.locator('.interval-inspection');
  18  |   await expect(surface).toContainText('One shared endpoint. No overlap.');
  19  |   await expect(surface).toContainText('[start, end)');
  20  |   await expect(surface).toContainText('ends at 10:00:00 exclusive');
  21  |   await expect(surface.locator('.intersection-band')).toHaveCount(0);
  22  |   await surface.getByRole('button', { name: 'One-second overlap', exact: true }).click();
  23  |   await expect(surface).toContainText('Intervals overlap from 10:00:00 to 10:00:01 UTC');
  24  |   await expect(surface.locator('.intersection-band')).toBeVisible();
  25  |   await surface.getByText('Inspect the interval relation', { exact: true }).click();
  26  |   await expect(surface).toContainText('Rust overlap result: true');
  27  |   await expect(surface).toContainText('NoSolution');
  28  | });
  29  | 
  30  | test('preference inspection uses a separate fixture without changing homepage evidence', async ({ page }) => {
  31  |   const surface = page.locator('.preferred-inspection');
  32  |   const expected = runtimeData.mathInspection.preferred;
  33  |   await expect(surface.locator('.source-link')).toHaveAttribute('href', /02-ranking-preferred-start.json$/);
  34  |   expect(await surface.locator('option').allTextContents()).toEqual(expected.candidates.map(c => c.start.slice(11, 19) + ' UTC'));
  35  |   await page.locator('#math-candidate').selectOption('0');
  36  |   await expect(surface).toContainText('Candidate 10:45:00 is 0 seconds from preferred start 10:45:00 UTC');
  37  |   await page.locator('#math-candidate').selectOption('1');
  38  |   await expect(surface).toContainText('900 seconds');
  39  |   expect(runtimeData.request.preferences.preferences).toEqual([]);
  40  |   expect(runtimeData.plannerResult.candidates[0].ranking_features.preferred_start_distance_seconds).toBeNull();
  41  |   await expect(page.locator('.planning-instrument')).toContainText('NONE');
  42  | });
  43  | 
  44  | test('measurement preserves exact instants and explicitly distinguishes absent placement', async ({ page }) => {
  45  |   const surface = page.locator('.quantization-inspection');
  46  |   await expect(surface).toContainText('Actual signed difference: 0.8 seconds; whole-second ranking observation: 0 seconds');
  47  |   await surface.getByText('Inspect exact instants and measurement', { exact: true }).click();
  48  |   await expect(surface).toContainText('1970-01-01T00:00:00.800Z');
  49  |   await surface.getByRole('button', { name: '-1 s', exact: true }).click();
  50  |   await expect(surface).toContainText('whole-second ranking observation: 1 seconds');
  51  |   await page.locator('#math-shift').selectOption('-1');
  52  |   await expect(page.locator('.shift-inspection')).toHaveAttribute('data-original', 'absent');
  53  |   await expect(page.locator('.ghost-placement')).toHaveCount(0);
  54  |   await expect(page.locator('.shift-inspection')).toContainText('it does not establish an unchanged placement');
  55  |   await page.locator('#math-shift').selectOption('1');
  56  |   await expect(page.locator('.ghost-placement')).toBeVisible();
  57  |   await expect(page.locator('.shift-inspection')).toContainText('Actual displacement: 0.8 seconds');
  58  |   await expect(page.locator('.shift-reading>strong')).toHaveText('0 s');
  59  | });
  60  | 
  61  | test('technical inspection preserves missingness, exact tuple fields and Rust order', async ({ page }) => {
  62  |   await page.locator('.technical-inspection>summary').click();
  63  |   await expect(page.locator('[data-observation="absent"]')).toContainText('None');
  64  |   await expect(page.locator('[data-observation="zero"]')).toContainText('Some(0)');
  65  |   for (let index = 0; index < 2; index++) {
  66  |     const tuple = page.locator('.ordering-tuple').nth(index);
  67  |     expect(await tuple.locator('li').evaluateAll(nodes => nodes.map(n => n.getAttribute('data-key-field')))).toEqual(fields);
  68  |     expect(await tuple.locator('strong').allTextContents()).toEqual(Object.values(runtimeData.plannerResult.candidates[index].ordering_key).map(String));
  69  |   }
  70  | });
  71  | 
  72  | test('keyboard inspection and reversible normal-motion transformations preserve state across fast scrolling', async ({ page }) => {
  73  |   await page.emulateMedia({ reducedMotion: 'no-preference' });
  74  |   const surface = page.locator('.math-inspection');
  75  |   await expect(surface).toHaveAttribute('data-motion', 'full');
  76  |   const compress = surface.getByRole('button', { name: 'Compress geometry to notation', exact: true });
  77  |   await compress.focus();
  78  |   await page.keyboard.press('Enter');
  79  |   await expect(page.locator('.interval-inspection')).toHaveAttribute('data-compressed', 'true');
  80  |   await page.evaluate(() => window.scrollTo({ top: document.body.scrollHeight, behavior: 'instant' }));
  81  |   await page.evaluate(() => window.scrollTo({ top: 0, behavior: 'instant' }));
  82  |   await expect(page.locator('.interval-inspection')).toHaveAttribute('data-compressed', 'true');
  83  |   await surface.getByRole('button', { name: 'Expand interval geometry', exact: true }).focus();
  84  |   await page.keyboard.press('Space');
  85  |   await expect(page.locator('.interval-inspection')).toHaveAttribute('data-compressed', 'false');
  86  |   const technical = page.locator('.technical-inspection>summary');
  87  |   await technical.focus();
  88  |   await page.keyboard.press('Enter');
  89  |   const shiftTerm = page.locator('.tuple-dimensions button').nth(2);
  90  |   await shiftTerm.focus();
  91  |   await page.keyboard.press('Enter');
  92  |   await expect(page.locator('.dimension-reading')).toContainText('shifted_seconds carries the observation shift_seconds');
  93  | });
  94  | 
  95  | test('reduced motion shows measurement and correspondence directly and survives preference changes', async ({ page }) => {
  96  |   await page.emulateMedia({ reducedMotion: 'reduce' });
  97  |   const surface = page.locator('.math-inspection');
  98  |   await expect(surface).toHaveAttribute('data-motion', 'reduced');
  99  |   await expect(surface.locator('.notation-equivalent')).toBeVisible();
  100 |   await expect(surface.locator('.measurement-lattice')).toHaveCSS('transform', 'none');
  101 |   await expect(surface.locator('.measurement-lattice')).toHaveCSS('opacity', '0.55');
  102 |   await expect(surface.locator('.ghost-placement')).toBeVisible();
  103 |   await expect(surface.locator('.shift-candidate')).toBeVisible();
  104 |   await expect(surface.locator('.shift-candidate')).toHaveCSS('transition-duration', '0s');
  105 |   await page.emulateMedia({ reducedMotion: 'no-preference' });
  106 |   await expect(surface).toHaveAttribute('data-motion', 'full');
  107 | });
  108 | 
  109 | for (const width of [320, 375, 390, 430, 768, 1280, 1440]) {
  110 |   test(`mathematical inspection retains its complete content at ${width}px`, async ({ page }) => {
  111 |     await page.setViewportSize({ width, height: 900 });
  112 |     await page.locator('.technical-inspection>summary').click();
  113 |     await page.getByRole('button', { name: 'Compress dimensions into tuples', exact: true }).click();
```
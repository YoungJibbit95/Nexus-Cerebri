import AxeBuilder from '@axe-core/playwright';
import { expect, test } from '@playwright/test';
import { createHash } from 'node:crypto';
import { visualBaselines } from './visual-baselines';

const prefix = process.env.CEREBRI_BASE_PATH ?? '';
const routes = ['/', '/explore/', '/cpir/', '/planning/', '/time/', '/safety/', '/architecture/', '/lab/', '/roadmap/', '/developers/', '/docs/'];

test('all primary knowledge routes prerender and expose a heading', async ({ page }) => {
  for (const route of routes) {
    const response = await page.goto(prefix + route);
    expect(response?.ok(), route).toBeTruthy();
    await expect(page.locator('h1').first()).toBeVisible();
  }
});

test('explanation depth changes visible information and persists', async ({ page }) => {
  await page.goto(prefix + '/architecture/');
  const technical = page.locator('.depth-technical');
  const research = page.locator('.depth-research');
  await expect(technical).toBeHidden();
  await expect(research).toBeHidden();
  await page.getByRole('button', { name: /Technical/ }).click();
  await expect(technical).toBeVisible();
  await expect(research).toBeHidden();
  await page.reload();
  await expect(technical).toBeVisible();
  await page.getByRole('button', { name: /Research/ }).click();
  await expect(research).toBeVisible();
});

test('homepage reflects the current structured deterministic planning boundary', async ({ page }) => {
  await page.goto(prefix + '/');
  const heroHeading = page.locator('h1').first();
  await expect(heroHeading).toContainText('Bound the problem.');
  await expect(heroHeading).toContainText('Verify the plan.');
  await expect(page.getByText('Current CPIR planning fixture', { exact: true })).toBeVisible();
  await expect(page.getByText('11 evaluated positions', { exact: true }).first()).toBeVisible();
  await expect(page.getByText('7 valid / 4 rejected', { exact: true })).toBeVisible();
  await expect(page.getByText('Structured evidence. Bounded search. Explicit authority.', { exact: true })).toBeVisible();
  await expect(page.getByText(/It can explore meaning and ambiguity/i)).toHaveCount(0);
  await expect(page.getByText(/HUMAN \/ PROBABILISTIC/i)).toHaveCount(0);
});

test('semantic planning visuals preserve candidate and authority distinctions', async ({ page }) => {
  await page.goto(prefix + '/');

  const observatory = page.locator('.observatory');
  await expect(observatory).toBeVisible();
  await expect(observatory).toHaveAttribute('aria-label', /Bounded search observatory/);

  const candidates = page.locator('ol[aria-label="Evaluated candidate intervals"] > li');
  await expect(candidates).toHaveCount(11);
  await expect(page.locator('.candidate-lanes li[data-state="rejected"]')).toHaveCount(4);
  await expect(page.locator('.candidate-lanes li[data-state="valid"]')).toHaveCount(6);
  await expect(page.locator('.candidate-lanes li[data-state="selected"]')).toHaveCount(1);

  const authorityBoundary = page.locator('.proposal-boundary');
  await expect(authorityBoundary).toHaveAttribute('aria-label', /Selected proposal at 10:00 UTC stops before a separate authority gate/);
  await expect(authorityBoundary.getByText('AUTHORITY GATE', { exact: true })).toBeVisible();
  await expect(authorityBoundary.getByText('EXECUTION', { exact: true })).toBeVisible();

  const instrument = page.locator('.planning-instrument');
  await expect(instrument.locator('[aria-label="Planning visual grammar"]')).toBeVisible();
  await expect(instrument.locator('[data-semantic-kind="scope"]')).toHaveCount(1);
  await expect(instrument.locator('[data-semantic-kind="fact"]')).toHaveCount(1);
  await expect(instrument.locator('[data-semantic-kind="constraint"]')).toHaveCount(1);
  await expect(instrument.locator('[data-semantic-kind="preference"]')).toHaveCount(1);

  const semanticFlow = instrument.getByRole('region', { name: /Candidate to authority flow/i });
  await expect(semanticFlow).toBeVisible();
  await expect(semanticFlow.locator('[data-flow-node]')).toHaveCount(4);
  await expect(semanticFlow.locator('[data-flow-node="candidates"]')).toContainText('7 valid after hard-rule checks');
  await expect(semanticFlow.locator('[data-flow-node="comparator"]')).toContainText('Rust-produced order');
  await expect(semanticFlow.locator('[data-flow-node="proposal"]')).toContainText('10:00 UTC remains a proposal');
  await expect(semanticFlow.locator('[data-flow-node="authority"]')).toContainText('authorization stay separate');

  const comparator = instrument.getByRole('region', { name: /Deterministic comparator/i });
  await expect(comparator).toBeVisible();
  await expect(comparator.locator('.comparator-row.decisive')).toHaveAttribute('data-key-field', 'start');
  await expect(comparator.locator('.comparator-resolution')).toContainText('First differing key:');

  const lifecycle = instrument.getByRole('region', { name: /Planner authority boundary/i });
  await expect(lifecycle).toBeVisible();
  await expect(lifecycle.locator('[data-authority-stage="proposal"]')).toHaveClass(/current/);
  await expect(lifecycle.locator('[data-authority-stage="authorized"]')).not.toHaveClass(/reached/);
});

test('reduced motion resolves the hero directly to an equivalent semantic state', async ({ page }) => {
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.goto(prefix + '/');
  const observatory = page.locator('.observatory');
  await expect(observatory).toHaveAttribute('data-motion', 'reduced');
  await expect(observatory).toHaveClass(/proposed/);

  const instrument = page.locator('.planning-instrument');
  await expect(instrument).toHaveAttribute('data-motion', 'reduced');
  await expect(instrument).toHaveAttribute('data-flow-stage', '3');
  await expect(instrument.locator('[data-flow-node="authority"]')).toHaveClass(/current/);
});

test('structured evidence inspection is keyboard operable', async ({ page }) => {
  await page.goto(prefix + '/');
  const inspect = page.locator('.inspect-button');
  await inspect.focus();
  await expect(inspect).toBeFocused();
  await page.keyboard.press('Enter');
  await expect(inspect).toHaveAttribute('aria-expanded', 'true');
  await expect(page.getByText('SCOPE / CANONICAL INPUT', { exact: true })).toBeVisible();
});

test('homepage and planning surface meet the serious axe floor', async ({ page }) => {
  for (const route of ['/', '/planning/']) {
    await page.goto(prefix + route);
    if (route === '/') {
      const releaseStatus = page.getByRole('group', { name: 'Release status' });
      await expect(releaseStatus.getByText('Published release: None', { exact: true })).toBeVisible();
      await expect(releaseStatus.getByText('Unreleased · qualification in progress', { exact: true })).toBeVisible();
      await expect(releaseStatus.getByText('CPIR 0.2', { exact: true })).toBeVisible();
    }
    const results = await new AxeBuilder({ page }).analyze();
    const blocking = results.violations.filter((violation) => violation.impact === 'serious' || violation.impact === 'critical');
    expect(blocking, JSON.stringify(blocking, null, 2)).toEqual([]);
  }
});

for (const width of [1440, 1280, 1024, 768, 430, 390, 375, 320]) {
  test('responsive layout has no horizontal overflow at ' + width + 'px', async ({ page }) => {
    await page.setViewportSize({ width, height: 900 });
    await page.goto(prefix + '/');
    const overflow = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
    expect(overflow).toBeLessThanOrEqual(1);
  });
}

test('homepage remains overflow-safe with enlarged root text', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 900 });
  await page.goto(prefix + '/');
  await page.evaluate(() => document.documentElement.style.fontSize = '200%');
  const overflow = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
  expect(overflow).toBeLessThanOrEqual(1);
});

for (const width of [1440, 1024, 768, 390]) {
  test('@visual homepage ' + width + 'px', async ({ page }) => {
    await page.emulateMedia({ reducedMotion: 'reduce' });
    await page.setViewportSize({ width, height: 1000 });
    await page.goto(prefix + '/');
    const screenshot = await page.screenshot({ path: test.info().outputPath('homepage-' + width + '.png'), fullPage: true, animations: 'disabled' });
    const hash = createHash('sha256').update(screenshot).digest('hex');
    const expected = visualBaselines[process.platform]?.[String(width)];
    expect(expected, 'reviewed visual baseline for ' + process.platform + ' at ' + width + 'px').toBeDefined();
    if (hash !== expected) {
      await test.info().attach('homepage-' + width + '.png', { body: screenshot, contentType: 'image/png' });
    }
    expect(hash, 'visual hash for ' + width + 'px').toBe(expected);
  });
}

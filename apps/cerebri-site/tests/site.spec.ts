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

for (const width of [1440, 1024, 768, 390]) {
  test('responsive layout has no horizontal overflow at ' + width + 'px', async ({ page }) => {
    await page.setViewportSize({ width, height: 900 });
    await page.goto(prefix + '/');
    const overflow = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
    expect(overflow).toBeLessThanOrEqual(1);
  });

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


test('semantic visual grammar keeps candidates, constraints, preferences and authority distinct', async ({ page }) => {
  await page.goto(prefix + '/');
  const workbench = page.getByRole('region', { name: /Planning workbench/i });
  await expect(workbench).toBeVisible();
  await expect(workbench.locator('[data-semantic-kind="fact"]')).toHaveCount(1);
  await expect(workbench.locator('[data-semantic-kind="constraint"]')).toHaveCount(1);
  await expect(workbench.locator('[data-semantic-kind="preference"]')).toHaveCount(1);
  await expect(workbench.locator('[data-candidate-state="rejected"]')).toHaveCount(4);
  await expect(workbench.locator('[data-candidate-state="valid"]')).toHaveCount(6);
  await expect(workbench.locator('[data-candidate-state="selected"]')).toHaveCount(1);
  await expect(workbench.locator('.comparator-row.decisive')).toContainText('start');
  await expect(workbench.getByText('7 valid / 4 rejected', { exact: true })).toBeVisible();

  const proposal = workbench.locator('[data-authority-stage="proposal"]');
  const authorized = workbench.locator('[data-authority-stage="authorized"]');
  await expect(proposal).toHaveClass(/current/);
  await expect(authorized).not.toHaveClass(/reached/);

  await page.goto(prefix + '/safety/');
  await expect(page.locator('.authority-path [data-authority-stage="proposal"]')).toBeVisible();
  await expect(page.locator('.authority-path [data-authority-stage="authorized"]')).toBeVisible();
});

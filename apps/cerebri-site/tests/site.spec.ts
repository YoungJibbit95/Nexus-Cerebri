import AxeBuilder from '@axe-core/playwright';
import { expect, test } from '@playwright/test';

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

test('homepage and planning surface meet the serious axe floor', async ({ page }) => {
  for (const route of ['/', '/planning/']) {
    await page.goto(prefix + route);
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
    await page.setViewportSize({ width, height: 1000 });
    await page.goto(prefix + '/');
    await expect(page).toHaveScreenshot('home-' + width + '.png', { fullPage: true, animations: 'disabled' });
  });
}

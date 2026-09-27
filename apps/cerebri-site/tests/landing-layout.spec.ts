import { expect, test } from '@playwright/test';
import { createHash } from 'node:crypto';
import { visualBaselines } from './visual-baselines';

const prefix = process.env.CEREBRI_BASE_PATH ?? '';

for (const width of [1920, 1440, 768, 390, 320]) {
  test(`arrival keeps prose, stages and the instrument separate at ${width}px`, async ({ page }) => {
    await page.setViewportSize({ width, height: 900 });
    await page.emulateMedia({ reducedMotion: 'reduce' });
    await page.goto(prefix + '/');
    const heading = await page.locator('h1').boundingBox();
    const header = await page.locator('.site-header').boundingBox();
    expect(heading!.y).toBeGreaterThan(header!.y + header!.height);
    for (const depth of ['Understand', 'Technical', 'Research']) {
      await page.getByRole('button', { name: new RegExp(depth) }).click();
      for (const textSize of ['100%', '200%']) {
        await page.evaluate(size => document.documentElement.style.fontSize = size, textSize);
        for (const phase of [0, 3, 6]) {
          await page.locator('.scene-controls button').nth(phase).click();
          for (const anchor of ['.hero-copy', '.arrival-instrument', '.scene-controls']) {
            await page.locator(anchor).scrollIntoViewIfNeeded();
            const collisions = await page.locator('.cerebri-hero').evaluate(el => {
              const selectors = ['.hero-copy', '.scene-explanation', '.arrival-instrument', '.scene-controls'];
              return selectors.flatMap((selector, index) => {
                const a = el.querySelector(selector)!.getBoundingClientRect();
                return selectors.slice(index + 1).filter(other => {
                  const b = el.querySelector(other)!.getBoundingClientRect();
                  return a.left < b.right - 1 && a.right > b.left + 1 && a.top < b.bottom - 1 && a.bottom > b.top + 1;
                }).map(other => [selector, other]);
              });
            });
            expect(collisions).toEqual([]);
          }
          expect(await page.evaluate(() => document.documentElement.scrollWidth - innerWidth)).toBeLessThanOrEqual(1);
        }
      }
    }
  });
}

for (const width of [1440, 390]) {
  test(`knowledge cards grow around enlarged descriptions at ${width}px`, async ({ page }) => {
    await page.setViewportSize({ width, height: 900 });
    await page.goto(prefix + '/');
    await page.evaluate(() => document.documentElement.style.fontSize = '200%');
    const collisions = await page.locator('.journey-card,.feature-card').evaluateAll(cards => cards.filter(card => {
      const description = card.querySelector('p')!.getBoundingClientRect();
      const link = card.querySelector('.journey-card > strong,.feature-card > a')!.getBoundingClientRect();
      return description.bottom > link.top + 1 || link.bottom > card.getBoundingClientRect().bottom + 1;
    }).map(card => card.querySelector('h3')!.textContent));
    expect(collisions).toEqual([]);
    const unusedAssessmentHeight = await page.locator('.intent-boundary-note').evaluate(node =>
      node.getBoundingClientRect().bottom - node.querySelector('p')!.getBoundingClientRect().bottom - Number.parseFloat(getComputedStyle(node).paddingBottom));
    expect(unusedAssessmentHeight).toBeLessThanOrEqual(1);
  });
}

for (const viewport of [{ width: 1440, height: 600 }, { width: 390, height: 700 }]) {
  test(`short viewport ${viewport.width}px keeps all stages available without pinning`, async ({ page }) => {
    await page.setViewportSize(viewport);
    await page.goto(prefix + '/');
    const track = page.locator('.observatory-track');
    await expect(track).toHaveClass(/static-view/);
    await expect(track.locator('.observatory')).toHaveAttribute('data-scene', '0');
    await track.locator('.scene-controls button').nth(6).click();
    await expect(track.locator('.observatory')).toHaveAttribute('data-scene', '6');
    expect(await track.evaluate(el => el.getBoundingClientRect().height - el.firstElementChild!.getBoundingClientRect().height)).toBeLessThanOrEqual(1);
    await page.setViewportSize({ width: 1440, height: 1200 });
    await expect(track).not.toHaveClass(/static-view/);
    await page.setViewportSize(viewport);
    await expect(track).toHaveClass(/static-view/);
    await expect(track.locator('[data-candidate-id]')).toHaveCount(11);
  });
}

for (const width of [1440, 768, 390, 320]) {
  test(`candidate captions and comparison remain legible with enlarged text at ${width}px`, async ({ page }) => {
    await page.setViewportSize({ width, height: 900 });
    await page.emulateMedia({ reducedMotion: 'reduce' });
    await page.goto(prefix + '/');
    const authorityCaption = page.locator('.authority-plane > span');
    expect(await authorityCaption.evaluate(node => Number.parseFloat(getComputedStyle(node).height) - Number.parseFloat(getComputedStyle(node).lineHeight))).toBeLessThanOrEqual(1);
    await page.evaluate(() => document.documentElement.style.fontSize = '200%');
    const fieldCollisions = await page.locator('.planning-instrument .candidate-coordinate').evaluate(root => {
      const labels = [...root.querySelectorAll<HTMLElement>('.candidate-identity,.candidate-coordinate-label,.candidate-state,.coordinate-axis small,.coordinate-busy span')].filter(node => node.getClientRects().length && getComputedStyle(node).opacity !== '0');
      return labels.flatMap((node, index) => {
        const a = node.getBoundingClientRect();
        return labels.slice(index + 1).filter(other => {
          const b = other.getBoundingClientRect();
          return a.left < b.right && a.right > b.left && a.top < b.bottom && a.bottom > b.top;
        }).map(other => [node.textContent, other.textContent]);
      });
    });
    expect(fieldCollisions).toEqual([]);
    const tickCollisions = await page.locator('.zoom-labels span').evaluateAll(nodes => {
      const boxes = nodes.map(node => node.getBoundingClientRect());
      return boxes.flatMap((box, index) => boxes.slice(index + 1).filter(other =>
        box.left < other.right && box.right > other.left && box.top < other.bottom && box.bottom > other.top));
    });
    expect(tickCollisions).toEqual([]);
    for (const name of ['Candidate capsules', 'Inspect comparison', 'Resolve proposal']) {
      await page.getByRole('button', { name, exact: true }).click();
      const problems = await page.locator('.deterministic-comparator').evaluate(root => {
        const problems: string[] = [];
        const surface = root.querySelector('.decision-surface')!.getBoundingClientRect();
        for (const node of root.querySelectorAll<HTMLElement>('.persistent-candidate b,.representation-label,.record-dimensions dt,.record-dimensions dd')) {
          if (node.closest('.record-dimensions') && root.getAttribute('data-mode') !== '1') continue;
          if (getComputedStyle(node).clipPath !== 'none') continue;
          const rect = node.getBoundingClientRect();
          if (node.scrollWidth > node.clientWidth + 1 || rect.right > surface.right + 1 || rect.bottom > surface.bottom + 1) problems.push(node.textContent ?? 'text');
        }
        return problems;
      });
      expect(problems, name).toEqual([]);
    }
    await page.locator('.planning-instrument').screenshot({ path: test.info().outputPath(`instrument-enlarged-${width}.png`), animations: 'disabled', style: '.site-header{visibility:hidden!important}' });
  });
}

for (const width of [1440, 390]) {
  for (const section of ['rejection', 'workbench']) test(`@visual ${section} ${width}px`, async ({ page }) => {
    await page.setViewportSize({ width, height: 1000 });
    await page.emulateMedia({ reducedMotion: 'reduce' });
    await page.goto(prefix + '/');
    await page.evaluate(() => document.fonts.ready);
    const name = `${section}-${width}`;
    const selector = section === 'rejection' ? '.rejection-witness' : '.intent-field';
    await page.locator(selector).scrollIntoViewIfNeeded();
    // The native scroll updates the chapter backdrop through an IntersectionObserver.
    await expect(page.locator('body')).toHaveAttribute('data-instrument-world', 'planning');
    const screenshot = await page.locator(selector).screenshot({ path: test.info().outputPath(name + '.png'), animations: 'disabled', style: '.site-header,.skip-link{visibility:hidden!important}' });
    await test.info().attach(name + '.png', { body: screenshot, contentType: 'image/png' });
    expect(createHash('sha256').update(screenshot).digest('hex')).toBe(visualBaselines[process.platform]?.[name]);
  });
}

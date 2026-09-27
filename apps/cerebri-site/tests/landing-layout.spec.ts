import { expect, test } from '@playwright/test';
import { createHash } from 'node:crypto';
import { visualBaselines } from './visual-baselines';

const prefix = process.env.CEREBRI_BASE_PATH ?? '';

for (const width of [1920, 1440, 768, 390, 320]) {
  test(`explanation keeps prose and annotations in separate rows at ${width}px`, async ({ page }) => {
    await page.setViewportSize({ width, height: 900 });
    await page.emulateMedia({ reducedMotion: 'reduce' });
    await page.goto(prefix + '/');
    for (const depth of ['Understand', 'Technical', 'Research']) {
      await page.getByRole('button', { name: new RegExp(depth) }).click();
      for (const textSize of ['100%', '200%']) {
        await page.evaluate(size => document.documentElement.style.fontSize = size, textSize);
        // The original sticky copy overlapped the following grid row only after scrolling.
        for (const anchor of ['.explainer-copy', '.explainer-panels', '.explainer-next']) {
          await page.locator(anchor).scrollIntoViewIfNeeded();
          const overlaps = await page.locator('.cerebri-explainer').evaluate(el => {
            const box = (selector: string) => el.querySelector(selector)!.getBoundingClientRect();
            const panels = box('.explainer-panels');
            const stage = box('.explainer-stage');
            const blocks = [...el.querySelectorAll('.explainer-stage > *')]
              .filter(node => getComputedStyle(node).position !== 'absolute' && node.getBoundingClientRect().height > 0);
            return {
              copy: box('.explainer-copy').bottom - panels.top,
              stage: stage.bottom - panels.top,
              next: panels.bottom - box('.explainer-next').top,
              annotations: blocks.slice(1).map((node, index) => blocks[index].getBoundingClientRect().bottom - node.getBoundingClientRect().top)
            };
          });
          expect(Math.max(overlaps.copy, overlaps.stage, overlaps.next, ...overlaps.annotations)).toBeLessThanOrEqual(1);
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
  });
}

for (const viewport of [{ width: 1440, height: 600 }, { width: 390, height: 700 }]) {
  test(`short viewport ${viewport.width}px shows a complete unpinned field`, async ({ page }) => {
    await page.setViewportSize(viewport);
    await page.goto(prefix + '/');
    const track = page.locator('.observatory-track');
    await expect(track).toHaveClass(/static-view/);
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
  for (const section of ['explainer', 'workbench']) test(`@visual ${section} ${width}px`, async ({ page }) => {
    await page.setViewportSize({ width, height: 1000 });
    await page.emulateMedia({ reducedMotion: 'reduce' });
    await page.goto(prefix + '/');
    await page.evaluate(() => document.fonts.ready);
    const name = `${section}-${width}`;
    const selector = section === 'explainer' ? '.cerebri-explainer' : '.intent-field';
    const screenshot = await page.locator(selector).screenshot({ path: test.info().outputPath(name + '.png'), animations: 'disabled', style: '.site-header,.skip-link{visibility:hidden!important}' });
    await test.info().attach(name + '.png', { body: screenshot, contentType: 'image/png' });
    expect(createHash('sha256').update(screenshot).digest('hex')).toBe(visualBaselines[process.platform]?.[name]);
  });
}

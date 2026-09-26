import { expect, test } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { runtimeData } from '../src/lib/generated/runtime-data';
import { runtimeLayers } from '../src/lib/architecture-ownership';
import { visualBaselines } from './visual-baselines';

const prefix = process.env.CEREBRI_BASE_PATH ?? '';
const request = runtimeData.request;

test('CPIR envelope keeps canonical fields, empty sets and authority boundaries exact', async ({ page }) => {
  expect(request).toEqual(JSON.parse(readFileSync(new URL('../../../examples/request.json', import.meta.url), 'utf8')));
  await page.goto(prefix + '/cpir/');
  await page.getByRole('button', { name: 'Structured evidence', exact: true }).click();
  const input = page.locator('[data-schema="request"]');
  await expect(input.getByRole('definition').filter({ hasText: request.request_id })).toHaveCount(1);
  for (const [path, value] of [
    ['scope.time_range.start', request.scope.time_range.start],
    ['scope.time_range.end', request.scope.time_range.end],
    ['context.objects[1].time.value.knowledge.data.start', request.context.objects[1].time.value.knowledge.data.start],
    ['context.objects[1].time.value.knowledge.data.end', request.context.objects[1].time.value.knowledge.data.end]
  ]) {
    const field = input.locator('[data-field="' + path + '"]');
    await expect(field).toHaveAttribute('data-value', value);
    await expect(field.locator('time')).toHaveText(value);
  }
  for (const object of request.context.objects) {
    const point = input.locator('[data-object-id="' + object.id + '"]');
    await expect(point).toHaveAttribute('data-processing', object.time.value.processing);
    await expect(point).toHaveAttribute('data-knowledge', object.time.value.knowledge.state);
    await expect(point).toContainText(object.time.provenance);
  }
  await expect(input.locator('.empty-stratum').first()).toContainText('[] · empty array');
  await expect(input.locator('.preference-stratum')).toContainText('preferences.preferences');
  await expect(input.locator('.preference-stratum')).toContainText('[] · empty array');
  await expect(input.locator('.policy-stratum')).toContainText(request.policy.snapshot.mutation.allowed_actions.join(' · '));
  await expect(input.locator('.capability-stratum')).toContainText(request.planning_capability.mutations[0].kind);
  await expect(input.locator('.capability-stratum')).toContainText('Neither is execution authority.');
  for (const [key, value] of Object.entries(request.budget)) {
    await expect(input.locator('[data-field="budget.' + key + '"] dd')).toHaveText(String(value));
  }
  await expect(input.locator('.budget-stratum')).toContainText('Limits, not observed resource usage.');
  await expect(input.locator('[data-candidate-id]')).toHaveCount(0);
  expect(await page.locator('[data-schema],.planner-aperture').evaluateAll(nodes => nodes.map(el => el.getAttribute('data-schema') ?? 'boundary'))).toEqual(['request', 'boundary', 'result']);
  const result = page.locator('[data-schema="result"]');
  const candidate = runtimeData.plannerResult.candidates[0];
  await expect(result.locator('[data-result-record="candidates[0]"]')).toHaveAttribute('data-candidate-id', 'C05');
  await expect(result.locator('.candidate-coordinate')).toContainText('not a Rust object ID');
  await expect(result.locator('.placement dd').last()).toHaveText(candidate.object_id);
  for (const [key, value] of Object.entries(candidate.ordering_key)) {
    await expect(result.locator('[data-key-field="' + key + '"] b')).toHaveText(String(value));
  }
  await expect(result).toContainText(runtimeData.plannerResult.assessment);
});

test('geometry retains its endpoints and global depth remains the source of precision', async ({ page }) => {
  await page.goto(prefix + '/cpir/');
  const scope = page.locator('.scope-stratum .range-field').first();
  const endpoint = await scope.locator('.end i').elementHandle();
  const endValue = request.scope.time_range.end;
  for (const [name, precision] of [['Exact fields', '1'], ['Structured evidence', '2'], ['Concept', '0']]) {
    await page.getByRole('button', { name, exact: true }).press('Enter');
    await expect(scope).toHaveAttribute('data-precision', precision);
    expect(await endpoint!.evaluate(el => el === document.querySelector('.scope-stratum .range-field .end i'))).toBe(true);
    await expect(scope.locator('.end')).toHaveAttribute('data-value', endValue);
  }
  await page.getByRole('group', { name: 'Explanation depth' }).getByRole('button', { name: /Technical/ }).click();
  await expect(scope).toHaveAttribute('data-precision', '2');
  await page.getByRole('button', { name: 'Concept', exact: true }).click();
  await page.getByRole('button', { name: 'Follow depth', exact: true }).click();
  await expect(scope).toHaveAttribute('data-precision', '2');
  const scopeEnd = await scope.locator('.end').boundingBox();
  const scopeStart = await scope.locator('.start').boundingBox();
  await expect.poll(async () => (await scope.locator('.end').boundingBox())!.y - (await scope.locator('.start').boundingBox())!.y).toBeGreaterThan(65);
  expect(scopeEnd).not.toBeNull();
  expect(scopeStart).not.toBeNull();
});

test('architecture ownership matches manifests and research has no production connection', async ({ page }) => {
  for (const layer of runtimeLayers) {
    for (const owner of layer.owners) {
      const manifest = readFileSync(new URL('../../../' + owner.path, import.meta.url), 'utf8');
      const dependencies = [...manifest.matchAll(/^(cerebri-[a-z-]+)\.workspace = true$/gm)].map(match => match[1]).sort();
      expect([...owner.dependencies].sort(), owner.name).toEqual(dependencies);
    }
  }
  await page.goto(prefix + '/architecture/');
  const layer = await page.locator('[data-layer="planner"]').elementHandle();
  await page.getByRole('button', { name: 'Exploded ownership', exact: true }).press('Enter');
  for (const entry of runtimeLayers) {
    await page.getByRole('button', { name: 'Inspect ' + entry.concept, exact: true }).press('Enter');
    await expect(page.locator('.architecture-instrument')).toHaveAttribute('data-focused', entry.id);
    for (const owner of entry.owners) await expect(page.locator('[data-owner="' + owner.name + '"]')).toBeVisible();
  }
  expect(await layer!.evaluate(el => el === document.querySelector('[data-layer="planner"]'))).toBe(true);
  await expect(page.locator('.direction-seam')).toContainText('REQUEST ↓ DELEGATE INWARD');
  await expect(page.locator('.direction-seam')).toContainText('RESULT ↑ RETURN OUTWARD');
  await expect(page.locator('.build-ingress')).toContainText('build-site-data.mjs');
  await expect(page.locator('.lifecycle-boundary')).toContainText('ProposedPlan → ValidatedPlan');
  await expect(page.locator('.research-detachment')).toHaveAttribute('data-production-dependency', 'none');
  await expect(page.locator('.runtime-strata')).not.toContainText('cerebri-ml');
  await expect(page.locator('.research-detachment :is(svg,path,[data-owner])')).toHaveCount(0);
  await expect(page.locator('.page-planet,.page-orbit')).toHaveCount(0);
});

test('technical descent links retain request/result correspondence and open runtime ownership', async ({ page }) => {
  await page.goto(prefix + '/');
  await page.locator('.technical-inspection>summary').click();
  const bridge = page.getByRole('navigation', { name: 'Continue technical descent' });
  await expect(bridge.locator('.range-field')).toHaveAttribute('data-field', 'scope.time_range');
  await bridge.getByRole('link', { name: /C05/ }).click();
  await expect(page.locator('#result-title')).toBeVisible();
  await page.locator('.planner-result').getByRole('link', { name: /runtime ownership/ }).click();
  await expect(page.locator('.architecture-instrument')).toHaveAttribute('data-precision', '2');
  await expect(page.locator('#runtime-planner [data-owner="cerebri-planner"]')).toBeVisible();
});

for (const route of ['cpir', 'architecture']) {
  test(route + ' reduced motion and accessibility expose complete correspondence', async ({ page }) => {
    await page.emulateMedia({ reducedMotion: 'reduce' });
    await page.setViewportSize({ width: 390, height: 900 });
    await page.goto(prefix + '/' + route + '/');
    const instrument = page.locator('.' + route + '-instrument');
    await expect(instrument).toHaveAttribute('data-precision', '2');
    if (route === 'cpir') {
      await expect(instrument.locator('[data-field="scope.time_range.end"] time')).toBeVisible();
      await expect(instrument.locator('.range-endpoint').first()).toHaveCSS('transition-duration', '0s');
    } else {
      await expect(instrument.locator('[data-owner]')).toHaveCount(9);
      await expect(instrument.locator('.layer-surface').first()).toHaveCSS('transition-duration', '0s');
    }
    await page.locator('.technical-instrument details').evaluateAll(nodes => nodes.forEach(node => node.setAttribute('open', '')));
    expect((await new AxeBuilder({ page }).analyze()).violations).toEqual([]);
    await page.emulateMedia({ reducedMotion: 'no-preference' });
    await page.getByRole('button', { name: route === 'cpir' ? 'Concept' : 'Compressed', exact: true }).click();
    expect((await new AxeBuilder({ page }).include('.' + route + '-instrument').analyze()).violations).toEqual([]);
  });
}

for (const width of [320, 375, 390, 430, 768, 1280, 1440]) {
  test('technical routes retain complete bounds at ' + width + 'px', async ({ page }) => {
    await page.setViewportSize({ width, height: 900 });
    for (const route of ['cpir', 'architecture']) {
      await page.goto(prefix + '/' + route + '/');
      for (const motion of ['no-preference', 'reduce'] as const) {
        await page.emulateMedia({ reducedMotion: motion });
        if (motion === 'no-preference') {
          await page.getByRole('button', { name: route === 'cpir' ? 'Structured evidence' : 'Exploded ownership', exact: true }).click();
        }
        expect(await page.evaluate(() => document.documentElement.scrollWidth - innerWidth)).toBeLessThanOrEqual(1);
        const overflow = await page.locator('.' + route + '-instrument').evaluate(root => [...root.querySelectorAll('*')].filter(el => {
          const r = el.getBoundingClientRect();
          return r.width > 0 && (r.right > innerWidth + 1 || r.left < -1);
        }).map(el => el.className));
        expect(overflow).toEqual([]);
      }
    }
  });
}

for (const route of ['cpir', 'architecture']) {
  for (const state of ['concept', 'technical', 'mobile']) {
    test('@visual ' + route + ' ' + state, async ({ page }) => {
      await page.setViewportSize({ width: state === 'mobile' ? 390 : 1440, height: 1000 });
      await page.goto(prefix + '/' + route + '/');
      if (state !== 'concept') await page.emulateMedia({ reducedMotion: 'reduce' });
      const instrument = page.locator('.' + route + '-instrument');
      await expect(instrument).toHaveAttribute('data-precision', state === 'concept' ? '0' : '2');
      const name = route + '-' + state;
      const screenshot = await instrument.screenshot({ path: test.info().outputPath(name + '.png'), animations: 'disabled', style: '.site-header,.skip-link{visibility:hidden!important}' });
      await test.info().attach(name + '.png', { body: screenshot, contentType: 'image/png' });
      expect(createHash('sha256').update(screenshot).digest('hex')).toBe(visualBaselines[process.platform]?.[name]);
    });
  }
}

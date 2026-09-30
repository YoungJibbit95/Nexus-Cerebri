// Optional browser verification using the repository's existing site test tooling.
// Build Lab and Rust API first. No new dependency or planner implementation is introduced.
import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import { chromium } from '../../cerebri-site/node_modules/playwright/index.mjs';
import AxeBuilder from '../../cerebri-site/node_modules/@axe-core/playwright/dist/index.mjs';

const root = fileURLToPath(new URL('../../../', import.meta.url));
const evidence = process.env.LAB_EVIDENCE_DIR || join(tmpdir(), 'cerebri-lab-l1-l2');
await mkdir(evidence, { recursive: true });
const server = spawn(join(root, 'target/debug/cerebri-api' + (process.platform === 'win32' ? '.exe' : '')), [], {
  cwd: root, env: { ...process.env, CEREBRI_BIND_ADDR: '127.0.0.1:0' }, windowsHide: true, stdio: ['ignore', 'pipe', 'pipe'],
});
let browser;
try {
  const base = await new Promise((resolve, reject) => {
    let output = '', errors = '';
    const timer = setTimeout(() => reject(new Error('API startup timed out: ' + errors)), 15000);
    server.stdout.on('data', (chunk) => {
      output += chunk;
      const address = /CEREBRI_LISTEN_ADDR=(127\.0\.0\.1:\d+)/.exec(output)?.[1];
      if (address) { clearTimeout(timer); resolve('http://' + address); }
    });
    server.stderr.on('data', (chunk) => errors = (errors + chunk).slice(-2000));
    server.once('error', (error) => { clearTimeout(timer); reject(error); });
    server.once('exit', (code) => { clearTimeout(timer); reject(new Error('API exited: ' + code)); });
  });
  browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({ viewport: { width: 1600, height: 1000 } });
  const page = await context.newPage();
  async function capture(name, fullPage = true) {
    await page.evaluate(() => window.scrollTo(0, 0));
    await page.screenshot({ path: join(evidence, name), fullPage, animations: 'disabled' });
  }
  const errors = [];
  page.on('pageerror', (error) => errors.push(error.message));
  let planningCalls = 0, sent;
  page.on('request', (request) => { if (new URL(request.url()).pathname === '/v1/plan') { planningCalls++; sent = request.postDataJSON(); } });
  await page.goto(base + '/lab/');
  await page.getByRole('button', { name: /Run planner/ }).waitFor();
  await capture('planner-before-run.png');
  async function run(target = page) {
    const response = target.waitForResponse((response) => new URL(response.url()).pathname === '/v1/plan');
    await target.getByRole('button', { name: /Run planner/ }).click();
    const result = await (await response).json();
    await target.locator('.planner-scene[data-stage="5"]').waitFor();
    return result;
  }
  const result = await run();
  assert.equal(result.outcome, 'Solution');
  assert.deepEqual(await page.locator('.run-flow strong').allTextContents(), [String(result.search_space.evaluated), String(result.conflicts.rejections.length), String(result.candidates.length), '#1']);
  assert.equal(await page.locator('pre').count(), 0);
  assert.equal(await page.locator('.activity-drawer').getAttribute('open'), null);
  await page.getByText('Proposal ≠ execution. No calendar has been changed.', { exact: true }).waitFor();
  await capture('planner-understand.png');
  const originalRequest = structuredClone(sent);
  const downloadEvent = page.waitForEvent('download');
  await page.getByRole('button', { name: 'Export JSON', exact: true }).click();
  const exported = await downloadEvent;
  assert.deepEqual(JSON.parse(await readFile(await exported.path(), 'utf8')), result);
  async function depth(name) { await page.getByRole('button', { name: new RegExp('0[123] ' + name) }).click(); }
  for (const name of ['Technical', 'Research']) {
    await depth(name);
    const payload = page.locator('details.json-panel').filter({ hasText: 'Complete PlanningResult / all returned source data' }).locator('pre');
    assert.deepEqual(JSON.parse(await payload.textContent()), result);
    assert.equal(planningCalls, 1);
    assert.deepEqual(sent, originalRequest);
    await capture('planner-' + name.toLowerCase() + '.png', name === 'Technical');
    if (name === 'Research') {
      await page.locator('.research-context').scrollIntoViewIfNeeded();
      await page.screenshot({ path: join(evidence, 'planner-research-data.png'), animations: 'disabled' });
    }
  }
  assert.equal(await page.locator('.activity-drawer').getAttribute('open'), '');
  await depth('Understand');
  await page.getByRole('button', { name: 'Compare candidate 2', exact: true }).click();
  assert.equal(await page.locator('.time-field').getAttribute('data-selected-candidate'), '2');
  assert.equal(await page.locator('.comparison-instrument').getAttribute('data-selected-candidate'), '2');
  await page.getByRole('heading', { name: 'Candidate 2 · alternative', exact: true }).waitFor();
  await capture('planner-selected-alternative.png');
  await page.getByRole('button', { name: /^Rejected position 1,/ }).click();
  await page.locator('.rejection-evidence').waitFor();
  assert.ok(await page.locator('.blocking').count() > 0);
  await capture('planner-selected-rejection.png');
  // Real browser keyboard selection and focus visibility.
  await page.getByLabel('Inspect candidate', { exact: true }).focus();
  await page.keyboard.press('ArrowDown'); await page.keyboard.press('Enter');
  assert.ok(await page.getByLabel('Inspect candidate', { exact: true }).evaluate((element) => getComputedStyle(element).outlineStyle !== 'none'));
  const axe = await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag21aa']).analyze();
  if (axe.violations.length) console.log(JSON.stringify(axe.violations.map(({ id, nodes }) => ({ id, nodes: nodes.map(({ target, failureSummary }) => ({ target, failureSummary })) })), null, 2));
  assert.deepEqual(axe.violations.map(({ id, nodes }) => ({ id, nodes: nodes.map(({ target }) => target) })), []);
  for (const width of [1366, 820]) {
    await page.setViewportSize({ width, height: 900 });
    assert.ok(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), 'No page overflow at ' + width);
    await capture('planner-' + width + '.png');
    if (width === 820) {
      await page.locator('.comparison-instrument').scrollIntoViewIfNeeded();
      await page.screenshot({ path: join(evidence, 'planner-820-comparison.png'), animations: 'disabled' });
      await page.locator('.proposal-inspector').scrollIntoViewIfNeeded();
      await page.screenshot({ path: join(evidence, 'planner-820-proposal.png'), animations: 'disabled' });
    }
  }
  await page.setViewportSize({ width: 1600, height: 1000 });
  await page.getByRole('button', { name: /Switch to light theme/ }).click();
  const lightAxe = await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag21aa']).analyze();
  if (lightAxe.violations.length) console.log(JSON.stringify(lightAxe.violations.map(({ id, nodes }) => ({ id, nodes: nodes.slice(0, 3).map(({ target, failureSummary }) => ({ target, failureSummary })) })), null, 2));
  assert.deepEqual(lightAxe.violations.map(({ id, nodes }) => ({ id, nodes: nodes.map(({ target }) => target) })), []);
  await capture('planner-light.png');
  await page.getByRole('button', { name: /Switch to dark theme/ }).click();
  await depth('Technical');
  await page.getByRole('button', { name: 'Preferences', exact: true }).click();
  await capture('preferences.png');
  await page.getByRole('button', { name: 'Trace', exact: true }).click();
  await capture('trace.png');
  await page.getByRole('button', { name: 'Temporal', exact: true }).click();
  await page.getByRole('button', { name: 'Inspect temporal data', exact: true }).click();
  await page.getByText('2 visible occurrences · 1 skipped', { exact: false }).waitFor();
  await capture('temporal-dst.png');
  await page.getByLabel('Source coverage', { exact: true }).selectOption('Incomplete');
  await page.getByRole('button', { name: 'Inspect temporal data', exact: true }).click();
  await page.getByText('Incomplete coverage', { exact: true }).waitFor();
  await page.getByText('0 free intervals', { exact: true }).waitFor();
  assert.ok(await page.locator('.unknown-block').count() > 0);
  assert.equal(await page.locator('.free-block').count(), 0);
  await capture('temporal-incomplete.png');
  await page.reload();
  assert.equal(await page.locator('.app-shell').getAttribute('data-depth'), 'Technical');
  await page.getByRole('combobox', { name: /Source scenario/ }).selectOption('hard-rejection.json');
  const blocked = await run();
  assert.equal(blocked.outcome, 'NoSolution');
  assert.equal(await page.getByRole('button', { name: /^Compare candidate/ }).count(), 0);
  await page.getByText('No feasible proposal returned', { exact: true }).waitFor();
  await capture('planner-no-solution.png');
  const reducedContext = await browser.newContext({ viewport: { width: 1600, height: 1000 }, reducedMotion: 'reduce' });
  const reducedPage = await reducedContext.newPage();
  reducedPage.on('pageerror', (error) => errors.push(error.message));
  await reducedPage.goto(base + '/lab/');
  const reducedResult = await run(reducedPage);
  assert.deepEqual(reducedResult, result);
  assert.equal(await reducedPage.locator('.planner-scene').getAttribute('data-stage'), '5');
  await reducedPage.getByRole('button', { name: 'Replay explanation', exact: true }).click();
  assert.equal(await reducedPage.locator('.planner-scene').getAttribute('data-stage'), '5');
  // Source-less imported results never borrow facts or preferences from the current request.
  await reducedPage.getByRole('button', { name: 'Import result', exact: true }).click();
  await reducedPage.getByLabel('Import PlanningResult', { exact: true }).setInputFiles({ name: 'result.json', mimeType: 'application/json', buffer: Buffer.from(JSON.stringify(result)) });
  await reducedPage.getByText('Imported result: no matching source request is attached.', { exact: false }).waitFor();
  assert.equal(await reducedPage.locator('.time-field .busy-block').count(), 0);
  assert.equal(await reducedPage.locator('.time-field .preference-marker').count(), 0);
  assert.deepEqual(errors, []);
  console.log(JSON.stringify({ checked: ['real API counts', 'depth/data parity', 'shared selection', 'rejection/blocker evidence', 'keyboard', 'axe dark and light', '1600/1366/820 widths', 'DST and incomplete coverage', 'persistence', 'scenario loading and no solution', 'complete JSON export', 'reduced motion', 'source-less import'], evidence, result: { evaluated: result.search_space.evaluated, rejected: result.conflicts.rejections.length, feasible: result.candidates.length } }, null, 2));
} finally {
  await browser?.close();
  if (server.exitCode === null && server.signalCode === null) {
    await new Promise((resolve) => { server.once('close', resolve); server.kill(); });
  }
}

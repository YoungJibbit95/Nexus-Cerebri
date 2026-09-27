# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: apps\cerebri-site\tests\site.spec.ts >> homepage and planning surface meet the serious axe floor
- Location: apps\cerebri-site\tests\site.spec.ts:112:1

# Error details

```
Error: page.goto: Protocol error (Page.navigate): Cannot navigate to invalid URL
Call log:
  - navigating to "/Nexus-Cerebri/", waiting until "load"

```

# Test source

```ts
  14  |   }
  15  | });
  16  | 
  17  | test('explanation depth changes visible information and persists', async ({ page }) => {
  18  |   await page.goto(prefix + '/architecture/');
  19  |   const technical = page.locator('.depth-technical');
  20  |   const research = page.locator('.depth-research');
  21  |   await expect(technical).toBeHidden();
  22  |   await expect(research).toBeHidden();
  23  |   await page.getByRole('button', { name: /Technical/ }).click();
  24  |   await expect(technical).toBeVisible();
  25  |   await expect(research).toBeHidden();
  26  |   await page.reload();
  27  |   await expect(technical).toBeVisible();
  28  |   await page.getByRole('button', { name: /Research/ }).click();
  29  |   await expect(research).toBeVisible();
  30  | });
  31  | 
  32  | test('homepage reflects the current structured deterministic planning boundary', async ({ page }) => {
  33  |   await page.goto(prefix + '/');
  34  |   const heroHeading = page.locator('h1').first();
  35  |   await expect(heroHeading).toContainText('Bound the problem.');
  36  |   await expect(heroHeading).toContainText('Verify the plan.');
  37  |   await expect(page.getByText('Current CPIR planning fixture', { exact: true })).toBeVisible();
  38  |   await expect(page.getByText('11 evaluated positions', { exact: true }).first()).toBeVisible();
  39  |   await expect(page.getByText('7 valid / 4 rejected', { exact: true })).toBeVisible();
  40  |   await expect(page.getByText('Structured evidence. Bounded search. Explicit authority.', { exact: true })).toBeVisible();
  41  |   await expect(page.getByText(/It can explore meaning and ambiguity/i)).toHaveCount(0);
  42  |   await expect(page.getByText(/HUMAN \/ PROBABILISTIC/i)).toHaveCount(0);
  43  | });
  44  | 
  45  | test('semantic planning visuals preserve candidate and authority distinctions', async ({ page }) => {
  46  |   await page.goto(prefix + '/');
  47  | 
  48  |   const observatory = page.locator('.observatory');
  49  |   await expect(observatory).toBeVisible();
  50  |   await expect(observatory).toHaveAttribute('aria-label', /Bounded search observatory/);
  51  | 
  52  |   const candidates = page.locator('ol[aria-label="Evaluated candidate intervals"] > li');
  53  |   await expect(candidates).toHaveCount(11);
  54  |   await expect(page.locator('.candidate-lanes li[data-state="rejected"]')).toHaveCount(4);
  55  |   await expect(page.locator('.candidate-lanes li[data-state="valid"]')).toHaveCount(6);
  56  |   await expect(page.locator('.candidate-lanes li[data-state="selected"]')).toHaveCount(1);
  57  | 
  58  |   const authorityBoundary = page.locator('.proposal-boundary');
  59  |   await expect(authorityBoundary).toHaveAttribute('aria-label', /Selected proposal at 10:00 UTC stops before a separate authority gate/);
  60  |   await expect(authorityBoundary.getByText('AUTHORITY GATE', { exact: true })).toBeVisible();
  61  |   await expect(authorityBoundary.getByText('EXECUTION', { exact: true })).toBeVisible();
  62  | 
  63  |   const instrument = page.locator('.planning-instrument');
  64  |   await expect(instrument.locator('[aria-label="Planning visual grammar"]')).toBeVisible();
  65  |   await expect(instrument.locator('[data-semantic-kind="scope"]')).toHaveCount(1);
  66  |   await expect(instrument.locator('[data-semantic-kind="fact"]')).toHaveCount(1);
  67  |   await expect(instrument.locator('[data-semantic-kind="constraint"]')).toHaveCount(1);
  68  |   await expect(instrument.locator('[data-semantic-kind="preference"]')).toHaveCount(1);
  69  | 
  70  |   const semanticFlow = instrument.getByRole('region', { name: /Candidate to authority flow/i });
  71  |   await expect(semanticFlow).toBeVisible();
  72  |   await expect(semanticFlow.locator('[data-flow-node]')).toHaveCount(4);
  73  |   await expect(semanticFlow.locator('[data-flow-node="candidates"]')).toContainText('7 valid after hard-rule checks');
  74  |   await expect(semanticFlow.locator('[data-flow-node="comparator"]')).toContainText('Rust-produced order');
  75  |   await expect(semanticFlow.locator('[data-flow-node="proposal"]')).toContainText('10:00 UTC remains a proposal');
  76  |   await expect(semanticFlow.locator('[data-flow-node="authority"]')).toContainText('authorization stay separate');
  77  | 
  78  |   const comparator = instrument.getByRole('region', { name: /Deterministic comparator/i });
  79  |   await expect(comparator).toBeVisible();
  80  |   await expect(comparator.locator('.comparator-row.decisive')).toHaveAttribute('data-key-field', 'start');
  81  |   await expect(comparator.locator('.comparator-resolution')).toContainText('First differing key:');
  82  | 
  83  |   const lifecycle = instrument.getByRole('region', { name: /Planner authority boundary/i });
  84  |   await expect(lifecycle).toBeVisible();
  85  |   await expect(lifecycle.locator('[data-authority-stage="proposal"]')).toHaveClass(/current/);
  86  |   await expect(lifecycle.locator('[data-authority-stage="authorized"]')).not.toHaveClass(/reached/);
  87  | });
  88  | 
  89  | test('reduced motion resolves the hero directly to an equivalent semantic state', async ({ page }) => {
  90  |   await page.emulateMedia({ reducedMotion: 'reduce' });
  91  |   await page.goto(prefix + '/');
  92  |   const observatory = page.locator('.observatory');
  93  |   await expect(observatory).toHaveAttribute('data-motion', 'reduced');
  94  |   await expect(observatory).toHaveClass(/proposed/);
  95  | 
  96  |   const instrument = page.locator('.planning-instrument');
  97  |   await expect(instrument).toHaveAttribute('data-motion', 'reduced');
  98  |   await expect(instrument).toHaveAttribute('data-flow-stage', '3');
  99  |   await expect(instrument.locator('[data-flow-node="authority"]')).toHaveClass(/current/);
  100 | });
  101 | 
  102 | test('structured evidence inspection is keyboard operable', async ({ page }) => {
  103 |   await page.goto(prefix + '/');
  104 |   const inspect = page.locator('.inspect-button');
  105 |   await inspect.focus();
  106 |   await expect(inspect).toBeFocused();
  107 |   await page.keyboard.press('Enter');
  108 |   await expect(inspect).toHaveAttribute('aria-expanded', 'true');
  109 |   await expect(page.getByText('SCOPE / CANONICAL INPUT', { exact: true })).toBeVisible();
  110 | });
  111 | 
  112 | test('homepage and planning surface meet the serious axe floor', async ({ page }) => {
  113 |   for (const route of ['/', '/planning/']) {
> 114 |     await page.goto(prefix + route);
      |                ^ Error: page.goto: Protocol error (Page.navigate): Cannot navigate to invalid URL
  115 |     if (route === '/') {
  116 |       const releaseStatus = page.getByRole('group', { name: 'Release status' });
  117 |       await expect(releaseStatus.getByText('Published release: None', { exact: true })).toBeVisible();
  118 |       await expect(releaseStatus.getByText('Unreleased · qualification in progress', { exact: true })).toBeVisible();
  119 |       await expect(releaseStatus.getByText('CPIR 0.2', { exact: true })).toBeVisible();
  120 |     }
  121 |     const results = await new AxeBuilder({ page }).analyze();
  122 |     const blocking = results.violations.filter((violation) => violation.impact === 'serious' || violation.impact === 'critical');
  123 |     expect(blocking, JSON.stringify(blocking, null, 2)).toEqual([]);
  124 |   }
  125 | });
  126 | 
  127 | for (const width of [1440, 1280, 1024, 768, 430, 390, 375, 320]) {
  128 |   test('responsive layout has no horizontal overflow at ' + width + 'px', async ({ page }) => {
  129 |     await page.setViewportSize({ width, height: 900 });
  130 |     await page.goto(prefix + '/');
  131 |     const overflow = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
  132 |     expect(overflow).toBeLessThanOrEqual(1);
  133 |   });
  134 | }
  135 | 
  136 | test('homepage remains overflow-safe with enlarged root text', async ({ page }) => {
  137 |   await page.setViewportSize({ width: 390, height: 900 });
  138 |   await page.goto(prefix + '/');
  139 |   await page.evaluate(() => document.documentElement.style.fontSize = '200%');
  140 |   const overflow = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
  141 |   expect(overflow).toBeLessThanOrEqual(1);
  142 | });
  143 | 
  144 | for (const width of [1440, 1024, 768, 390]) {
  145 |   test('@visual homepage ' + width + 'px', async ({ page }) => {
  146 |     await page.emulateMedia({ reducedMotion: 'reduce' });
  147 |     await page.setViewportSize({ width, height: 1000 });
  148 |     await page.goto(prefix + '/');
  149 |     const screenshot = await page.screenshot({ path: test.info().outputPath('homepage-' + width + '.png'), fullPage: true, animations: 'disabled' });
  150 |     const hash = createHash('sha256').update(screenshot).digest('hex');
  151 |     const expected = visualBaselines[process.platform]?.[String(width)];
  152 |     expect(expected, 'reviewed visual baseline for ' + process.platform + ' at ' + width + 'px').toBeDefined();
  153 |     if (hash !== expected) {
  154 |       await test.info().attach('homepage-' + width + '.png', { body: screenshot, contentType: 'image/png' });
  155 |     }
  156 |     expect(hash, 'visual hash for ' + width + 'px').toBe(expected);
  157 |   });
  158 | }
  159 | 
```
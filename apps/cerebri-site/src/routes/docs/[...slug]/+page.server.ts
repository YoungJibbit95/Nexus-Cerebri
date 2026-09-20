import { error } from '@sveltejs/kit';
import MarkdownIt from 'markdown-it';
import { posix } from 'node:path';
import { docsData } from '$lib/generated/docs-data';

export const prerender = true;

const markdown = new MarkdownIt({ html: true, linkify: false, typographer: false });
const bySource = new Map<string, (typeof docsData)[number]>();
for (const doc of docsData) bySource.set(doc.sourcePath, doc);

markdown.renderer.rules.image = (tokens, idx) => {
  const alt = markdown.utils.escapeHtml(tokens[idx].content || 'repository image');
  return `<span class="doc-image-alt" role="img" aria-label="${alt}">[image: ${alt}]</span>`;
};

export function entries() {
  return docsData.map((doc) => ({ slug: doc.slug }));
}

function rewriteInternalLinks(raw: string, sourcePath: string) {
  return raw.replace(/\]\(([^)]+\.md(?:#[^)\s]+)?)(?:\s+"[^"]*")?\)/g, (whole, href: string) => {
    if (/^[a-z]+:/i.test(href)) return whole;
    const [pathPart, hash] = href.split('#');
    const resolved = posix.normalize(posix.join(posix.dirname(sourcePath), pathPart));
    const target = bySource.get(resolved);
    if (!target) return whole;
    return `](__BASE__/docs/${target.slug}/${hash ? `#${hash}` : ''})`;
  });
}

export function load({ params }) {
  const requestedSlug = (params.slug ?? '').replace(/^\\/+|\\/+$/g, '');
  const doc = docsData.find((item) => item.slug === requestedSlug);
  if (!doc) error(404, 'Documentation page not found');
  const raw = doc.raw.replace(/^<!-- doc:[^]*?-->\s*/, '');
  return {
    title: doc.title,
    language: doc.language,
    status: doc.status,
    sourcePath: doc.sourcePath,
    sourceUrl: doc.sourceUrl,
    html: markdown.render(rewriteInternalLinks(raw, doc.sourcePath))
  };
}

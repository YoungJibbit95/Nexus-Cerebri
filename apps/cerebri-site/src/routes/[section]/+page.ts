import { error } from '@sveltejs/kit';
import { sectionKeys, sections, type SectionKey } from '$lib/site';

export const prerender = true;

export function entries() {
  return sectionKeys.map((section) => ({ section }));
}

export function load({ params }) {
  const key = params.section as SectionKey;
  const section = sections[key];
  if (!section) error(404, 'Unknown Cerebri surface');
  return { key, section };
}

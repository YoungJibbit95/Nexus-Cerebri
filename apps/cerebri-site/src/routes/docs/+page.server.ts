import { docsData } from '$lib/generated/docs-data';

export const prerender = true;

export function load() {
  return {
    docs: docsData.map(({ slug, sourcePath, title, language, status }) => ({
      slug,
      sourcePath,
      title,
      language,
      status
    }))
  };
}

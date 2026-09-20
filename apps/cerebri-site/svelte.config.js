import adapter from '@sveltejs/adapter-static';

const base = process.env.CEREBRI_BASE_PATH ?? (process.env.GITHUB_ACTIONS ? '/Nexus-Cerebri' : '');

export default {
  kit: {
    adapter: adapter({
      pages: '../../site',
      assets: '../../site',
      precompress: true,
      strict: true
    }),
    paths: { base },
    prerender: {
      handleHttpError: 'fail'
    }
  }
};

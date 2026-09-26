// Exact hashes are renderer/platform dependent. Keep each platform independently reviewed.
export const visualBaselines: Partial<Record<NodeJS.Platform, Record<string, string>>> = {
  linux: {
    '1440': '8f72b97bf7d537640ae7ec25eec971c6bfacb39fb2009e94fe933e9704565c01',
    '1024': '18d7b71bc747350976396b0e4eaaaa98a0aa3ee44a28acd1d02dde331f772fea',
    '768': '0d6f70a2d7740e78ed44b69149db94321596f4529e22562fb38ed31d672d0eae',
    '390': '916cec36966e630a1bef87420206c5df17d71eaa9757f19618212f7a731e14a6'
  },
  win32: {
    '1440': 'ff4cb454985314ac7bc67d9d29c2a57ac09de307ae4ad02524adf8681eca0c6e',
    '1024': '77e3cfab0ac27e51e91d0956029eceb4d3f650c8f7fee84be5ca37f7fba97b92',
    '768': 'ff9d572ffd7798e4da723367ebc096a63d5e71af0ea670780b52e2e294e8c47d',
    '390': '676b859ff186935fdb746744b887adb5cf0d8e03537920b6290eb6de47beebc4'
  }
};

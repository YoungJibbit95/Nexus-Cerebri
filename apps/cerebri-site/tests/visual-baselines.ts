// Exact hashes are renderer/platform dependent. Keep each platform independently reviewed.
export const visualBaselines: Partial<Record<NodeJS.Platform, Record<string, string>>> = {
  linux: {
  '1440': '21c5fa3912596e49eace781971e4fa7b4be19fec176323c6d85309f3c63fd6c4',
  '1024': '9f04d6695e372c8a8b682db85d11aeceb41b6ef625832db69fd1fb7bbdc3d490',
  '768': 'dd0da6bf0434c7e55e859d76a0854d1710c176e5535ef29d5a2579f93c84d32a',
  '390': '610cbddde178bd35df093df1996ba1d444c2d3826fef7d3f3b3025a83d57d028'
  },
  win32: {
    '1440': 'ff4cb454985314ac7bc67d9d29c2a57ac09de307ae4ad02524adf8681eca0c6e',
    '1024': '77e3cfab0ac27e51e91d0956029eceb4d3f650c8f7fee84be5ca37f7fba97b92',
    '768': 'ff9d572ffd7798e4da723367ebc096a63d5e71af0ea670780b52e2e294e8c47d',
    '390': '676b859ff186935fdb746744b887adb5cf0d8e03537920b6290eb6de47beebc4'
  }
};

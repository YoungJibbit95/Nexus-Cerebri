// Exact hashes are renderer/platform dependent. Keep each platform independently reviewed.
export const visualBaselines: Partial<Record<NodeJS.Platform, Record<string, string>>> = {
  linux: {
    '1440': 'eeae90c16c3190b10def3bf2ffb22cddcea227986f2848985a64667d49d0b722',
    '1024': '8ee1116509e52fa74a84e15859ee5ff045c22ecba4b06273fd88735be8c100cc',
    '768': 'cd8041fa7eaeb794943e61158cc902595d3dfcdfd6bc9da2b797c6b14da288f9',
    '390': '290a317a7d307f9bb633431f333e8e5300c56d2d792ba347d13e8915105c641d'
  },
  win32: {
    '1440': 'ff4cb454985314ac7bc67d9d29c2a57ac09de307ae4ad02524adf8681eca0c6e',
    '1024': '77e3cfab0ac27e51e91d0956029eceb4d3f650c8f7fee84be5ca37f7fba97b92',
    '768': 'ff9d572ffd7798e4da723367ebc096a63d5e71af0ea670780b52e2e294e8c47d',
    '390': '676b859ff186935fdb746744b887adb5cf0d8e03537920b6290eb6de47beebc4'
  }
};

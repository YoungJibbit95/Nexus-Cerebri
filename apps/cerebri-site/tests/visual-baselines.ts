// Exact hashes are renderer/platform dependent. Keep each platform independently reviewed.
export const visualBaselines: Partial<Record<NodeJS.Platform, Record<string, string>>> = {
  linux: {
    '1440': 'f9675ceeff3afab80d1d777e16ef3147de17b6a703ca550c60b05c15e033dd74',
    '1024': '4f44825814d35eebda2a5e9047deb8d1c4888c89d745d0a48aa8e6fc775096c2',
    '768': '378e98e715621030a46559a868df7bf633abb85f0d6d51b54c59c0f8e3e90406',
    '390': '96da73d41ea650742cb111e8870487188e21cf06d505e0b64a41066d25d3b47f'
  },
  win32: {
    '1440': 'ff4cb454985314ac7bc67d9d29c2a57ac09de307ae4ad02524adf8681eca0c6e',
    '1024': '77e3cfab0ac27e51e91d0956029eceb4d3f650c8f7fee84be5ca37f7fba97b92',
    '768': 'ff9d572ffd7798e4da723367ebc096a63d5e71af0ea670780b52e2e294e8c47d',
    '390': '676b859ff186935fdb746744b887adb5cf0d8e03537920b6290eb6de47beebc4'
  }
};

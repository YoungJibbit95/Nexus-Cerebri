// Exact hashes are renderer/platform dependent. Keep each platform independently reviewed.
export const visualBaselines: Partial<Record<NodeJS.Platform, Record<string, string>>> = {
  linux: {
    '1440': 'f42b4776885a0d9bb2ac8b5ee69122983015134cb8e066397a41ff7be05de3ab',
    '1024': '1cd72960b5dc2c4d7666adf1d6fd0f74d33cf2fb74cc35b343bc3f046a704aa6',
    '768': '293287d00897ecd8bcfed28ef68284c957904fa3d38f73c2e2e0c8bd3a6f3528',
    '390': '9213674a52dcf095e38c2d8d8652372cf4910bb88976ffdb01512fe5b2fc09c4'
  },
  win32: {
    '1440': 'ff4cb454985314ac7bc67d9d29c2a57ac09de307ae4ad02524adf8681eca0c6e',
    '1024': '77e3cfab0ac27e51e91d0956029eceb4d3f650c8f7fee84be5ca37f7fba97b92',
    '768': 'ff9d572ffd7798e4da723367ebc096a63d5e71af0ea670780b52e2e294e8c47d',
    '390': '676b859ff186935fdb746744b887adb5cf0d8e03537920b6290eb6de47beebc4'
  }
};

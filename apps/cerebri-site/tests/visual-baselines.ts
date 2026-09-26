// Exact hashes are renderer/platform dependent. Keep each platform independently reviewed.
export const visualBaselines: Partial<Record<NodeJS.Platform, Record<string, string>>> = {
  linux: {
    '1440': 'a39d41a1f7744355c18727dc82e61000dbbcb06c00df738d8a5d8c6227f374c5',
    '1024': '96a1a62732752dad2d5e2cabe4c5dfd671d2f8511fceaa50eee9df63ae71024a',
    '768': 'c44df4cc9a4297490a1dcebb3709362a6a737607be045f1a0575bb578fc927d4',
    '390': '3e96ecdba82912111461c24403e1ceebf22c99d43cd2a2f12b34733454ddf68d'
  },
  win32: {
    '1440': 'ff4cb454985314ac7bc67d9d29c2a57ac09de307ae4ad02524adf8681eca0c6e',
    '1024': '77e3cfab0ac27e51e91d0956029eceb4d3f650c8f7fee84be5ca37f7fba97b92',
    '768': 'ff9d572ffd7798e4da723367ebc096a63d5e71af0ea670780b52e2e294e8c47d',
    '390': '676b859ff186935fdb746744b887adb5cf0d8e03537920b6290eb6de47beebc4'
  }
};

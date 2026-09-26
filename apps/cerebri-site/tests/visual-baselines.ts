// Exact hashes are renderer/platform dependent. Keep each platform independently reviewed.
export const visualBaselines: Partial<Record<NodeJS.Platform, Record<string, string>>> = {
  linux: {
    '1440': 'ef5b4dff3461099d50a094dcf72381590a8657b450528a785efb7ba31e407026',
    '1024': '6e9d5d394454190be259d0af32696b36f21b016aac1e73569b4eefb5ca61898b',
    '768': 'b66adae6c35bc9f51cb7598f987199b3a9f923e6a5790ce71aea25f4161d21eb',
    '390': '3fb0b8ec5866b88bfb7fe8ad3602410205e35c0930a06badf48e801c8d9110c5'
  },
  win32: {
    '1440': 'ff4cb454985314ac7bc67d9d29c2a57ac09de307ae4ad02524adf8681eca0c6e',
    '1024': '77e3cfab0ac27e51e91d0956029eceb4d3f650c8f7fee84be5ca37f7fba97b92',
    '768': 'ff9d572ffd7798e4da723367ebc096a63d5e71af0ea670780b52e2e294e8c47d',
    '390': '676b859ff186935fdb746744b887adb5cf0d8e03537920b6290eb6de47beebc4'
  }
};

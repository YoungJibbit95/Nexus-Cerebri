// Exact hashes are renderer/platform dependent. Keep each platform independently reviewed.
export const visualBaselines: Partial<Record<NodeJS.Platform, Record<string, string>>> = {
  linux: {
    '1440': '59bf6faa912bc76c6a9bf4ba338104b900e0c43e1b42360443f1b0d6a4b28899',
    '1024': '8fed22d1a7cd4b916bca0f5d265cd3db18ef05b374049f24a2c20c5599c58ca6',
    '768': 'ccc9ef57aafd8acd9b63c6994225cbd7db43da2908ce4ecf7edc0eb26f7a4a34',
    '390': '125bbe826e54490f8212438ee3622bf3aca4f0f3963384e5eb479af7ba164482'
  },
  win32: {
    '1440': 'ff4cb454985314ac7bc67d9d29c2a57ac09de307ae4ad02524adf8681eca0c6e',
    '1024': '77e3cfab0ac27e51e91d0956029eceb4d3f650c8f7fee84be5ca37f7fba97b92',
    '768': 'ff9d572ffd7798e4da723367ebc096a63d5e71af0ea670780b52e2e294e8c47d',
    '390': '676b859ff186935fdb746744b887adb5cf0d8e03537920b6290eb6de47beebc4'
  }
};

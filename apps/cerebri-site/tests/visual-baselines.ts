// Exact hashes are renderer/platform dependent. Keep each platform independently reviewed.
export const visualBaselines: Partial<Record<NodeJS.Platform, Record<string, string>>> = {
  linux: {
    '1440': 'f42b4776885a0d9bb2ac8b5ee69122983015134cb8e066397a41ff7be05de3ab',
    '1024': '1cd72960b5dc2c4d7666adf1d6fd0f74d33cf2fb74cc35b343bc3f046a704aa6',
    '768': '293287d00897ecd8bcfed28ef68284c957904fa3d38f73c2e2e0c8bd3a6f3528',
    '390': '9213674a52dcf095e38c2d8d8652372cf4910bb88976ffdb01512fe5b2fc09c4'
  },
  win32: {
    '1024': '25930de4b0dcbebb75f7ee3c111bd4a84dc4e97a08c46d1a37bdf42196d4d0a9',
    '1440': '8d36e4c54fd8dbd9a56541a3e5ffe53cb2735891af3f289c2a49809ccb2c0b18',
    '390': 'd1d77cd52cec4d8d77d9ec4bc068aea214e548520627768b293574ace047307c',
    '768': '3e355bf3831e913df8881ecbf585ed87a5e7faabf82b58154cd67ce2200d4471',
    'math-1440': '7b26fc8345b72fd7cb9a66bffd1193d398c3ffa38d0d4d3b12b504735670d3b7',
    'math-390': 'b62ca5f56bb73218e2988cdfa609b63c4751ff6e1d7ffeee648604d677c11b8c',
    'math-768': '089a65eccbd4181b7845572353c939b67d351b2783ea540be0aba3de1c3de88b',
    'technical-1440': '903f741fcbe6b37610b8759bd1b6c23a01094c149085ad883d8ff6812bac445e',
    'technical-390': 'bc3bafbff67dc9a0f947f7bb15f95b79c2d18b889624fb641789a00157cd4f05',
    'technical-768': 'a27396c1d6b698e3e06df045252a5018f546513f78087ad22992d15c52e515f2'
  }
};

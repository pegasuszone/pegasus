import codegen from '@cosmwasm/ts-codegen'

codegen({
  contracts: [
    {
      name: 'Trade',
      dir: '../schema',
    },
  ],
  outPath: './types/',

  // options are completely optional ;)
  options: {
    bundle: {
      enabled: false,
    },
    types: {
      enabled: true,
    },
    client: {
      enabled: true,
    },
    reactQuery: {
      enabled: true,
      optionalClient: true,
      version: 'v4',
      mutations: true,
      queryKeys: true,
    },
    recoil: {
      enabled: false,
    },
    messageComposer: {
      enabled: true,
    },
  },
}).then(() => {
  console.log('✨ all done!')
})

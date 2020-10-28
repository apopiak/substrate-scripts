console.log(api.createType('RefCount').toRawType())
console.log(JSON.stringify(
  api
    .createType('AccountInfo', '0x4f00000001dcde4d05296d000000000000000000000081e13981050000000000000000000000de5c91256d0000000000000000000000000000000000000000000000000000')
    .toHuman()
    ))
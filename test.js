const crypto = require('crypto')

const { createHash, createHashClassHash } = require('./index')

const hasher = crypto.createHash('sha256')
hasher.update('hello world!!!')
console.log(hasher.digest('hex'))

const binding = createHash('sha256')
binding.update('hello world!!!')
console.log(binding.digest('hex'))

const bindingNativeClass = createHashClassHash('sha256')
bindingNativeClass.update(Buffer.from('hello world!!!'))
console.log(binding.digest('hex'))

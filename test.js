const crypto = require('crypto')

const { createHash } = require('./index')

const hasher = crypto.createHash('sha256')
hasher.update('hello world')
hasher.update('!!!')
console.log(hasher.digest('hex'))

const bindingHasher = createHash('sha256')
bindingHasher.update('hello world')
bindingHasher.update('!!!')
console.log(bindingHasher.digest('hex'))

const crypto = require('crypto')

const binding = require('./index')

const hasher = crypto.createHash('sha256')
hasher.update('hello world!!!')
console.log(hasher.digest('hex'))

console.log(binding.sha256('hello world!!!'))

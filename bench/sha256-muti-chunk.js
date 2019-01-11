const Benchmark = require('benchmark')
const crypto = require('crypto')
const { chain } = require('lodash')
const { createHash } = require('../index')

const suite = new Benchmark.Suite()

const fixture = chain()
  .range(100)
  .map(() => Buffer.from('hello'))
  .value()

suite
  .add('sha256#binding-js-class', () => {
    const hasher = createHash('sha256')
    fixture.forEach((b) => {
      hasher.update(b)
    })
    hasher.digest('hex')
  })
  .add('sha256#binding-native-class', () => {
    const hasher = createHash('sha256')
    fixture.forEach((b) => {
      hasher.update(b)
    })
    hasher.digest('hex')
  })
  .add('sha256#native', () => {
    const hasher = crypto.createHash('sha256')
    fixture.forEach((b) => {
      hasher.update(b)
    })
    hasher.digest('hex')
  })
  .on('cycle', function(event) {
    console.log(String(event.target))
  })
  .on('complete', function() {
    console.log('Fastest is ' + this.filter('fastest').map('name'))
  })
  .run()

const Benchmark = require('benchmark')
const { createHash } = require('crypto')
const { md5 } = require('crypto-wasm')
const { MD5 } = require('crypto-js')
const bindings = require('../index')

const fixture = 'hello world!'

const suite = new Benchmark.Suite()

suite
  .add('md5#binding', () => {
    const hasher = bindings.createHash('md5')
    hasher.update(fixture)
    hasher.digest('hex')
  })
  .add('md5#native', () => {
    const hasher = createHash('md5')
    hasher.update(fixture)
    hasher.digest('hex')
  })
  .add('md5#wasm', () => {
    md5(fixture)
  })
  .add('md5#js', () => {
    MD5(fixture).toString()
  })
  .on('cycle', function(event) {
    console.log(String(event.target))
  })
  .on('complete', function() {
    console.log('Fastest is ' + this.filter('fastest').map('name'))
  })
  .run()

const fs = require('fs')
const Benchmark = require('benchmark')
const crypto = require('crypto')
const { sha256 } = require('crypto-wasm')
const { SHA256 } = require('crypto-js')
const { createHash } = require('../index')

const suite = new Benchmark.Suite

const fixture = 'hello world!'
// const fixtureBuffer = fs.readFileSync('./bench/fixture.json')
// const fixture = fixtureBuffer.toString()

suite.add('sha256#native', () => {
  const hasher = crypto.createHash('sha256')
  hasher.update(fixture)
  hasher.digest('hex')
})
  .add('sha256#wasm', () => {
    sha256(fixture)
  })
  .add('sha256#js', () => {
    SHA256(fixture).toString()
  })
  .add('sha256#binding', () => {
    const hasher = createHash('sha256')
    hasher.update(fixture)
    hasher.digest('hex')
  })
  .on('cycle', function(event) {
    console.log(String(event.target))
  })
  .on('complete', function() {
    console.log('Fastest is ' + this.filter('fastest').map('name'))
  })
  .run()

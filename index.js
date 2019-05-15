const { hash, JsHasher } = require('./native.node')

class Hasher {
  constructor(method) {
    this.method = method
    /**
     * @type {Buffer}
     */
    this.buffer = Buffer.alloc(0)
  }

  update(data) {
    let dataBuffer
    if (typeof data === 'string') {
      dataBuffer = Buffer.from(data)
    } else if (Buffer.isBuffer(data)) {
      dataBuffer = data
    } else {
      throw new TypeError('Expect data to be Buffer or String')
    }
    const oldBuffer = this.buffer
    const oldArrLength = oldBuffer.length
    this.buffer = Buffer.alloc(oldArrLength + dataBuffer.length)
    this.buffer.set(oldBuffer)
    this.buffer.set(dataBuffer, oldArrLength)
  }

  digest(encoding) {
    const data = Buffer.from(this.buffer.buffer)
    return hash(this.method, data, encoding)
  }
}

module.exports = {
  createHash: function createHash(method) {
    return new Hasher(method)
  },

  createHashClassHash: function createHash(method) {
    return new JsHasher(method)
  },
}

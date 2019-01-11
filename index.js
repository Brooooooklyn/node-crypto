const { hash, JsHasher } = require('./native.node')

class Hasher {
  constructor(method) {
    this.method = method
    /**
     * @type {Uint8Array}
     */
    this.buffer = new Uint8Array(0)
  }

  update(data) {
    let dataBuffer
    if (typeof data === 'string') {
      dataBuffer = Buffer.from(data).buffer
    } else if (Buffer.isBuffer(data)) {
      dataBuffer = data
    } else {
      throw new TypeError('Expect data to be Buffer or String')
    }
    const u8Array = new Uint8Array(dataBuffer)
    const oldBuffer = this.buffer
    const oldArrLength = oldBuffer.byteLength
    this.buffer = new Uint8Array(oldArrLength + u8Array.byteLength)
    this.buffer.set(oldBuffer)
    this.buffer.set(u8Array, oldArrLength)
  }

  digest(encoding) {
    const data = Buffer.from(this.buffer.buffer)
    console.log(data)
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

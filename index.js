const { hash } = require('./native.node')

class Hasher {
  constructor(method) {
    this.method = method
    this.buffers = []
  }

  update(data) {
    if (typeof data === 'string') {
      this.buffers.push(Buffer.from(data))
    } else if (Buffer.isBuffer(data)) {
      this.buffers.push(data)
    } else {
      throw new TypeError('Expect data to be Buffer or String')
    }
  }

  digest(encoding) {
    return hash(this.method, this.buffers, encoding)
  }
}

module.exports = {
  createHash: function createHash(method) {
    return new Hasher(method)
  },
}

const { hash } = require('./native.node')

class Hasher {
  constructor(method) {
    this.method = method
    this.pending = ''
  }

  update(data) {
    if (ArrayBuffer.isView(data)) {
      this.pending += Buffer.from(data).toString('utf-8')
    } else if (typeof data === 'string') {
      this.pending += data
    }
  }

  digest(encoding) {
    return hash(this.method, this.pending, encoding)
  }
}

module.exports = {
  createHash: function createHash(method) {
    return new Hasher(method)
  },
}

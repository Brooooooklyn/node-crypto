const { Hasher } = require('./native.node')

module.exports = {
  createHash: function createHash(method) {
    return new Hasher(method)
  }
}

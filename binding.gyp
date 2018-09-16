{
  "targets": [{
    "target_name": "nodecrypto",
    "sources": [ "src/node-crypto.cc" ],
    "include_dirs": [ "<!(node -e \"require('nan')\")" ],
    'configurations': {
      'Release': {
        'msvs_settings': {
          'VCCLCompilerTool': {
            'WholeProgramOptimization': 'false'
          },
          'VCLinkerTool': {
            'LinkTimeCodeGeneration': 0
          }
        }
      }
    }
  }]
}

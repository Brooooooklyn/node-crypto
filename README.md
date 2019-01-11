# crypto-node
[![Build Status](https://travis-ci.com/Brooooooklyn/node-crypto.svg?branch=master)](https://travis-ci.com/Brooooooklyn/node-crypto)
[![CircleCI](https://circleci.com/gh/Brooooooklyn/node-crypto.svg?style=svg)](https://circleci.com/gh/Brooooooklyn/node-crypto)
[![Build status](https://ci.appveyor.com/api/projects/status/bu0lymyenwi66rlt/branch/master?svg=true)](https://ci.appveyor.com/project/Brooooooklyn/node-crypto/branch/master)

## Prebuilt Platform Support

### Operating Systems

| Linux | macOS | Windows x86 | Windows x64 |
| ----- | ----- | ----------- | ----------- |
| ✓     | ✓     | ✓           | ✓           |

### Python

| Node8   | Node10  | Node11  |
| ------- | ------- | --------|
| ✓       | ✓       | ✓       |

## Contribute

### Install dependencies
Make sure you have rust installed in your environment. follow this link: https://www.rust-lang.org/en-US/install.html to install rust toolchains.

#### Use nightly rust in development

```bash
rustup default nightly
```

#### 中国用户安装 Tips (Tips for developers in China)

如果你是中国用户, 执行
```bash
cat > $HOME/.cargo/config << EOF
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
EOF
```
然后

```bash
curl https://sh.rustup.rs -sSf | sed "s/https:\/\/static.rust-lang.org\/rustup\/dist/https:\/\/mirrors.ustc.edu.cn\/rust-static\/rustup\/dist/g" | sh
```

### Build script
```bash
yarn && yarn build
```

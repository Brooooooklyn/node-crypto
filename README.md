# crypto-node

## Dependencies
Make sure you have rust installed in your environment. follow this link: https://www.rust-lang.org/en-US/install.html to install rust toolchains.

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

## Build
```bash
yarn && yarn build
```

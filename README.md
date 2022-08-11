# 介绍

## Anchor.toml 文件介绍

```toml
# 本地网络合约地址
[programs.localnet]
gamefi = "NFTMTNtLozbwJzvLDcdp2qRSgm4tKHxo2eu4cD3nC9y"

# 测试网络合约地址
[programs.devnet]
gamefi = "NFTMTNtLozbwJzvLDcdp2qRSgm4tKHxo2eu4cD3nC9y"

# 主网络合约地址
[programs.mainnet]
gamefi = "NFTMTNtLozbwJzvLDcdp2qRSgm4tKHxo2eu4cD3nC9y"

[provider]
# 网络（localnet、devnet、mainnet）
cluster = "devnet"
# 钱包地址，json文件，此为部署合约时所用，只需要给这个钱包准备SOL就可以，非管理员地址，非合约地址，只是为了付燃料费使用，所以需要保证此钱包地址有SOL，主网大概需要10个SOL才能部署合约
wallet = "keypair/NFT99UUrXCrtYfyAEgAnKPfh9sA3LtSP2sSDLmBW8UF.json"

```

## 程序执行命令

需要先安装 Anchor，安装[地址](https://www.anchor-lang.com/docs/installation)

```bash
yarn(或者npm install) 安装依赖

npm run build 构建合约程序

npm run deploy 部署合约程序

npm run test 运行单元测试
```

## 文件目录结构

```bash
app          前端程序
keypair      钱包或者合约私钥文件
programs     合约代码
```

## 其他

首次构建合约，在 target/deploy 会自动生成两个文件（solana_marketplace-keypair.json、solana_marketplace.so），若要自定义合约地址，则把根目录的 keypair 文件夹里面的 json 文件内容，拷贝覆盖 target/deploy/solana_marketplace-keypair.json 内容，在首次部署即可为自己定义的合约程序地址；但是在部署之前，请先修改 programs/solana-marketplace/src/lib.rs 里面的 declare_id!内容为当前的合约程序地址，然后再次构建，才进行第一次部署合约；注意自定义合约地址，必须是全新未使用的地址，不能向此地址充值任何代币，包括禁止充值 SOL，必须是全新未使用

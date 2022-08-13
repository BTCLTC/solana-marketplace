# NFT Marketplace WEB

## 安装依赖

```bash
yarn install
```

## 配置

部署合约后的操作，替换掉：

```bash
cp target/idl/solana_marketplace.json app/src/solana/idl
cp target/types/solana_marketplace.ts app/src/solana/types
```

修改.env 文件里的配置

## 运行

```bash
yarn dev
```

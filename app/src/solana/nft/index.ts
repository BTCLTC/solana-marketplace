import { getParsedNftAccountsByOwner } from '@nfteyez/sol-rayz';
import { Metaplex } from '@metaplex-foundation/js';
import { AnchorProvider } from '@project-serum/anchor';
import marketNfts from '../../data/list.json';
import { PublicKey } from '@solana/web3.js';

export const loadAllNFTs = async (provider: AnchorProvider) => {
  try {
    const nfts = await getParsedNftAccountsByOwner({
      publicAddress: provider.wallet.publicKey.toBase58(),
      connection: provider.connection,
    });
    console.log('nfts', nfts);
    return nfts;
  } catch (error) {
    console.log(error);
  }
};

export const loadMarketNfts = async (provider: AnchorProvider) => {
  let marketNFTs = [];
  const mx = Metaplex.make(provider.connection);
  for (let item of marketNfts) {
    const nft = await mx
      .nfts()
      .findByMint({ mintAddress: new PublicKey(item.mint) });
    marketNFTs.push({
      data: {
        creators: nft.creators,
        name: nft.name,
        symbol: nft.symbol,
        uri: nft.uri,
        sellerFeeBasisPoints: nft.sellerFeeBasisPoints,
      },
      editionNonce: nft.editionNonce,
      isMutable: nft.isMutable,
      key: 0,
      mint: item.mint,
      primarySaleHappened: nft.primarySaleHappened,
      updateAuthority: nft.updateAuthorityAddress.toBase58(),
      seller: item.seller,
    });
  }
  console.log('marketNFTs', marketNFTs);
  return marketNFTs;
};

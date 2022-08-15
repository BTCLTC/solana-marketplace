import axios from 'axios';
import { getParsedNftAccountsByOwner } from '@nfteyez/sol-rayz';
import { AnchorProvider } from '@project-serum/anchor';
import marketNfts from '../../data/list.json';

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

export const loadMarketNfts = async () => {
  let marketNFTs = [];
  for (let item of marketNfts) {
    const res = await axios.get(
      `https://api.solscan.io/account?address=${item.mint}&cluster=devnet`
    );
    marketNFTs.push({
      ...res?.data?.data?.metadata,
      seller: item.seller,
    });
  }
  console.log('marketNFTs', marketNFTs);
  return marketNFTs;
};

import { getParsedNftAccountsByOwner } from '@nfteyez/sol-rayz';
import { AnchorProvider } from '@project-serum/anchor';

export async function loadAllNFTs(provider: AnchorProvider) {
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
}

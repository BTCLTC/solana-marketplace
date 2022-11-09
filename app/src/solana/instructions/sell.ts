import { AnchorProvider, BN, Program } from '@project-serum/anchor';
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { getParsedAccountByMint } from '@nfteyez/sol-rayz';

import {
  findConfigAddress,
  findNftMetadataAddress,
  findSellAddress,
  findVaultAddress,
} from '../utils/accounts';
import { SolanaMarketplace } from '../types/solana_marketplace';
import { SOL_DECIMALS } from '../utils/constant';

export const sell = async (
  priceStr: string,
  nftMint: string,
  provider: AnchorProvider,
  program: Program<SolanaMarketplace>
) => {
  const nftMetadata = await findNftMetadataAddress(new PublicKey(nftMint));

  const [config, _configBump] = await findConfigAddress();
  const [nftVault, _nftVaultBump] = await findVaultAddress(
    new PublicKey(nftMint)
  );
  const [sellAddress, _sellBump] = await findSellAddress(
    provider.wallet.publicKey,
    new PublicKey(nftMint)
  );
  const userNftVault = await getParsedAccountByMint({
    mintAddress: nftMint,
    connection: provider.connection,
  });

  const decimals = 10 ** SOL_DECIMALS;
  const price = new BN(Number(priceStr) * decimals);

  return await program.methods
    .sellNft(price)
    .accounts({
      seller: provider.wallet.publicKey,
      config,
      nftMint: new PublicKey(nftMint),
      nftMetadata,
      nftVault,
      userNftVault: userNftVault.pubkey,
      sell: sellAddress,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: SYSVAR_RENT_PUBKEY,
    })
    .rpc();
};

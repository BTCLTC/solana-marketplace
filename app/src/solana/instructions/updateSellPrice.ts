import { AnchorProvider, BN, Program } from '@project-serum/anchor';
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';

import { findConfigAddress, findSellAddress } from '../utils/accounts';
import { SolanaMarketplace } from '../types/solana_marketplace';

export const updateSellPrice = async (
  price: string,
  nftMint: string,
  provider: AnchorProvider,
  program: Program<SolanaMarketplace>
) => {
  const [config, _configBump] = await findConfigAddress();
  const [sell, _sellBump] = await findSellAddress(
    provider.wallet.publicKey,
    new PublicKey(nftMint)
  );

  return await program.methods
    .updateSellPrice(new BN(price))
    .accounts({
      user: provider.wallet.publicKey,
      config,
      nftMint: new PublicKey(nftMint),
      sell,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: SYSVAR_RENT_PUBKEY,
    })
    .rpc();
};

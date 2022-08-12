import { AnchorProvider, Program } from '@project-serum/anchor';
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';

import {
  findConfigAddress,
  findSellAddress,
  findVaultAddress,
} from '../utils/accounts';
import { SolanaMarketplace } from '../types/solana_marketplace';

export const closeSell = async (
  nftMint: string,
  userNftVault: string,
  provider: AnchorProvider,
  program: Program<SolanaMarketplace>
) => {
  const [config, _configBump] = await findConfigAddress();
  const [nftVault, _nftVaultBump] = await findVaultAddress(
    new PublicKey(nftMint)
  );
  const [sell, _sellBump] = await findSellAddress(
    provider.wallet.publicKey,
    new PublicKey(nftMint)
  );

  return await program.methods
    .closeSell()
    .accounts({
      user: provider.wallet.publicKey,
      config,
      nftMint: new PublicKey(nftMint),
      nftVault,
      userNftVault: new PublicKey(userNftVault),
      sell,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: SYSVAR_RENT_PUBKEY,
    })
    .rpc();
};

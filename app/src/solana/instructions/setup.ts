import { AnchorProvider, BN, Program } from '@project-serum/anchor';
import { SystemProgram, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { findConfigAddress } from '../utils/accounts';
import { SolanaMarketplace } from '../types/solana_marketplace';
import { feeAccountPublicKey } from '../utils/constant';

export const setup = async (
  provider: AnchorProvider,
  program: Program<SolanaMarketplace>
) => {
  const [config, bump] = await findConfigAddress();

  return await program.methods
    .setup(bump, new BN(0)) // 25: 0.25% feeRate; 0-10000
    .accounts({
      owner: provider.wallet.publicKey,
      feeAccount: feeAccountPublicKey,
      config,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: SYSVAR_RENT_PUBKEY,
    })
    .rpc();
};

import { AnchorProvider, Program } from '@project-serum/anchor';
import { PublicKey, SystemProgram } from '@solana/web3.js';

import { findConfigAddress } from '../utils/accounts';
import { SolanaMarketplace } from '../types/solana_marketplace';

export const updateFeeAccount = async (
  feeAccount: string,
  provider: AnchorProvider,
  program: Program<SolanaMarketplace>
) => {
  const [config, _bump] = await findConfigAddress();

  return await program.methods
    .updateFeeAccount()
    .accounts({
      owner: provider.wallet.publicKey,
      feeAccount: new PublicKey(feeAccount),
      config,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
};

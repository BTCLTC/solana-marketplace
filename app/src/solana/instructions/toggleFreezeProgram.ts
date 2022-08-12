import { AnchorProvider, Program } from '@project-serum/anchor';

import { findConfigAddress } from '../utils/accounts';
import { SolanaMarketplace } from '../types/solana_marketplace';

export const toggleFreezeProgram = async (
  provider: AnchorProvider,
  program: Program<SolanaMarketplace>
) => {
  const [config, _bump] = await findConfigAddress();

  return await program.methods
    .toggleFreezeProgram()
    .accounts({
      owner: provider.wallet.publicKey,
      config,
    })
    .rpc();
};

import { AnchorProvider, Program } from '@project-serum/anchor';

import { findConfigAddress } from '../utils/accounts';
import { SolanaMarketplace } from '../types/solana_marketplace';

export const toggleFreeze = async (
  provider: AnchorProvider,
  program: Program<SolanaMarketplace>
) => {
  const [config, _bump] = await findConfigAddress();

  return await program.methods
    .toggleFreeze()
    .accounts({
      owner: provider.wallet.publicKey,
      config,
    })
    .rpc();
};

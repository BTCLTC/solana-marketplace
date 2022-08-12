import { AnchorProvider, BN, Program } from '@project-serum/anchor';

import { findConfigAddress } from '../utils/accounts';
import { SolanaMarketplace } from '../types/solana_marketplace';

export const updateFeeRate = async (
  rate: string,
  provider: AnchorProvider,
  program: Program<SolanaMarketplace>
) => {
  const [config, _bump] = await findConfigAddress();

  return await program.methods
    .updateFeeRate(new BN(rate))
    .accounts({
      owner: provider.wallet.publicKey,
      config,
    })
    .rpc();
};

import { AnchorProvider, Program } from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';

import { findConfigAddress } from '../utils/accounts';
import { SolanaMarketplace } from '../types/solana_marketplace';

export const updateOwner = async (
  owner: string,
  provider: AnchorProvider,
  program: Program<SolanaMarketplace>
) => {
  const [config, _bump] = await findConfigAddress();

  return await program.methods
    .updateOwner()
    .accounts({
      owner: provider.wallet.publicKey,
      newOwner: new PublicKey(owner),
      config,
    })
    .rpc();
};

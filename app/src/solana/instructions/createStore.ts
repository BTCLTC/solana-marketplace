import { AnchorProvider, Program } from '@project-serum/anchor';
import { SystemProgram } from '@solana/web3.js';
import { storeKeypair, storePublicKey } from '../../utils/constant';
import { SolanaMarketplace } from '../../types/solana_marketplace';

export const createStore = async (
  provider: AnchorProvider,
  program: Program<SolanaMarketplace>
) => {
  return await program.methods
    .createStore('<name>', '<description>')
    .accounts({
      admin: provider.wallet.publicKey,
      store: storePublicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([storeKeypair])
    .rpc();
};

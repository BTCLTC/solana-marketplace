import { AnchorProvider, Program } from '@project-serum/anchor';
import { SystemProgram } from '@solana/web3.js';
import {
  marketKeypair,
  marketPublicKey,
  storePublicKey,
} from '../../utils/constant';
import { SolanaMarketplace } from '../types/solana_marketplace';

export const initSellingResource = async (
  provider: AnchorProvider,
  program: Program<SolanaMarketplace>
) => {
  return await program.methods
    .initSellingResource('<name>', '<description>')
    .accounts({
      market: marketPublicKey,
      store: storePublicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([marketKeypair])
    .rpc();
};

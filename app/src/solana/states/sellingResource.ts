import { Program } from '@project-serum/anchor';
import { storePublicKey } from '../../utils/constant';

import { SolanaMarketplace } from '../types/solana_marketplace';

export const getSellingResource = async (
  program: Program<SolanaMarketplace>
) => {
  console.log(storePublicKey.toBase58());
  return await program.account.sellingResource.fetch(storePublicKey);
};

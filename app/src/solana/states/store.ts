import { Program } from '@project-serum/anchor';
import { storePublicKey } from '../../utils/constant';

import { SolanaMarketplace } from '../../types/solana_marketplace';

export const getStore = async (program: Program<SolanaMarketplace>) => {
  return await program.account.store.fetch(storePublicKey);
};

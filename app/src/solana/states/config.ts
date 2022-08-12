import { Program } from '@project-serum/anchor';
import { findConfigAddress } from '../utils/accounts';

import { SolanaMarketplace } from '../types/solana_marketplace';

export const getConfig = async (program: Program<SolanaMarketplace>) => {
  const [config, _bump] = await findConfigAddress();
  return await program.account.config.fetch(config);
};

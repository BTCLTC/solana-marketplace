import { Program } from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';

import { findSellAddress } from '../utils/accounts';
import { SolanaMarketplace } from '../types/solana_marketplace';

export const getSell = async (
  seller: string,
  mint: string,
  program: Program<SolanaMarketplace>
) => {
  const [sell, _bump] = await findSellAddress(
    new PublicKey(seller),
    new PublicKey(mint)
  );
  return await program.account.sell.fetch(sell);
};

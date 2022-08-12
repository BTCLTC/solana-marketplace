import { Program } from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';

import { findSellAddress } from '../utils/accounts';
import { SolanaMarketplace } from '../types/solana_marketplace';
import { feeAccountPublicKey } from '../utils/constant';

export const getSell = async (
  mint: string,
  program: Program<SolanaMarketplace>
) => {
  const [sell, _bump] = await findSellAddress(
    feeAccountPublicKey,
    new PublicKey(mint)
  );
  return await program.account.sell.fetch(sell);
};

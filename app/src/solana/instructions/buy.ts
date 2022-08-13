import { AnchorProvider, Program } from '@project-serum/anchor';
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';

import {
  findConfigAddress,
  findSellAddress,
  findVaultAddress,
} from '../utils/accounts';
import { SolanaMarketplace } from '../types/solana_marketplace';
import { feeAccountPublicKey } from '../utils/constant';

export const buy = async (
  seller: string,
  nftMint: string,
  buyerNftVault: string,
  provider: AnchorProvider,
  program: Program<SolanaMarketplace>
) => {
  const [config, _configBump] = await findConfigAddress();
  const [nftVault, _nftVaultBump] = await findVaultAddress(
    new PublicKey(nftMint)
  );
  const [sell, _sellBump] = await findSellAddress(
    provider.wallet.publicKey,
    new PublicKey(nftMint)
  );

  const feeAccountValue = localStorage.getItem('fee_account');
  const feeAccount = feeAccountValue
    ? new PublicKey(feeAccountValue)
    : feeAccountPublicKey;

  return await program.methods
    .buy()
    .accounts({
      buyer: provider.wallet.publicKey,
      seller: new PublicKey(seller),
      config,
      nftMint: new PublicKey(nftMint),
      nftVault,
      buyerNftVault: new PublicKey(buyerNftVault),
      feeAccount,
      sell,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: SYSVAR_RENT_PUBKEY,
    })
    .rpc();
};
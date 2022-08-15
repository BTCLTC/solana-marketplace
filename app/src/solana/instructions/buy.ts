import { AnchorProvider, Program } from '@project-serum/anchor';
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import { getAssociatedTokenAddress, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { ASSOCIATED_PROGRAM_ID } from '@project-serum/anchor/dist/cjs/utils/token';

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
  provider: AnchorProvider,
  program: Program<SolanaMarketplace>
) => {
  const [config, _configBump] = await findConfigAddress();
  const [nftVault, _nftVaultBump] = await findVaultAddress(
    new PublicKey(nftMint)
  );
  const [sell, _sellBump] = await findSellAddress(
    new PublicKey(seller),
    new PublicKey(nftMint)
  );

  const buyerNftVault = await getAssociatedTokenAddress(
    new PublicKey(nftMint),
    provider.wallet.publicKey
  );

  const feeAccountValue = localStorage.getItem('fee_account');
  const feeAccount = feeAccountValue
    ? new PublicKey(feeAccountValue)
    : feeAccountPublicKey;

  return await program.methods
    .buyNft()
    .accounts({
      buyer: provider.wallet.publicKey,
      // buyerTokenAccount,
      seller: new PublicKey(seller),
      config,
      nftMint: new PublicKey(nftMint),
      nftVault,
      buyerNftVault: buyerNftVault,
      feeAccount,
      sell,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
      rent: SYSVAR_RENT_PUBKEY,
    })
    .rpc();
};

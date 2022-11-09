import { AnchorProvider, Program } from '@project-serum/anchor';
import { AccountMeta, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import { getAssociatedTokenAddress, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { ASSOCIATED_PROGRAM_ID } from '@project-serum/anchor/dist/cjs/utils/token';
import { Metaplex } from '@metaplex-foundation/js';

import {
  findConfigAddress,
  findNftMetadataAddress,
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
  const nftMetadata = await findNftMetadataAddress(new PublicKey(nftMint));

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

  const mx = Metaplex.make(provider.connection);
  const nft = await mx
    .nfts()
    .findByMint({ mintAddress: new PublicKey(nftMint) });

  const creators = nft.creators.map((item, index: number) => {
    return {
      pubkey: item.address,
      isSigner: false,
      isWritable: true,
    };
  });

  return await program.methods
    .buyNft()
    .accounts({
      buyer: provider.wallet.publicKey,
      // buyerTokenAccount,
      seller: new PublicKey(seller),
      config,
      nftMint: new PublicKey(nftMint),
      nftMetadata,
      nftVault,
      buyerNftVault: buyerNftVault,
      feeAccount,
      sell,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
      rent: SYSVAR_RENT_PUBKEY,
    })
    .remainingAccounts(creators)
    .rpc();
};

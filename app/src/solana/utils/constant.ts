import { clusterApiUrl, Keypair, PublicKey } from '@solana/web3.js';
import bs58 from 'bs58';

export const CONFIG_PDA_SEED = 'config';
export const NFT_VAULT_PDA_SEED = 'nft_vault';
export const SELL_PDA_SEED = 'sell';

export const programId = process.env.NEXT_PUBLIC_PROGRAM_ID || '';

export const LOCALHOST = 'http://127.0.0.1:8899';

export const DEVNET = clusterApiUrl('devnet');

export const PROGRAM_ID = new PublicKey(programId);

export const SOL_DECIMALS = 9;

export const connectionURL =
  process.env.NEXT_PUBLIC_NETWORK == 'devnet' ? DEVNET : LOCALHOST;

export const feeAccountPublicKey = new PublicKey(
  process.env.NEXT_PUBLIC_FEE_ACCOUNT_PUBLIC_KEY || ''
);

// export const storeKeypair = Keypair.fromSecretKey(
//   Uint8Array.from(bs58.decode(process.env.NEXT_PUBLIC_STORE_PRIVATE_KEY || ''))
// );

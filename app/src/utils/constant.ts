import { clusterApiUrl, Keypair, PublicKey } from '@solana/web3.js';
import bs58 from 'bs58';

export const VAULT_OWNER_PREFIX = 'mt_vault';
export const HISTORY_PREFIX = 'history';
export const PAYOUT_TICKET_PREFIX = 'payout_ticket';
export const HOLDER_PREFIX = 'holder';
export const PRIMARY_METADATA_CREATORS_PREFIX = 'primary_creators';

export const programId = process.env.NEXT_PUBLIC_PROGRAM_ID || '';

export const LOCALHOST = 'http://127.0.0.1:8899';

export const DEVNET = clusterApiUrl('devnet');

export const PROGRAM_ID = new PublicKey(programId);

export const connectionURL =
  process.env.NEXT_PUBLIC_NETWORK == 'devnet' ? DEVNET : LOCALHOST;

export const storeKeypair = Keypair.fromSecretKey(
  Uint8Array.from(bs58.decode(process.env.NEXT_PUBLIC_STORE_PRIVATE_KEY || ''))
);

export const storePublicKey = storeKeypair.publicKey;

export const marketKeypair = Keypair.fromSecretKey(
  Uint8Array.from(bs58.decode(process.env.NEXT_PUBLIC_MARKET_PRIVATE_KEY || ''))
);

export const marketPublicKey = marketKeypair.publicKey;

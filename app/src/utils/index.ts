import { Connection, PublicKey } from '@solana/web3.js';

export { createAndSignTransaction } from './createAndSignTransaction';
export { programId, PROGRAM_ID, connectionURL } from './constant';

import { StringPublicKey, TokenAccount } from '@metaplex-foundation/mpl-core';
import { deprecated } from '@metaplex-foundation/mpl-token-metadata';
import {
  HISTORY_PREFIX,
  HOLDER_PREFIX,
  PAYOUT_TICKET_PREFIX,
  PRIMARY_METADATA_CREATORS_PREFIX,
  PROGRAM_ID,
  VAULT_OWNER_PREFIX,
} from './constant';

export const findVaultOwnerAddress = (
  mint: PublicKey,
  store: PublicKey
): Promise<[PublicKey, number]> =>
  PublicKey.findProgramAddress(
    [Buffer.from(VAULT_OWNER_PREFIX), mint.toBuffer(), store.toBuffer()],
    PROGRAM_ID
  );

export const findTreasuryOwnerAddress = (
  treasuryMint: PublicKey,
  sellingResource: PublicKey
): Promise<[PublicKey, number]> =>
  PublicKey.findProgramAddress(
    [
      Buffer.from(HOLDER_PREFIX),
      treasuryMint.toBuffer(),
      sellingResource.toBuffer(),
    ],
    PROGRAM_ID
  );

export const findTradeHistoryAddress = (
  wallet: PublicKey,
  market: PublicKey
): Promise<[PublicKey, number]> =>
  PublicKey.findProgramAddress(
    [Buffer.from(HISTORY_PREFIX), wallet.toBuffer(), market.toBuffer()],
    PROGRAM_ID
  );

export const findPayoutTicketAddress = (
  market: PublicKey,
  funder: PublicKey
): Promise<[PublicKey, number]> => {
  return PublicKey.findProgramAddress(
    [Buffer.from(PAYOUT_TICKET_PREFIX), market.toBuffer(), funder.toBuffer()],
    PROGRAM_ID
  );
};

export const findPrimaryMetadataCreatorsAddress = (
  metadata: PublicKey
): Promise<[PublicKey, number]> =>
  PublicKey.findProgramAddress(
    [Buffer.from(PRIMARY_METADATA_CREATORS_PREFIX), metadata.toBuffer()],
    PROGRAM_ID
  );

export const validateMembershipToken = async (
  connection: Connection,
  me: StringPublicKey,
  ta: TokenAccount
) => {
  const edition = (await deprecated.Metadata.getEdition(
    connection,
    ta?.data?.mint!
  )) as deprecated.Edition;
  return edition?.data?.parent === me;
};

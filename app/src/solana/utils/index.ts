import { PublicKey } from '@solana/web3.js';
import { PROGRAM_ID } from './constant';

export { programId, PROGRAM_ID, connectionURL } from './constant';

export const findProgramAddress = (
  seeds: (Buffer | Uint8Array)[]
): Promise<[PublicKey, number]> =>
  PublicKey.findProgramAddress(seeds, PROGRAM_ID);

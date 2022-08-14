import { AnchorProvider, Program } from '@project-serum/anchor';
import { atom } from 'recoil';
import { SolanaMarketplace } from '../solana/types/solana_marketplace';

interface IApp {
  provider?: AnchorProvider;
  program?: Program<SolanaMarketplace>;
}

export const appState = atom<IApp>({
  key: 'appState',
  default: {
    provider: undefined,
    program: undefined,
  },
});

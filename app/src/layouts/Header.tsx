import { FC, useMemo, useEffect } from 'react';
import {
  WalletModalProvider,
  WalletDisconnectButton,
  WalletMultiButton,
} from '@solana/wallet-adapter-react-ui';
import { AnchorProvider, Program } from '@project-serum/anchor';
import { Connection } from '@solana/web3.js';
import { useAnchorWallet } from '@solana/wallet-adapter-react';
import { ToastContainer } from 'react-toastify';
import { useSetRecoilState } from 'recoil';

import { SolanaMarketplace } from '../solana/types/solana_marketplace';
import idl from '../solana/idl/solana_marketplace.json';
import { programId, connectionURL } from '../solana/utils';
import Link from '../components/ActiveLink';
import { appState } from '../stores';

const Header: FC = () => {
  const setApp = useSetRecoilState(appState);

  const wallet = useAnchorWallet();

  const provider = useMemo(() => {
    if (wallet) {
      const connection = new Connection(connectionURL);
      return new AnchorProvider(connection, wallet, {});
    }
    return undefined;
  }, [wallet]);

  const program = useMemo(() => {
    if (provider) {
      return new Program<SolanaMarketplace>(
        idl as unknown as SolanaMarketplace,
        programId,
        provider
      );
    }
    return undefined;
  }, [provider]);

  useEffect(() => {
    setApp((oldData) => {
      return {
        ...oldData,
        provider,
      };
    });
  }, [provider]);

  useEffect(() => {
    setApp((oldData) => {
      return {
        ...oldData,
        program,
      };
    });
  }, [program]);

  return (
    <header className="container mx-auto flex justify-between items-center">
      <h1 className="text-gray-700 font-bold flex-shrink-0">NFT Marketplace</h1>
      <nav className="flex items-center justify-between w-[240px] text-gray-500">
        <Link href="/" activeClassName="text-blue-900 font-bold">
          <a>Home</a>
        </Link>
        <Link href="/buy" activeClassName="text-blue-900 font-bold">
          <a>Buy NFT</a>
        </Link>
        <Link href="/sell" activeClassName="text-blue-900 font-bold">
          <a>Sell NFT</a>
        </Link>
      </nav>
      <WalletModalProvider>
        <div className="flex">
          <ToastContainer />
          <WalletMultiButton />
          <div className="wallet-space"></div>
          <WalletDisconnectButton />
        </div>
      </WalletModalProvider>
    </header>
  );
};

export default Header;

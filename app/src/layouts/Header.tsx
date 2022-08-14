import { FC } from 'react';
import {
  WalletModalProvider,
  WalletDisconnectButton,
  WalletMultiButton,
} from '@solana/wallet-adapter-react-ui';
import { ToastContainer } from 'react-toastify';

const Header: FC = () => {
  return (
    <header className="container mx-auto flex justify-between items-center">
      <h1 className="text-gray-700 font-bold flex-shrink-0">NFT Marketplace</h1>
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

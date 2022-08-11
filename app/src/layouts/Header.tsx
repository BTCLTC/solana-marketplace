import { FC } from 'react';
import {
  WalletModalProvider,
  WalletDisconnectButton,
  WalletMultiButton,
} from '@solana/wallet-adapter-react-ui';

const Header: FC = () => {
  return (
    <header className="container mx-auto flex justify-between items-center">
      <h1 className="text-gray-700 font-bold flex-shrink-0">NFT Marketplace</h1>
      <WalletModalProvider>
        <div className="flex">
          <WalletMultiButton />
          <div className="wallet-space"></div>
          <WalletDisconnectButton />
        </div>
      </WalletModalProvider>
    </header>
  );
};

export default Header;

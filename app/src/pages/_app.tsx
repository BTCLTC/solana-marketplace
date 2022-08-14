import type { AppProps } from 'next/app';
import { useMemo } from 'react';
import { RecoilRoot } from 'recoil';
import {
  ConnectionProvider,
  WalletProvider,
} from '@solana/wallet-adapter-react';
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base';
import {
  GlowWalletAdapter,
  PhantomWalletAdapter,
  SlopeWalletAdapter,
  SolflareWalletAdapter,
  TorusWalletAdapter,
} from '@solana/wallet-adapter-wallets';
import { clusterApiUrl } from '@solana/web3.js';

import 'windi.css';
import 'react-toastify/dist/ReactToastify.css';
import '../styles/base/globals.scss';
import '../styles/layouts/Header.scss';
import '../styles/layouts/Footer.scss';
import '@solana/wallet-adapter-react-ui/styles.css';

import Layout from '../layouts/Layout';

function MyApp({ Component, pageProps }: AppProps) {
  // The network can be set to 'devnet', 'testnet', or 'mainnet-beta'.
  const network = WalletAdapterNetwork.Devnet;

  // You can also provide a custom RPC endpoint.
  const endpoint = useMemo(() => clusterApiUrl(network), [network]);

  const wallets = useMemo(
    () => [
      new PhantomWalletAdapter(),
      new GlowWalletAdapter(),
      new SlopeWalletAdapter(),
      new SolflareWalletAdapter({ network }),
      new TorusWalletAdapter(),
    ],
    [network]
  );

  return (
    <RecoilRoot>
      <ConnectionProvider endpoint={endpoint}>
        <WalletProvider wallets={wallets} autoConnect>
          <Layout>
            <Component {...pageProps} />
          </Layout>
        </WalletProvider>
      </ConnectionProvider>
    </RecoilRoot>
  );
}

export default MyApp;

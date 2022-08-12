import type { NextPage } from 'next';
import { useEffect, useMemo, useCallback, useState } from 'react';
import { m } from 'framer-motion';
import { AnchorProvider, Program } from '@project-serum/anchor';
import { Connection } from '@solana/web3.js';
import { useAnchorWallet } from '@solana/wallet-adapter-react';

import { SolanaMarketplace } from '../solana/types/solana_marketplace';
import idl from '../idl/solana_marketplace.json';
import { programId, connectionURL } from '../solana/utils';
import { getStore } from '../solana/states';
import { createStore } from '../solana/instructions';

const Home: NextPage = () => {
  const [loading, setLoading] = useState(false);

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
    return null;
  }, [provider]);

  useEffect(() => {
    if (program) {
      getStore(program)
        .then((data) => {
          console.log(data);
          console.log(`admin pubkey: ${data.admin.toBase58()}`);
        })
        .catch((error) => {
          console.error(error);
        });
    }
  }, [program]);

  const handleClick = useCallback(
    async (fun: string) => {
      if (!provider || !program) {
        alert('请先连接钱包，并切换到devnet网络');
        return;
      }
      setLoading(true);
      if (fun == 'createStore') {
        const tx = await createStore(provider, program).catch((error) => {
          console.log(error.logs);
          setLoading(false);
        });
        console.log(tx);
      }
      setLoading(false);
    },
    [provider, program]
  );

  return (
    <m.div>
      <button
        className={`btn btn-active btn-primary normal-case ${
          loading ? 'loading' : ''
        }`}
        onClick={() => handleClick('createStore')}
        disabled={loading}
      >
        createStore
      </button>
    </m.div>
  );
};

export default Home;

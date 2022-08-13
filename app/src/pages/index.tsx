import type { NextPage } from 'next';
import { useEffect, useMemo, useCallback, useState } from 'react';
import { AnchorProvider, Program } from '@project-serum/anchor';
import { Connection } from '@solana/web3.js';
import { useAnchorWallet } from '@solana/wallet-adapter-react';
import { ToastContainer, toast } from 'react-toastify';

import { SolanaMarketplace } from '../solana/types/solana_marketplace';
import idl from '../solana/idl/solana_marketplace.json';
import { programId, connectionURL } from '../solana/utils';
import { getConfig } from '../solana/states';
import { setup, toggleFreezeProgram } from '../solana/instructions';

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
      getConfig(program)
        .then((data) => {
          console.log(data);
        })
        .catch((error) => {
          console.error(error);
        });
    }
  }, [program]);

  const handleClick = useCallback(
    async (fun: string) => {
      if (!provider || !program) {
        toast.error('请先连接钱包，并切换到devnet网络');
        return;
      }
      setLoading(true);
      if (fun == 'setup') {
        const tx = await setup(provider, program).catch((error) => {
          console.log(error.logs);
          setLoading(false);
        });
        console.log(tx);
      } else if (fun == 'toggleFreezeProgram') {
        const tx = await toggleFreezeProgram(provider, program).catch(
          (error) => {
            console.log(error.logs);
            setLoading(false);
          }
        );
        console.log(tx);
      }
      setLoading(false);
    },
    [provider, program]
  );

  return (
    <main>
      <ToastContainer />
      <section className="py-6 px-12 border-1 rounded-xl text-gray-900">
        <p className="text-center mb-4">admin</p>
        <button
          className={`btn btn-active btn-primary normal-case mr-4 ${
            loading ? 'loading' : ''
          }`}
          onClick={() => handleClick('setup')}
          disabled={loading}
        >
          setup
        </button>
        <button
          className={`btn btn-active btn-primary normal-case ${
            loading ? 'loading' : ''
          }`}
          onClick={() => handleClick('toggleFreezeProgram')}
          disabled={loading}
        >
          toggleFreezeProgram
        </button>
      </section>
    </main>
  );
};

export default Home;

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
import { setup, toggleFreeze } from '../solana/instructions';
import { formatTx } from '../utils';

const Home: NextPage = () => {
  const [loading, setLoading] = useState(false);
  const [config, setConfig] = useState<any>(null);

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

  const getConfigInfo = useCallback(async () => {
    if (program) {
      getConfig(program)
        .then((data) => {
          setConfig(data);
        })
        .catch((error) => {
          console.error(error);
        });
    }
  }, [program]);

  useEffect(() => {
    getConfigInfo();
  }, [program]);

  const handleClick = useCallback(
    async (fun: string) => {
      if (!provider || !program) {
        toast.error('请先连接钱包，并切换到devnet网络');
        return;
      }
      setLoading(true);
      let tx;
      if (fun == 'setup') {
        tx = await setup(provider, program).catch((error) => {
          console.log(error.logs);
          setLoading(false);
        });
      } else if (fun == 'toggleFreeze') {
        tx = await toggleFreeze(provider, program).catch((error) => {
          console.log(error.logs);
          setLoading(false);
        });
      }

      if (tx) {
        console.log(`tx: ${tx}`);
        toast.success(
          <div>
            Tx:{' '}
            <a
              href={`https://solscan.io/tx/${tx}?cluster=devnet`}
              target="_blank"
              rel="noopener noreferrer"
            >
              {formatTx(tx)}
            </a>
          </div>
        );
        await getConfigInfo();
      }

      setLoading(false);
    },
    [getConfigInfo, provider, program]
  );

  return (
    <main>
      <ToastContainer />
      <section className="py-6 px-12 border-1 rounded-xl text-gray-900">
        <h3 className="text-center mb-4 text-xl font-bold">admin operation</h3>
        <button
          className={`btn btn-active btn-primary normal-case mr-4 ${
            loading ? 'loading' : ''
          }`}
          onClick={() => handleClick('setup')}
          disabled={loading || config?.owner}
        >
          setup
        </button>
        <button
          className={`btn btn-active btn-primary normal-case ${
            loading ? 'loading' : ''
          }`}
          onClick={() => handleClick('toggleFreeze')}
          disabled={loading}
        >
          toggleFreeze
        </button>
      </section>
      <section className="py-6 px-12 mt-4 border-1 rounded-xl text-gray-900">
        <h3 className="text-center mb-4 text-xl font-bold">contract config</h3>
        <div>
          <p>owner: {config?.owner.toBase58()}</p>
        </div>
        <div>
          <p>feeAccount: {config?.feeAccount.toBase58()}</p>
        </div>
        <div>
          <p>feeRate: {config?.feeRate.toString()}</p>
        </div>
        <div>
          <p>freeze: {config?.freeze.toString()}</p>
        </div>
        <div>
          <p>orderId: {config?.orderId.toString()}</p>
        </div>
        <div>
          <p>orderCount: {config?.orderCount.toString()}</p>
        </div>
        <div>
          <p>bump: {config?.bump}</p>
        </div>
      </section>
    </main>
  );
};

export default Home;

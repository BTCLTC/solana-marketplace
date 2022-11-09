import type { NextPage } from 'next';
import { useEffect, useMemo, useCallback, useState } from 'react';
import { toast } from 'react-toastify';
import { useRecoilValue } from 'recoil';
import { m } from 'framer-motion';

import { getConfig } from '../solana/states';
import {
  setup,
  toggleFreeze,
  updateFeeAccount,
  updateFeeRate,
  updateOwner,
} from '../solana/instructions';
import { fade, formatTx } from '../utils';
import { appState } from '../stores';

const Home: NextPage = () => {
  const { provider, program } = useRecoilValue(appState);

  const [input, setInput] = useState('');
  const [loading, setLoading] = useState(false);
  const [config, setConfig] = useState<any>(null);

  const isAddress = useMemo(() => {
    return input.length == 43 || input.length == 44;
  }, [input]);

  const isRate = useMemo(() => {
    return (
      input.length &&
      input.trim() != '' &&
      Number(input) >= 0 &&
      Number(input) <= 10000
    );
  }, [input]);

  const rateValue = useMemo(() => {
    if (config?.feeRate) {
      const rate = config.feeRate.toNumber() / 100;
      return `(${rate}%)`;
    }
    return '';
  }, [config?.feeRate]);

  const getConfigInfo = useCallback(async () => {
    if (program) {
      getConfig(program)
        .then((data) => {
          setConfig(data);
          localStorage.setItem('fee_account', data.feeAccount.toBase58());
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
      try {
        if (fun == 'setup') {
          tx = await setup(provider, program);
        } else if (fun == 'toggleFreeze') {
          tx = await toggleFreeze(provider, program);
        } else if (fun == 'updateOwner') {
          tx = await updateOwner(input, provider, program);
        } else if (fun == 'updateFeeAccount') {
          tx = await updateFeeAccount(input, provider, program);
        } else if (fun == 'updateFeeRate') {
          tx = await updateFeeRate(input, provider, program);
        }
      } catch (error: any) {
        console.log(error.logs);
        setLoading(false);
      }

      if (tx) {
        console.log(`tx: ${tx}`);
        toast.success(
          <div className="text-sm">
            Tx:{' '}
            <a
              className="text-blue-500 cursor-pointer"
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
    [provider, program, input, getConfigInfo]
  );

  return (
    <m.main variants={fade} className="container">
      <section className="container flex items-center justify-between">
        <section className="flex-shrink-0 py-6 px-12 border-1 rounded-xl text-gray-900 w-[600px]">
          <h3 className="text-center mb-4 text-xl font-bold">
            admin operation
          </h3>
          <div className="mb-4">
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
          </div>
          <div className="mb-4">
            <button
              className={`btn btn-active btn-primary normal-case mr-4 ${
                loading ? 'loading' : ''
              }`}
              onClick={() => handleClick('updateOwner')}
              disabled={loading || !isAddress}
            >
              updateOwner
            </button>
            <button
              className={`btn btn-active btn-primary normal-case mr-4 ${
                loading ? 'loading' : ''
              }`}
              onClick={() => handleClick('updateFeeAccount')}
              disabled={loading || !isAddress}
            >
              updateFeeAccount
            </button>
            <button
              className={`btn btn-active btn-primary normal-case ${
                loading ? 'loading' : ''
              }`}
              onClick={() => handleClick('updateFeeRate')}
              disabled={loading || !isRate}
            >
              updateFeeRate
            </button>
          </div>
          <div>
            <input
              type="text"
              placeholder="Input address or fee rate (0 - 10000)"
              className="input input-bordered input-info w-full"
              onChange={(e) => setInput(e.target.value)}
            />
          </div>
        </section>
        <section className="py-6 px-12 mt-4 border-1 rounded-xl text-gray-900">
          <h3 className="text-center mb-4 text-xl font-bold">
            contract config
          </h3>
          <div>
            <p>owner: {config?.owner.toBase58()}</p>
          </div>
          <div>
            <p>feeAccount: {config?.feeAccount.toBase58()}</p>
          </div>
          <div>
            <p>
              feeRate: {config?.feeRate.toString()} {rateValue}
            </p>
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
      </section>
    </m.main>
  );
};

export default Home;

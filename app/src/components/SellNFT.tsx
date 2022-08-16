import Image from 'next/image';
import axios from 'axios';
import { toast } from 'react-toastify';
import { useState, useCallback } from 'react';
import { useMount } from 'ahooks';
import { useRecoilValue } from 'recoil';

import { appState } from '../stores';
import { sell } from '../solana/instructions';
import { INFT } from '../interface';
import { formatTx } from '../utils';
import placeholder from '../images/placeholder.jpg';

type Props = {
  info: INFT;
  refreshNftList: () => void;
};

const SellNFT = ({ info, refreshNftList }: Props) => {
  const { provider, program } = useRecoilValue(appState);

  const [loading, setLoading] = useState(false);
  const [srcUrl, setSrcUrl] = useState(placeholder);
  const [price, setPrice] = useState('');

  useMount(() => {
    axios.get(info.data.uri).then((res) => {
      if (res?.data?.image) {
        setSrcUrl(res.data.image);
      }
    });
  });

  const handleSell = useCallback(async () => {
    if (!provider || !program) {
      toast.error('请先连接钱包，并切换到devnet网络');
      return;
    }
    setLoading(true);
    const tx = await sell(price, info.mint, provider, program).catch(
      (error: any) => {
        console.log(error);
        console.log(error.logs);
        setLoading(false);
      }
    );
    setLoading(false);
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
    }
    setTimeout(() => {
      refreshNftList();
    }, 18000);
  }, [info.mint, price, program, provider, refreshNftList]);

  return (
    <div className="card card-compact w-96 bg-base-100 shadow-xl my-6">
      <figure className="w-[384px] h-[384px]">
        <Image src={srcUrl} width="384" height="384" alt="NFT" />
      </figure>
      <div className="card-body">
        <h2 className="card-title">
          {info.data.name}
          <div className="badge badge-secondary">{info.data.symbol}</div>
        </h2>
        <p>
          mint:{' '}
          <a
            className="text-xs text-blue-500 cursor-pointer"
            href={`https://solscan.io/token/${info.mint}?cluster=devnet`}
            target="_blank"
            rel="noopener noreferrer"
          >
            {info.mint}
          </a>
        </p>
        <div>
          <input
            type="text"
            placeholder="Input SOL amount"
            className="input input-bordered input-info w-full my-4"
            onChange={(e) => setPrice(e.target.value)}
          />
        </div>
        <div className="card-actions justify-center">
          <button
            className={`btn btn-active btn-error text-white ${
              loading ? 'loading' : ''
            }`}
            disabled={loading || !price}
            onClick={() => handleSell()}
          >
            Sell
          </button>
        </div>
      </div>
    </div>
  );
};

export default SellNFT;

import Image from 'next/image';
import axios from 'axios';
import { toast } from 'react-toastify';
import { useState, useCallback, useMemo, useEffect } from 'react';
import { useMount } from 'ahooks';
import { useRecoilValue } from 'recoil';

import { SOL_DECIMALS } from '../solana/utils/constant';
import { appState } from '../stores';
import { buy, closeSell, updateSellPrice } from '../solana/instructions';
import { INFT } from '../interface';
import { formatTx } from '../utils';
import { getSell } from '../solana/states/sell';
import placeholder from '../images/placeholder.jpg';

type Props = {
  info: INFT;
  refreshNftList: () => void;
};

const BuyNFT = ({ info, refreshNftList }: Props) => {
  const { provider, program } = useRecoilValue(appState);

  const [loading, setLoading] = useState(false);
  const [srcUrl, setSrcUrl] = useState(placeholder);
  const [price, setPrice] = useState('');
  const [infos, setInfos] = useState<any>({});
  const [isSold, setSold] = useState(false);

  useMount(() => {
    axios.get(info.data.uri).then((res) => {
      if (res?.data?.image) {
        setSrcUrl(res.data.image);
      }
    });
  });

  useEffect(() => {
    getNftSell();
  }, [program]);

  const getNftSell = useCallback(() => {
    if (program) {
      getSell(info.seller || '', info.mint, program)
        .then((res) => {
          setInfos(res);
        })
        .catch((error) => {
          setSold(true);
          console.error(error);
        });
    }
  }, [info.mint, info.seller, program]);

  const isSelf = useMemo(() => {
    const pubKey = provider?.wallet.publicKey.toBase58();
    return pubKey === info.seller;
  }, [provider, info]);

  const getSellPrice = useMemo(() => {
    const decimals = 10 ** SOL_DECIMALS;
    return infos.price ? infos.price.toString() / decimals : 0;
  }, [infos]);

  const handleBuy = useCallback(async () => {
    if (!provider || !program) {
      toast.error('请先连接钱包，并切换到devnet网络');
      return;
    }
    setLoading(true);
    const tx = await buy(info.seller!, info.mint, provider, program).catch(
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
      setSold(true);
    }
    refreshNftList();
  }, [info.mint, program, provider, info.seller, refreshNftList]);

  const handleCancel = useCallback(async () => {
    if (!provider || !program) {
      toast.error('请先连接钱包，并切换到devnet网络');
      return;
    }
    setLoading(true);
    const tx = await closeSell(info.mint, provider, program).catch(
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
    refreshNftList();
  }, [program, provider, info.mint, refreshNftList]);

  const handleUpdate = useCallback(async () => {
    if (!provider || !program) {
      toast.error('请先连接钱包，并切换到devnet网络');
      return;
    }
    setLoading(true);
    const tx = await updateSellPrice(price, info.mint, provider, program).catch(
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
    refreshNftList();
    setTimeout(() => {
      getNftSell();
    }, 18000);
  }, [info.mint, price, program, provider, refreshNftList, getNftSell]);

  const renderInfo = useMemo(() => {
    if (isSold) {
      return (
        <div className="w-full bg-gray-500 mt-3 p-2 rounded-lg flex text-white items-center justify-center">
          Already Sold
        </div>
      );
    }
    if (isSelf) {
      return (
        <>
          <button
            className={`btn btn-active btn-warning text-white ${
              loading ? 'loading' : ''
            }`}
            disabled={loading}
            onClick={() => handleCancel()}
          >
            Cancel
          </button>
          <button
            className={`btn btn-active btn-info text-white ${
              loading ? 'loading' : ''
            }`}
            disabled={loading || !price}
            onClick={() => handleUpdate()}
          >
            Update
          </button>
        </>
      );
    }
    return (
      <button
        className={`btn btn-active btn-accent text-white ${
          loading ? 'loading' : ''
        }`}
        disabled={loading}
        onClick={() => handleBuy()}
      >
        buy
      </button>
    );
  }, [isSold, isSelf, loading, handleBuy, price, handleUpdate, handleCancel]);

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
        <div>
          mint:{' '}
          <a
            className="text-xs text-blue-500 cursor-pointer"
            href={`https://solscan.io/token/${info.mint}?cluster=devnet`}
            target="_blank"
            rel="noopener noreferrer"
          >
            {info.mint}
          </a>
        </div>
        <div>
          <p>price: {getSellPrice} SOL</p>
          {!isSold && isSelf && (
            <input
              type="text"
              placeholder="Input SOL amount"
              className="input input-bordered input-info w-full my-4"
              onChange={(e) => setPrice(e.target.value)}
            />
          )}
        </div>
        <div className="card-actions justify-center">{renderInfo}</div>
      </div>
    </div>
  );
};

export default BuyNFT;

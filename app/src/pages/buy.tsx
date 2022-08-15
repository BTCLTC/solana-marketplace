import { NextPage } from 'next';
import { m } from 'framer-motion';
import { useEffect, useState } from 'react';
import { useRecoilValue } from 'recoil';

import BuyNFT from '../components/BuyNFT';
import { appState } from '../stores';
import { fade } from '../utils';
import { loadMarketNfts } from '../solana/nft';
import { INFT } from '../interface';

const Sell: NextPage = () => {
  const { provider } = useRecoilValue(appState);

  const [nfts, setNfts] = useState<INFT[]>([]);

  useEffect(() => {
    if (provider) {
      loadMarketNfts().then((data) => {
        setNfts(data);
      });
    }
  }, [provider]);

  return (
    <m.section
      variants={fade}
      className="container flex flex-wrap justify-between"
    >
      {nfts.map((item) => (
        <BuyNFT key={item.mint} info={item} />
      ))}
    </m.section>
  );
};

export default Sell;

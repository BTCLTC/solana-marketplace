import { NextPage } from 'next';
import { m } from 'framer-motion';
import { useEffect, useState } from 'react';
import { useRecoilValue } from 'recoil';

import SellNFT from '../components/SellNFT';
import { appState } from '../stores';
import { fade } from '../utils';
import { loadAllNFTs } from '../solana/nft';
import { INFT } from '../interface';

const Sell: NextPage = () => {
  const { provider } = useRecoilValue(appState);

  const [nfts, setNfts] = useState<INFT[]>([]);

  useEffect(() => {
    if (provider) {
      loadAllNFTs(provider).then((data) => {
        if (data) {
          setNfts(data);
        }
      });
    }
  }, [provider]);

  return (
    <m.section
      variants={fade}
      className="container flex flex-wrap justify-between"
    >
      {nfts.map((item) => (
        <SellNFT key={item.mint} info={item} />
      ))}
    </m.section>
  );
};

export default Sell;

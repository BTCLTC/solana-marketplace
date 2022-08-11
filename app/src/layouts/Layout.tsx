import { FC } from 'react';
import { DefaultSeo } from 'next-seo';
import { LazyMotion, domAnimation, m } from 'framer-motion';
import SEO from '../../next-seo.json';

import Header from './Header';
import Footer from './Footer';

const DefaultLayout: FC = ({ children }) => {
  return (
    <>
      <DefaultSeo {...SEO} />
      <Header />
      <LazyMotion features={domAnimation}>
        <m.main
          initial="initial"
          animate="enter"
          exit="exit"
          className="main flex-1 flex items-center justify-center"
        >
          {children}
        </m.main>
      </LazyMotion>
      <Footer />
    </>
  );
};

export default DefaultLayout;

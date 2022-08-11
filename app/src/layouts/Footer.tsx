import React from 'react';

const Footer: React.FC = () => {
  return (
    <footer className="footer-wrap text-gray-700 flex flex-col flex-shrink-0 w-full">
      <div className="footer-micro flex flex-1 flex-col justify-center items-center">
        <div className="color-text mb-2">NFT Marketplace</div>
        <div>© 2022 BitKeep</div>
      </div>
    </footer>
  );
};

export default Footer;

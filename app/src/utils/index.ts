export const formatTx = (tx: string, num = 8) => {
  if (!tx) return '';
  const len = tx.length;
  if (len <= num * 2) {
    return tx;
  }
  return tx.substring(0, num) + '...' + tx.substring(len - num, len);
};

export const fade = {
  initial: { opacity: 0 },
  enter: {
    opacity: 1,
    transition: { duration: 0.4, ease: [0.83, 0, 0.17, 1] },
  },
  exit: {
    opacity: 0,
    transition: { duration: 0.4, ease: [0.83, 0, 0.17, 1] },
  },
};

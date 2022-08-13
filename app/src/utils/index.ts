export const formatTx = (tx: string, num = 8) => {
  if (!tx) return '';
  const len = tx.length;
  if (len <= num * 2) {
    return tx;
  }
  return tx.substring(0, num) + '...' + tx.substring(len - num, len);
};

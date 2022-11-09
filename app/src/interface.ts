import { PublicKey } from "@solana/web3.js";

interface ICrators {
  address: PublicKey;
  share: number;
  verified: boolean;
}

interface IData {
  creators: ICrators[];
  name: string;
  symbol: string;
  uri: string;
  sellerFeeBasisPoints: number;
}

export interface INFT {
  data: IData;
  edition?: string;
  editionNonce: number | null;
  isMutable: boolean;
  key: number;
  masterEdition?: string;
  mint: string;
  primarySaleHappened: boolean;
  updateAuthority: string;
  seller?: string;
}

interface ICrators {
  address: string;
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
  editionNonce: number;
  isMutable: boolean;
  key: number;
  masterEdition?: string;
  mint: string;
  primarySaleHappened: boolean;
  updateAuthority: string;
}

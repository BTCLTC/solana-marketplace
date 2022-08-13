export type SolanaMarketplace = {
  version: '0.1.0';
  name: 'solana_marketplace';
  instructions: [
    {
      name: 'setup';
      accounts: [
        {
          name: 'owner';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'feeAccount';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'config';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
          docs: ['used by anchor for init of the token'];
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'rent';
          isMut: false;
          isSigner: false;
        }
      ];
      args: [
        {
          name: 'bump';
          type: 'u8';
        },
        {
          name: 'feeRate';
          type: 'u64';
        }
      ];
    },
    {
      name: 'updateFeeAccount';
      accounts: [
        {
          name: 'owner';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'feeAccount';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'config';
          isMut: true;
          isSigner: false;
        }
      ];
      args: [];
    },
    {
      name: 'updateFeeRate';
      accounts: [
        {
          name: 'owner';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'config';
          isMut: true;
          isSigner: false;
        }
      ];
      args: [
        {
          name: 'feeRate';
          type: 'u64';
        }
      ];
    },
    {
      name: 'updateOwner';
      accounts: [
        {
          name: 'owner';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'newOwner';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'config';
          isMut: true;
          isSigner: false;
        }
      ];
      args: [];
    },
    {
      name: 'toggleFreeze';
      accounts: [
        {
          name: 'owner';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'config';
          isMut: true;
          isSigner: false;
        }
      ];
      args: [];
    },
    {
      name: 'sell';
      accounts: [
        {
          name: 'seller';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'config';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'nftMint';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'nftVault';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'userNftVault';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'sell';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
          docs: ['used by anchor for init of the token'];
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'rent';
          isMut: false;
          isSigner: false;
        }
      ];
      args: [
        {
          name: 'price';
          type: 'u64';
        }
      ];
    },
    {
      name: 'updateSellPrice';
      accounts: [
        {
          name: 'seller';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'config';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'nftMint';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'sell';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
          docs: ['used by anchor for init of the token'];
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'rent';
          isMut: false;
          isSigner: false;
        }
      ];
      args: [
        {
          name: 'price';
          type: 'u64';
        }
      ];
    },
    {
      name: 'closeSell';
      accounts: [
        {
          name: 'seller';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'config';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'nftMint';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'nftVault';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'userNftVault';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'sell';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
          docs: ['used by anchor for init of the token'];
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'rent';
          isMut: false;
          isSigner: false;
        }
      ];
      args: [];
    },
    {
      name: 'buy';
      accounts: [
        {
          name: 'buyer';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'seller';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'config';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'nftMint';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'nftVault';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'buyerNftVault';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'feeAccount';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'sell';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
          docs: ['used by anchor for init of the token'];
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'rent';
          isMut: false;
          isSigner: false;
        }
      ];
      args: [];
    }
  ];
  accounts: [
    {
      name: 'config';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'owner';
            type: 'publicKey';
          },
          {
            name: 'feeAccount';
            type: 'publicKey';
          },
          {
            name: 'feeRate';
            docs: [
              '(0-10000), 1: 0.01%; 10: 0.1%; 100: 1%; 1000: 10%; 10000: 100%'
            ];
            type: 'u64';
          },
          {
            name: 'orderId';
            type: 'u64';
          },
          {
            name: 'orderCount';
            type: 'u64';
          },
          {
            name: 'freeze';
            type: 'bool';
          },
          {
            name: 'bump';
            type: 'u8';
          }
        ];
      };
    },
    {
      name: 'sell';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'orderId';
            type: 'u64';
          },
          {
            name: 'seller';
            type: 'publicKey';
          },
          {
            name: 'nftMint';
            type: 'publicKey';
          },
          {
            name: 'nftVault';
            type: 'publicKey';
          },
          {
            name: 'price';
            type: 'u64';
          },
          {
            name: 'createdAt';
            type: 'u64';
          }
        ];
      };
    }
  ];
  events: [
    {
      name: 'BuyEvent';
      fields: [
        {
          name: 'orderId';
          type: 'u64';
          index: false;
        },
        {
          name: 'buyer';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'seller';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'nftMint';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'nftVault';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'buyerNftVault';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'price';
          type: 'u64';
          index: false;
        },
        {
          name: 'createdAt';
          type: 'u64';
          index: false;
        }
      ];
    },
    {
      name: 'CloseSellEvent';
      fields: [
        {
          name: 'orderId';
          type: 'u64';
          index: false;
        },
        {
          name: 'seller';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'nftMint';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'nftVault';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'createdAt';
          type: 'u64';
          index: false;
        }
      ];
    },
    {
      name: 'SellEvent';
      fields: [
        {
          name: 'orderId';
          type: 'u64';
          index: false;
        },
        {
          name: 'seller';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'nftMint';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'nftVault';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'price';
          type: 'u64';
          index: false;
        },
        {
          name: 'createdAt';
          type: 'u64';
          index: false;
        }
      ];
    }
  ];
  errors: [
    {
      code: 6000;
      name: 'FreezeProgramError';
      msg: 'The contract frozen';
    },
    {
      code: 6001;
      name: 'FeeRateError';
      msg: 'Fee Rate Error';
    },
    {
      code: 6002;
      name: 'InvalidRequestError';
      msg: 'Invalid Request';
    },
    {
      code: 6003;
      name: 'InsufficientSolAmountError';
      msg: 'Not enough SOL';
    },
    {
      code: 6004;
      name: 'InsufficientMinAmountError';
      msg: 'The amount is small than min price';
    },
    {
      code: 6005;
      name: 'IncorrectOwner';
      msg: 'IncorrectOwner';
    },
    {
      code: 6006;
      name: 'DerivedKeyInvalid';
      msg: 'Derived key invalid';
    },
    {
      code: 6007;
      name: 'MetadataNotExist';
      msg: "Metadata doesn't exist";
    },
    {
      code: 6008;
      name: 'PublicKeyMismatch';
      msg: 'PublicKeyMismatch';
    },
    {
      code: 6009;
      name: 'UninitializedAccount';
      msg: 'UninitializedAccount';
    },
    {
      code: 6010;
      name: 'NoPayerPresent';
      msg: 'No payer present on this txn';
    },
    {
      code: 6011;
      name: 'InvalidTokenAmount';
      msg: 'Invalid token amount';
    }
  ];
};

export const IDL: SolanaMarketplace = {
  version: '0.1.0',
  name: 'solana_marketplace',
  instructions: [
    {
      name: 'setup',
      accounts: [
        {
          name: 'owner',
          isMut: true,
          isSigner: true,
        },
        {
          name: 'feeAccount',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'config',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
          docs: ['used by anchor for init of the token'],
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'rent',
          isMut: false,
          isSigner: false,
        },
      ],
      args: [
        {
          name: 'bump',
          type: 'u8',
        },
        {
          name: 'feeRate',
          type: 'u64',
        },
      ],
    },
    {
      name: 'updateFeeAccount',
      accounts: [
        {
          name: 'owner',
          isMut: true,
          isSigner: true,
        },
        {
          name: 'feeAccount',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'config',
          isMut: true,
          isSigner: false,
        },
      ],
      args: [],
    },
    {
      name: 'updateFeeRate',
      accounts: [
        {
          name: 'owner',
          isMut: true,
          isSigner: true,
        },
        {
          name: 'config',
          isMut: true,
          isSigner: false,
        },
      ],
      args: [
        {
          name: 'feeRate',
          type: 'u64',
        },
      ],
    },
    {
      name: 'updateOwner',
      accounts: [
        {
          name: 'owner',
          isMut: true,
          isSigner: true,
        },
        {
          name: 'newOwner',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'config',
          isMut: true,
          isSigner: false,
        },
      ],
      args: [],
    },
    {
      name: 'toggleFreeze',
      accounts: [
        {
          name: 'owner',
          isMut: false,
          isSigner: true,
        },
        {
          name: 'config',
          isMut: true,
          isSigner: false,
        },
      ],
      args: [],
    },
    {
      name: 'sell',
      accounts: [
        {
          name: 'seller',
          isMut: true,
          isSigner: true,
        },
        {
          name: 'config',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'nftMint',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'nftVault',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'userNftVault',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'sell',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
          docs: ['used by anchor for init of the token'],
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'rent',
          isMut: false,
          isSigner: false,
        },
      ],
      args: [
        {
          name: 'price',
          type: 'u64',
        },
      ],
    },
    {
      name: 'updateSellPrice',
      accounts: [
        {
          name: 'seller',
          isMut: true,
          isSigner: true,
        },
        {
          name: 'config',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'nftMint',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'sell',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
          docs: ['used by anchor for init of the token'],
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'rent',
          isMut: false,
          isSigner: false,
        },
      ],
      args: [
        {
          name: 'price',
          type: 'u64',
        },
      ],
    },
    {
      name: 'closeSell',
      accounts: [
        {
          name: 'seller',
          isMut: true,
          isSigner: true,
        },
        {
          name: 'config',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'nftMint',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'nftVault',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'userNftVault',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'sell',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
          docs: ['used by anchor for init of the token'],
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'rent',
          isMut: false,
          isSigner: false,
        },
      ],
      args: [],
    },
    {
      name: 'buy',
      accounts: [
        {
          name: 'buyer',
          isMut: true,
          isSigner: true,
        },
        {
          name: 'seller',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'config',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'nftMint',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'nftVault',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'buyerNftVault',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'feeAccount',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'sell',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
          docs: ['used by anchor for init of the token'],
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'rent',
          isMut: false,
          isSigner: false,
        },
      ],
      args: [],
    },
  ],
  accounts: [
    {
      name: 'config',
      type: {
        kind: 'struct',
        fields: [
          {
            name: 'owner',
            type: 'publicKey',
          },
          {
            name: 'feeAccount',
            type: 'publicKey',
          },
          {
            name: 'feeRate',
            docs: [
              '(0-10000), 1: 0.01%; 10: 0.1%; 100: 1%; 1000: 10%; 10000: 100%',
            ],
            type: 'u64',
          },
          {
            name: 'orderId',
            type: 'u64',
          },
          {
            name: 'orderCount',
            type: 'u64',
          },
          {
            name: 'freeze',
            type: 'bool',
          },
          {
            name: 'bump',
            type: 'u8',
          },
        ],
      },
    },
    {
      name: 'sell',
      type: {
        kind: 'struct',
        fields: [
          {
            name: 'orderId',
            type: 'u64',
          },
          {
            name: 'seller',
            type: 'publicKey',
          },
          {
            name: 'nftMint',
            type: 'publicKey',
          },
          {
            name: 'nftVault',
            type: 'publicKey',
          },
          {
            name: 'price',
            type: 'u64',
          },
          {
            name: 'createdAt',
            type: 'u64',
          },
        ],
      },
    },
  ],
  events: [
    {
      name: 'BuyEvent',
      fields: [
        {
          name: 'orderId',
          type: 'u64',
          index: false,
        },
        {
          name: 'buyer',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'seller',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'nftMint',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'nftVault',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'buyerNftVault',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'price',
          type: 'u64',
          index: false,
        },
        {
          name: 'createdAt',
          type: 'u64',
          index: false,
        },
      ],
    },
    {
      name: 'CloseSellEvent',
      fields: [
        {
          name: 'orderId',
          type: 'u64',
          index: false,
        },
        {
          name: 'seller',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'nftMint',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'nftVault',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'createdAt',
          type: 'u64',
          index: false,
        },
      ],
    },
    {
      name: 'SellEvent',
      fields: [
        {
          name: 'orderId',
          type: 'u64',
          index: false,
        },
        {
          name: 'seller',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'nftMint',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'nftVault',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'price',
          type: 'u64',
          index: false,
        },
        {
          name: 'createdAt',
          type: 'u64',
          index: false,
        },
      ],
    },
  ],
  errors: [
    {
      code: 6000,
      name: 'FreezeProgramError',
      msg: 'The contract frozen',
    },
    {
      code: 6001,
      name: 'FeeRateError',
      msg: 'Fee Rate Error',
    },
    {
      code: 6002,
      name: 'InvalidRequestError',
      msg: 'Invalid Request',
    },
    {
      code: 6003,
      name: 'InsufficientSolAmountError',
      msg: 'Not enough SOL',
    },
    {
      code: 6004,
      name: 'InsufficientMinAmountError',
      msg: 'The amount is small than min price',
    },
    {
      code: 6005,
      name: 'IncorrectOwner',
      msg: 'IncorrectOwner',
    },
    {
      code: 6006,
      name: 'DerivedKeyInvalid',
      msg: 'Derived key invalid',
    },
    {
      code: 6007,
      name: 'MetadataNotExist',
      msg: "Metadata doesn't exist",
    },
    {
      code: 6008,
      name: 'PublicKeyMismatch',
      msg: 'PublicKeyMismatch',
    },
    {
      code: 6009,
      name: 'UninitializedAccount',
      msg: 'UninitializedAccount',
    },
    {
      code: 6010,
      name: 'NoPayerPresent',
      msg: 'No payer present on this txn',
    },
    {
      code: 6011,
      name: 'InvalidTokenAmount',
      msg: 'Invalid token amount',
    },
  ],
};

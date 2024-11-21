import { EVMNetwork } from './pages/Background/types/network';

// eslint-disable-next-line import/no-anonymous-default-export
export default {
  enablePasswordEncryption: false,
  showTransactionConfirmationScreen: true,
  //factory_address: '0x91E60e0613810449d098b0b5Ec8b51A0FE8c8985',
  factory_address: '0xDc64a140Aa3E981100a9becA4E685f962f0cF6C9',
  stateVersion: '0.1',
  network: {
    chainID: '11011',
    family: 'EVM',
    name: 'Sepolia',
    provider: 'http://127.0.0.1:8545',
    //provider: 'https://sepolia.infura.io/v3/bdabe9d2f9244005af0f566398e648da',
    //entryPointAddress: '0x2797b22CFACf9D243B0587ddEF368f8C362A81f2',
    entryPointAddress: '0x5ff137d4b0fdcd49dca30c7cf57e578a026d2789',
    bundler: 'http://localhost:3000/rpc',
    baseAsset: {
      symbol: 'ETH',
      name: 'ETH',
      decimals: 18,
      image:
        'https://ethereum.org/static/6b935ac0e6194247347855dc3d328e83/6ed5f/eth-diamond-black.webp',
    },
  } satisfies EVMNetwork,
};

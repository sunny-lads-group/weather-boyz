import { Contract, BrowserProvider } from 'ethers';

const CONTRACT_ADDRESS =
  import.meta.env.CONTRACT_ADDRESS ||
  '0x5FbDB2315678afecb367f032d93F642f64180aa3';
const CONTRACT_ABI = [
  {
    name: 'buyPolicy',
    type: 'function',
    inputs: [
      { name: 'duration', type: 'uint256' },
      { name: 'payout', type: 'uint256' },
      { name: 'threshold', type: 'int256' },
      { name: 'eventType', type: 'string' },
      { name: 'h3HexId', type: 'string' },
    ],
    outputs: [],
    stateMutability: 'payable',
  },
];

export const getContract = async (provider: BrowserProvider) => {
  try {
    const signer = await provider.getSigner();
    const contract = new Contract(CONTRACT_ADDRESS, CONTRACT_ABI, signer);
    console.log('Contract address:', await contract.getAddress());
    return contract;
  } catch (error) {
    console.error('Error getting contract:', error);
    throw error;
  }
};

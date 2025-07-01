import { Contract, BrowserProvider } from 'ethers';

const CONTRACT_ADDRESS =
  import.meta.env.CONTRACT_ADDRESS ||
  '0x5FbDB2315678afecb367f032d93F642f64180aa3';
const CONTRACT_ABI = [
  'function buyPolicy(uint duration, uint payout, int256 threshold, string memory eventType, string memory h3HexId) public payable',
];

export const getContract = async (provider: BrowserProvider) => {
  const signer = await provider.getSigner();
  return new Contract(CONTRACT_ADDRESS, CONTRACT_ABI, signer);
};

// src/components/WalletConnect.tsx
import { useWallet } from '../context/WalletContext';

const WalletConnect = () => {
  const { address, isConnecting, connectWallet, disconnectWallet } = useWallet();

  const handleConnect = async () => {
    try {
      await connectWallet();
    } catch (error) {
      console.error('Error connecting wallet:', error);
    }
  };

  return (
    <div className="flex items-center">
      {!address ? (
        <button
          onClick={handleConnect}
          disabled={isConnecting}
          className="bg-white text-orange-500 px-4 py-2 rounded-lg font-medium
                     hover:bg-orange-100 transition-colors duration-200
                     disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {isConnecting ? 'Connecting...' : 'Connect Wallet'}
        </button>
      ) : (
        <div className="flex items-center space-x-2">
          <span className="text-white">
            {address.slice(0, 6)}...{address.slice(-4)}
          </span>
          <button
            onClick={disconnectWallet}
            className="text-white hover:text-orange-200 text-sm"
          >
            Disconnect
          </button>
        </div>
      )}
    </div>
  );
};

export default WalletConnect;
// src/context/WalletContext.tsx
import { createContext, useContext, useState, useEffect } from 'react';
import type { ReactNode } from 'react';
import { BrowserProvider } from 'ethers';
import { updateWalletAddress } from '../services/authService';

// Add type declaration for window.ethereum
declare global {
  interface Window {
    ethereum?: any;
  }
}

interface WalletContextType {
  address: string;
  isConnected: boolean;
  isConnecting: boolean;
  isLoading: boolean;
  connectWallet: () => Promise<void>;
  disconnectWallet: () => void;
  syncWalletAddress: () => Promise<string>;
}

const WalletContext = createContext<WalletContextType | undefined>(undefined);

export const WalletProvider = ({ children }: { children: ReactNode }) => {
  const [address, setAddress] = useState('');
  const [isConnecting, setIsConnecting] = useState(false);
  const [isLoading, setIsLoading] = useState(true);

  // Check for existing connection on mount
  useEffect(() => {
    const checkConnection = async () => {
      setIsLoading(true);
      const savedAddress = localStorage.getItem('walletAddress');
      if (savedAddress && window.ethereum) {
        try {
          const provider = new BrowserProvider(window.ethereum);
          const accounts = await provider.listAccounts();
          if (accounts.length > 0 && accounts[0].address === savedAddress) {
            setAddress(savedAddress);
          } else {
            localStorage.removeItem('walletAddress');
          }
        } catch (error) {
          console.error('Error checking wallet connection:', error);
          localStorage.removeItem('walletAddress');
        }
      }
      setIsLoading(false);
    };

    checkConnection();
  }, []);

  // Listen for wallet disconnect events (e.g., when user logs out)
  useEffect(() => {
    const handleWalletDisconnect = () => {
      setAddress('');
    };

    // Listen for custom wallet disconnect event
    window.addEventListener('walletDisconnect', handleWalletDisconnect);
    
    return () => window.removeEventListener('walletDisconnect', handleWalletDisconnect);
  }, []);

  // Listen for MetaMask account changes
  useEffect(() => {
    if (!window.ethereum) return;

    const handleAccountsChanged = async (accounts: string[]) => {
      console.log('MetaMask accounts changed:', accounts);
      
      if (accounts.length === 0) {
        // User disconnected all accounts
        console.log('No accounts connected');
        setAddress('');
        localStorage.removeItem('walletAddress');
      } else {
        // User switched to a different account
        const newAddress = accounts[0];
        console.log('Switched to account:', newAddress);
        setAddress(newAddress);
        localStorage.setItem('walletAddress', newAddress);
        
        // Sync new wallet address with backend (only if user is authenticated)
        const token = localStorage.getItem('authToken');
        if (token) {
          try {
            await updateWalletAddress(newAddress);
            console.log('✅ New wallet address synced with backend:', newAddress);
          } catch (error) {
            console.warn('⚠️ Failed to sync new wallet address with backend:', error);
          }
        }
      }
    };

    // Add event listener for account changes
    window.ethereum.on('accountsChanged', handleAccountsChanged);
    
    // Cleanup
    return () => {
      if (window.ethereum) {
        window.ethereum.removeListener('accountsChanged', handleAccountsChanged);
      }
    };
  }, []);

  const connectWallet = async () => {
    try {
      setIsConnecting(true);

      if (!window.ethereum) {
        throw new Error('Please install MetaMask!');
      }

      const provider = new BrowserProvider(window.ethereum);
      await window.ethereum.request({ method: 'eth_requestAccounts' });
      const signer = await provider.getSigner();
      const walletAddress = await signer.getAddress();
      setAddress(walletAddress);
      localStorage.setItem('walletAddress', walletAddress);

      // Update wallet address in backend (only if user is authenticated)
      const token = localStorage.getItem('authToken');
      if (token) {
        try {
          await updateWalletAddress(walletAddress);
          console.log('✅ Wallet address synced with backend');
        } catch (error) {
          console.warn('⚠️ Failed to sync wallet address with backend:', error);
          // Don't throw error here as wallet connection was successful
        }
      }
    } catch (error) {
      console.error('Error connecting wallet:', error);
      throw error;
    } finally {
      setIsConnecting(false);
    }
  };

  const disconnectWallet = () => {
    setAddress('');
    localStorage.removeItem('walletAddress');
  };

  const syncWalletAddress = async (): Promise<string> => {
    if (!window.ethereum) {
      throw new Error('MetaMask is not installed');
    }

    try {
      const provider = new BrowserProvider(window.ethereum);
      const signer = await provider.getSigner();
      const currentAddress = await signer.getAddress();
      
      // Update local state if address changed
      if (currentAddress !== address) {
        console.log('Wallet address changed from', address, 'to', currentAddress);
        setAddress(currentAddress);
        localStorage.setItem('walletAddress', currentAddress);
        
        // Sync with backend (only if user is authenticated)
        const token = localStorage.getItem('authToken');
        if (token) {
          await updateWalletAddress(currentAddress);
          console.log('✅ Wallet address synced with backend:', currentAddress);
        }
      }
      
      return currentAddress;
    } catch (error) {
      console.error('Error syncing wallet address:', error);
      throw error;
    }
  };

  return (
    <WalletContext.Provider
      value={{
        address,
        isConnected: !!address,
        isConnecting,
        isLoading,
        connectWallet,
        disconnectWallet,
        syncWalletAddress,
      }}
    >
      {children}
    </WalletContext.Provider>
  );
};

export const useWallet = () => {
  const context = useContext(WalletContext);
  if (!context) throw new Error('useWallet must be used within WalletProvider');
  return context;
};

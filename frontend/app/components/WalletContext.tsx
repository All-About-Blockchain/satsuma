import React, { createContext, useContext, useState, useEffect, useCallback } from 'react';
import { Wallet, WalletStrategy } from '@injectivelabs/wallet-ts';
import { ChainId } from '@injectivelabs/ts-types';

export type WalletType = Wallet | 'icp' | null;

interface WalletContextProps {
  address: string | null;
  walletType: WalletType;
  icpPrincipal: string | null;
  connect: (type: Wallet | 'icp') => Promise<void>;
  disconnect: () => void;
  isConnected: boolean;
  isIcpConnected: boolean;
  isInjectiveConnected: boolean;
}

const WalletContext = createContext<WalletContextProps | undefined>(undefined);

export const WalletProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [address, setAddress] = useState<string | null>(null);
  const [walletType, setWalletType] = useState<WalletType>(null);
  const [icpPrincipal, setIcpPrincipal] = useState<string | null>(null);
  const [walletStrategy] = useState(() => new WalletStrategy({ chainId: ChainId.Testnet }));

  const connect = useCallback(async (type: Wallet | 'icp') => {
    if (type === 'icp') {
      // Connect to Internet Identity
      try {
        // @ts-expect-error - Internet Identity types
        const identity = await window.ic?.plug?.requestConnect();
        if (identity) {
          setWalletType('icp');
          setIcpPrincipal(identity.principal);
          setAddress(identity.principal);
          localStorage.setItem('wallet', 'icp');
        }
      } catch (error) {
        console.error('Failed to connect to ICP:', error);
      }
    } else {
      // Connect to Injective wallet
      try {
        await walletStrategy.setWallet(type);
        const addresses = await walletStrategy.getAddresses();
        setWalletType(type);
        setAddress(addresses[0]);
        setIcpPrincipal(null);
        localStorage.setItem('wallet', type);
      } catch (error) {
        console.error('Failed to connect wallet:', error);
      }
    }
  }, [walletStrategy]);

  const disconnect = useCallback(() => {
    setWalletType(null);
    setAddress(null);
    setIcpPrincipal(null);
    localStorage.removeItem('wallet');
    walletStrategy.disconnect();
  }, [walletStrategy]);

  // Auto-reconnect on mount
  useEffect(() => {
    const lastWallet = localStorage.getItem('wallet') as Wallet | 'icp' | null;
    if (lastWallet) connect(lastWallet);
  }, [connect]);

  // Listen for keystore changes (Keplr example)
  useEffect(() => {
    const reconnect = () => {
      if (walletType === Wallet.Keplr) connect(Wallet.Keplr);
    };
    window.addEventListener('keplr_keystorechange', reconnect);
    return () => window.removeEventListener('keplr_keystorechange', reconnect);
  }, [walletType, connect]);

  const isConnected = !!address;
  const isIcpConnected = walletType === 'icp';
  const isInjectiveConnected = walletType !== null && walletType !== 'icp';

  return (
    <WalletContext.Provider value={{ 
      address, 
      walletType, 
      icpPrincipal,
      connect, 
      disconnect,
      isConnected,
      isIcpConnected,
      isInjectiveConnected
    }}>
      {children}
    </WalletContext.Provider>
  );
};

export function useWallet() {
  const ctx = useContext(WalletContext);
  if (!ctx) throw new Error('useWallet must be used within WalletProvider');
  return ctx;
} 
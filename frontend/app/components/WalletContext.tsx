import React, { createContext, useContext, useState, useEffect, useCallback } from 'react';
import { Wallet, WalletStrategy } from '@injectivelabs/wallet-ts';
import { ChainId } from '@injectivelabs/ts-types';

export type WalletType = Wallet | null;

interface WalletContextProps {
  address: string | null;
  walletType: WalletType;
  connect: (type: Wallet) => Promise<void>;
  disconnect: () => void;
}

const WalletContext = createContext<WalletContextProps | undefined>(undefined);

export const WalletProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [address, setAddress] = useState<string | null>(null);
  const [walletType, setWalletType] = useState<WalletType>(null);
  const [walletStrategy] = useState(() => new WalletStrategy({ chainId: ChainId.Testnet }));

  const connect = useCallback(async (type: Wallet) => {
    await walletStrategy.setWallet(type);
    const addresses = await walletStrategy.getAddresses();
    setWalletType(type);
    setAddress(addresses[0]);
    localStorage.setItem('wallet', type);
  }, [walletStrategy]);

  const disconnect = useCallback(() => {
    setWalletType(null);
    setAddress(null);
    localStorage.removeItem('wallet');
    walletStrategy.disconnect();
  }, [walletStrategy]);

  // Auto-reconnect on mount
  useEffect(() => {
    const lastWallet = localStorage.getItem('wallet') as Wallet | null;
    if (lastWallet) connect(lastWallet as Wallet);
  }, [connect]);

  // Listen for keystore changes (Keplr example)
  useEffect(() => {
    const reconnect = () => {
      if (walletType === Wallet.Keplr) connect(Wallet.Keplr);
    };
    window.addEventListener('keplr_keystorechange', reconnect);
    return () => window.removeEventListener('keplr_keystorechange', reconnect);
  }, [walletType, connect]);

  return (
    <WalletContext.Provider value={{ address, walletType, connect, disconnect }}>
      {children}
    </WalletContext.Provider>
  );
};

export function useWallet() {
  const ctx = useContext(WalletContext);
  if (!ctx) throw new Error('useWallet must be used within WalletProvider');
  return ctx;
} 
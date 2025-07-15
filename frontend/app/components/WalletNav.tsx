import React from 'react';
import { useWallet } from './WalletContext';
import { Wallet } from '@injectivelabs/wallet-ts';
import { Button } from './ui/button';

const WALLET_OPTIONS = [
  { name: 'Keplr', type: Wallet.Keplr },
  { name: 'Leap', type: Wallet.Leap },
  { name: 'MetaMask', type: Wallet.Metamask },
];

const shortAddress = (addr: string) => addr.slice(0, 6) + '...' + addr.slice(-4);

const WalletNav: React.FC = () => {
  const { address, walletType, connect, disconnect } = useWallet();

  return (
    <div className="flex items-center gap-2">
      {address ? (
        <>
          <span className="px-3 py-1 bg-muted rounded text-xs font-mono">
            {walletType} {shortAddress(address)}
          </span>
          <Button size="sm" variant="outline" onClick={disconnect}>
            Disconnect
          </Button>
        </>
      ) : (
        WALLET_OPTIONS.map((w) => (
          <Button
            key={w.type}
            size="sm"
            variant="outline"
            onClick={() => connect(w.type)}
          >
            Connect {w.name}
          </Button>
        ))
      )}
    </div>
  );
};

export default WalletNav; 
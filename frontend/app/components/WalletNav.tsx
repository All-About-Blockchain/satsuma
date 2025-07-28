import React from 'react';
import { useWallet } from './WalletContext';
import { Wallet } from '@injectivelabs/wallet-ts';
import { Button } from './ui/button';

const WALLET_OPTIONS = [
  { name: 'ICP', type: 'icp' as const, description: 'Internet Computer' },
  { name: 'Keplr', type: Wallet.Keplr, description: 'Cosmos Ecosystem' },
  { name: 'Leap', type: Wallet.Leap, description: 'Cosmos Ecosystem' },
  { name: 'MetaMask', type: Wallet.Metamask, description: 'Ethereum Ecosystem' },
];

const shortAddress = (addr: string) => addr.slice(0, 6) + '...' + addr.slice(-4);

const WalletNav: React.FC = () => {
  const { 
    address, 
    walletType, 
    icpPrincipal,
    connect, 
    disconnect,
    isConnected,
    isIcpConnected,
    isInjectiveConnected
  } = useWallet();

  const getWalletDisplay = () => {
    if (isIcpConnected && icpPrincipal) {
      return `ICP ${shortAddress(icpPrincipal)}`;
    } else if (isInjectiveConnected && address) {
      return `${walletType} ${shortAddress(address)}`;
    }
    return null;
  };

  const getConnectionStatus = () => {
    if (isIcpConnected) {
      return { status: 'ICP Connected', color: 'text-blue-600' };
    } else if (isInjectiveConnected) {
      return { status: 'Injective Connected', color: 'text-green-600' };
    }
    return { status: 'Not Connected', color: 'text-gray-500' };
  };

  const { status, color } = getConnectionStatus();

  return (
    <div className="flex items-center gap-4">
      {/* Connection Status */}
      <div className="text-sm">
        <span className={`font-medium ${color}`}>{status}</span>
      </div>

      {/* Wallet Display */}
      {isConnected && (
        <div className="flex items-center gap-2">
          <span className="px-3 py-1 bg-muted rounded text-xs font-mono">
            {getWalletDisplay()}
          </span>
          <Button size="sm" variant="outline" onClick={disconnect}>
            Disconnect
          </Button>
        </div>
      )}

      {/* Connect Options */}
      {!isConnected && (
        <div className="flex gap-2">
          {WALLET_OPTIONS.map((w) => (
            <Button
              key={w.type}
              size="sm"
              variant="outline"
              onClick={() => connect(w.type)}
              className="flex flex-col items-center gap-1"
            >
              <span className="text-xs font-medium">{w.name}</span>
              <span className="text-xs text-muted-foreground">{w.description}</span>
            </Button>
          ))}
        </div>
      )}

      {/* Cross-Chain Actions */}
      {isConnected && (
        <div className="flex gap-2">
          {isIcpConnected && (
            <Button size="sm" variant="secondary" className="text-xs">
              View ICP Balance
            </Button>
          )}
          {isInjectiveConnected && (
            <Button size="sm" variant="secondary" className="text-xs">
              View Injective Balance
            </Button>
          )}
        </div>
      )}
    </div>
  );
};

export default WalletNav; 
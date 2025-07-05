import React from 'react';

interface Transaction {
  id: string;
  type: 'yield_conversion' | 'yield_earned' | 'deposit';
  amount: number;
  currency: string;
  bitcoinAmount?: number;
  timestamp: Date;
  txHash?: string;
  status: 'completed' | 'pending' | 'failed';
}

interface TransactionHistoryProps {
  transactions: Transaction[];
  onViewTransaction: (txHash: string) => void;
}

export default function TransactionHistory({
  transactions,
  onViewTransaction
}: TransactionHistoryProps) {
  const getTransactionIcon = (type: Transaction['type']) => {
    switch (type) {
      case 'yield_conversion':
        return '🔄';
      case 'yield_earned':
        return '💰';
      case 'deposit':
        return '📥';
      default:
        return '📋';
    }
  };

  const getTransactionColor = (type: Transaction['type']) => {
    switch (type) {
      case 'yield_conversion':
        return 'text-green-600';
      case 'yield_earned':
        return 'text-blue-600';
      case 'deposit':
        return 'text-purple-600';
      default:
        return 'text-gray-600';
    }
  };

  const formatTimestamp = (timestamp: Date) => {
    return timestamp.toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  };

  return (
    <div className="bg-card p-6 rounded-lg border">
      <h3 className="text-lg font-semibold mb-4">Recent Transactions</h3>
      <div className="space-y-3">
        {transactions.map((tx) => (
          <div key={tx.id} className="flex items-center justify-between p-3 bg-muted rounded">
            <div className="flex items-center space-x-3">
              <div className={`text-xl ${getTransactionColor(tx.type)}`}>
                {getTransactionIcon(tx.type)}
              </div>
              <div>
                <div className="font-medium">
                  {tx.type === 'yield_conversion' && 'Yield Converted to Bitcoin'}
                  {tx.type === 'yield_earned' && 'Yield Earned'}
                  {tx.type === 'deposit' && 'Deposit'}
                </div>
                <div className="text-sm text-muted-foreground">
                  {formatTimestamp(tx.timestamp)}
                </div>
              </div>
            </div>
            <div className="text-right">
              <div className="font-medium">
                {tx.type === 'yield_conversion' && (
                  <>
                    <div>+{tx.bitcoinAmount?.toFixed(8)} BTC</div>
                    <div className="text-sm text-muted-foreground">
                      ${tx.amount.toFixed(2)} {tx.currency}
                    </div>
                  </>
                )}
                {tx.type === 'yield_earned' && (
                  <div>+{tx.amount.toFixed(2)} {tx.currency}</div>
                )}
                {tx.type === 'deposit' && (
                  <div>+{tx.amount.toLocaleString()} {tx.currency}</div>
                )}
              </div>
              {tx.txHash && (
                <button
                  onClick={() => onViewTransaction(tx.txHash!)}
                  className="text-xs text-primary hover:underline"
                >
                  View
                </button>
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
} 
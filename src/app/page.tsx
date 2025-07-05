'use client';

import { useState } from 'react';
import Dashboard from './components/Dashboard';
import BitcoinAccumulationChart from './components/BitcoinAccumulationChart';
import TransactionHistory from './components/TransactionHistory';
import DepositModal from './components/DepositModal';
import { ToastProvider, useToast } from './components/Toaster';

// Mock data
const mockStablecoinHoldings = [
  {
    symbol: "USDC",
    amount: 25000,
    value: 25000,
    apy: 5.2,
    dailyYield: 3.56
  },
  {
    symbol: "USDT",
    amount: 15000,
    value: 15000,
    apy: 4.8,
    dailyYield: 1.97
  },
  {
    symbol: "DAI",
    amount: 10000,
    value: 10000,
    apy: 5.5,
    dailyYield: 1.51
  }
];

const mockAccumulationData = [
  { date: "Dec 1", bitcoinAmount: 0.00234567, yieldConverted: 105.23, portfolioValue: 48500 },
  { date: "Dec 5", bitcoinAmount: 0.00456789, yieldConverted: 205.67, portfolioValue: 49200 },
  { date: "Dec 10", bitcoinAmount: 0.00678912, yieldConverted: 315.45, portfolioValue: 49800 },
  { date: "Dec 15", bitcoinAmount: 0.00891234, yieldConverted: 425.89, portfolioValue: 50300 },
  { date: "Dec 20", bitcoinAmount: 0.01123456, yieldConverted: 545.12, portfolioValue: 50900 },
  { date: "Dec 25", bitcoinAmount: 0.01345678, yieldConverted: 665.78, portfolioValue: 51400 },
  { date: "Dec 30", bitcoinAmount: 0.01567890, yieldConverted: 786.45, portfolioValue: 52000 }
];

const mockTransactions = [
  {
    id: "1",
    type: "yield_conversion" as const,
    amount: 45.67,
    currency: "USD",
    bitcoinAmount: 0.00101234,
    timestamp: new Date("2024-12-30T14:30:00"),
    txHash: "0x1234...5678",
    status: "completed" as const
  },
  {
    id: "2",
    type: "yield_earned" as const,
    amount: 7.04,
    currency: "USDC",
    timestamp: new Date("2024-12-30T10:15:00"),
    status: "completed" as const
  },
  {
    id: "3",
    type: "deposit" as const,
    amount: 5000,
    currency: "USDT",
    timestamp: new Date("2024-12-29T16:45:00"),
    txHash: "0xabcd...efgh",
    status: "completed" as const
  },
  {
    id: "4",
    type: "yield_conversion" as const,
    amount: 38.92,
    currency: "USD",
    bitcoinAmount: 0.00086543,
    timestamp: new Date("2024-12-29T14:30:00"),
    txHash: "0x9876...5432",
    status: "completed" as const
  },
  {
    id: "5",
    type: "yield_earned" as const,
    amount: 6.23,
    currency: "DAI",
    timestamp: new Date("2024-12-29T09:20:00"),
    status: "completed" as const
  }
];

const mockStablecoinOptions = [
  {
    symbol: "USDC",
    name: "USD Coin",
    apy: 5.2,
    balance: 50000,
    minDeposit: 100
  },
  {
    symbol: "USDT",
    name: "Tether USD",
    apy: 4.8,
    balance: 25000,
    minDeposit: 100
  },
  {
    symbol: "DAI",
    name: "Dai Stablecoin",
    apy: 5.5,
    balance: 30000,
    minDeposit: 100
  }
];

function BitcoinYieldAccumulator() {
  const [chartTimeframe, setChartTimeframe] = useState<'7d' | '30d' | '90d' | '1y'>('30d');
  const [isDepositModalOpen, setIsDepositModalOpen] = useState(false);
  const [stablecoinHoldings] = useState(mockStablecoinHoldings);
  const { addToast } = useToast();

  const totalBitcoinAccrued = 0.01567890;
  const bitcoinPrice = 45123.45;
  const totalYieldGenerated = 786.45;

  const handleDeposit = async (amount: number, currency: string) => {
    // Simulate API call
    await new Promise(resolve => setTimeout(resolve, 2000));
    
    addToast({
      type: 'success',
      title: `Successfully deposited ${amount.toLocaleString()} ${currency}`,
      description: "Your deposit is now earning yield and will be converted to Bitcoin automatically."
    });
  };

  const handleWithdraw = () => {
    addToast({
      type: 'info',
      title: "Withdrawal feature coming soon",
      description: "You'll be able to withdraw your stablecoins while keeping accumulated Bitcoin."
    });
  };

  const handleViewTransaction = (txHash: string) => {
    addToast({
      type: 'info',
      title: `Opening transaction ${txHash.substring(0, 10)}...`,
      description: "This would open the transaction in a blockchain explorer."
    });
  };

  return (
    <div className="min-h-screen bg-background">
      <div className="container mx-auto p-6 space-y-8">
        {/* Header */}
        <div className="text-center space-y-2">
          <h1 className="text-3xl font-bold">Satsuma</h1>
          <h2 className="text-1xl font-bold">Bitcoin Yield Accumulator</h2>
          <p className="text-muted-foreground max-w-2xl mx-auto">
            Deposit yield-bearing stablecoins on Internet Computer and automatically convert the yield to Bitcoin. 
            Earn passive income while building your Bitcoin stack.
          </p>
        </div>

        {/* Main Dashboard */}
        <Dashboard
          stablecoinHoldings={stablecoinHoldings}
          totalBitcoinAccrued={totalBitcoinAccrued}
          bitcoinPrice={bitcoinPrice}
          totalYieldGenerated={totalYieldGenerated}
          onDeposit={() => setIsDepositModalOpen(true)}
          onWithdraw={handleWithdraw}
        />

        {/* Charts and Analytics */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <BitcoinAccumulationChart
            data={mockAccumulationData}
            timeframe={chartTimeframe}
            onTimeframeChange={setChartTimeframe}
          />
          <TransactionHistory
            transactions={mockTransactions}
            onViewTransaction={handleViewTransaction}
          />
        </div>

        {/* Modals */}
        <DepositModal
          isOpen={isDepositModalOpen}
          onClose={() => setIsDepositModalOpen(false)}
          onDeposit={handleDeposit}
          stablecoinOptions={mockStablecoinOptions}
        />
      </div>
    </div>
  );
}

export default function Home() {
  return (
    <ToastProvider>
      <BitcoinYieldAccumulator />
    </ToastProvider>
  );
}

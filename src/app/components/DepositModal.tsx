import React, { useState } from 'react';

interface StablecoinOption {
  symbol: string;
  name: string;
  apy: number;
  balance: number;
  minDeposit: number;
}

interface DepositModalProps {
  isOpen: boolean;
  onClose: () => void;
  onDeposit: (amount: number, currency: string) => Promise<void>;
  stablecoinOptions: StablecoinOption[];
}

export default function DepositModal({
  isOpen,
  onClose,
  onDeposit,
  stablecoinOptions
}: DepositModalProps) {
  const [selectedCurrency, setSelectedCurrency] = useState(stablecoinOptions[0]?.symbol || '');
  const [amount, setAmount] = useState('');
  const [isLoading, setIsLoading] = useState(false);

  const selectedOption = stablecoinOptions.find(option => option.symbol === selectedCurrency);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!amount || !selectedCurrency) return;

    const numAmount = parseFloat(amount);
    if (numAmount < (selectedOption?.minDeposit || 0)) {
      alert(`Minimum deposit is ${selectedOption?.minDeposit} ${selectedCurrency}`);
      return;
    }

    setIsLoading(true);
    try {
      await onDeposit(numAmount, selectedCurrency);
      setAmount('');
      onClose();
    } catch (error) {
      console.error('Deposit failed:', error);
    } finally {
      setIsLoading(false);
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-card p-6 rounded-lg border max-w-md w-full mx-4">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-xl font-semibold">Deposit Stablecoins</h2>
          <button
            onClick={onClose}
            className="text-muted-foreground hover:text-foreground"
          >
            ✕
          </button>
        </div>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block text-sm font-medium mb-2">
              Select Stablecoin
            </label>
            <select
              value={selectedCurrency}
              onChange={(e) => setSelectedCurrency(e.target.value)}
              className="w-full p-3 border rounded-lg bg-background"
            >
              {stablecoinOptions.map((option) => (
                <option key={option.symbol} value={option.symbol}>
                  {option.name} ({option.symbol}) - {option.apy}% APY
                </option>
              ))}
            </select>
          </div>

          <div>
            <label className="block text-sm font-medium mb-2">
              Amount
            </label>
            <input
              type="number"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              placeholder={`Min: ${selectedOption?.minDeposit} ${selectedCurrency}`}
              className="w-full p-3 border rounded-lg bg-background"
              min={selectedOption?.minDeposit}
              step="0.01"
              required
            />
          </div>

          {selectedOption && (
            <div className="p-3 bg-muted rounded-lg">
              <div className="text-sm text-muted-foreground">
                <div>APY: {selectedOption.apy}%</div>
                <div>Available: {selectedOption.balance.toLocaleString()} {selectedOption.symbol}</div>
                <div>Min deposit: {selectedOption.minDeposit} {selectedOption.symbol}</div>
              </div>
            </div>
          )}

          <div className="flex gap-3">
            <button
              type="button"
              onClick={onClose}
              className="flex-1 p-3 border rounded-lg hover:bg-muted transition-colors"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={isLoading || !amount || !selectedCurrency}
              className="flex-1 p-3 bg-primary text-primary-foreground rounded-lg hover:bg-primary/90 transition-colors disabled:opacity-50"
            >
              {isLoading ? 'Processing...' : 'Deposit'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
} 
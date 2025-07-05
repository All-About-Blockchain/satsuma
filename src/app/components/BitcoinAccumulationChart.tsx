import React from 'react';

interface AccumulationData {
  date: string;
  bitcoinAmount: number;
  yieldConverted: number;
  portfolioValue: number;
}

interface BitcoinAccumulationChartProps {
  data: AccumulationData[];
  timeframe: '7d' | '30d' | '90d' | '1y';
  onTimeframeChange: (timeframe: '7d' | '30d' | '90d' | '1y') => void;
}

export default function BitcoinAccumulationChart({
  data,
  timeframe,
  onTimeframeChange
}: BitcoinAccumulationChartProps) {
  const timeframes = [
    { value: '7d', label: '7D' },
    { value: '30d', label: '30D' },
    { value: '90d', label: '90D' },
    { value: '1y', label: '1Y' }
  ] as const;

  return (
    <div className="bg-card p-6 rounded-lg border">
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-lg font-semibold">Bitcoin Accumulation</h3>
        <div className="flex space-x-1">
          {timeframes.map((tf) => (
            <button
              key={tf.value}
              onClick={() => onTimeframeChange(tf.value)}
              className={`px-3 py-1 rounded text-sm font-medium transition-colors ${
                timeframe === tf.value
                  ? 'bg-primary text-primary-foreground'
                  : 'bg-muted text-muted-foreground hover:bg-muted/80'
              }`}
            >
              {tf.label}
            </button>
          ))}
        </div>
      </div>

      {/* Simple chart representation */}
      <div className="space-y-4">
        {data.map((item, index) => (
          <div key={index} className="flex items-center justify-between p-3 bg-muted rounded">
            <div className="flex items-center space-x-3">
              <div className="w-3 h-3 bg-primary rounded-full"></div>
              <div>
                <div className="font-medium">{item.date}</div>
                <div className="text-sm text-muted-foreground">
                  +{item.bitcoinAmount.toFixed(8)} BTC
                </div>
              </div>
            </div>
            <div className="text-right">
              <div className="font-medium">${item.yieldConverted.toFixed(2)}</div>
              <div className="text-sm text-muted-foreground">
                Portfolio: ${item.portfolioValue.toLocaleString()}
              </div>
            </div>
          </div>
        ))}
      </div>

      <div className="mt-4 text-sm text-muted-foreground text-center">
        Chart shows Bitcoin accumulation from yield conversion over time
      </div>
    </div>
  );
} 
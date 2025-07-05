import React from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "./ui/card";
import { Badge } from "./ui/badge";
import { Button } from "./ui/button";
import { Progress } from "./ui/progress";
import { Separator } from "./ui/separator";
import { Wallet, Bitcoin, DollarSign, Plus, Minus } from "./ui/icons";

interface StablecoinHolding {
  symbol: string;
  amount: number;
  value: number;
  apy: number;
  dailyYield: number;
}

interface DashboardProps {
  stablecoinHoldings: StablecoinHolding[];
  totalBitcoinAccrued: number;
  bitcoinPrice: number;
  totalYieldGenerated: number;
  onDeposit: () => void;
  onWithdraw: () => void;
}

export default function Dashboard({
  stablecoinHoldings,
  totalBitcoinAccrued,
  bitcoinPrice,
  totalYieldGenerated,
  onDeposit,
  onWithdraw
}: DashboardProps) {
  const totalValue = stablecoinHoldings.reduce((sum, holding) => sum + holding.value, 0);
  const bitcoinValue = totalBitcoinAccrued * bitcoinPrice;
  const totalPortfolioValue = totalValue + bitcoinValue;
  const totalDailyYield = stablecoinHoldings.reduce((sum, holding) => sum + holding.dailyYield, 0);

  return (
    <div className="space-y-6">
      {/* Portfolio Overview */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Wallet className="h-5 w-5" />
            Portfolio Overview
          </CardTitle>
          <CardDescription>
            Your yield-bearing stablecoins automatically accumulating Bitcoin
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            <div className="space-y-2">
              <p className="text-sm text-muted-foreground">Total Portfolio Value</p>
              <p className="text-2xl font-bold">${totalPortfolioValue.toLocaleString('en-US', { minimumFractionDigits: 2 })}</p>
            </div>
            <div className="space-y-2">
              <p className="text-sm text-muted-foreground">Stablecoin Holdings</p>
              <p className="text-2xl font-bold">${totalValue.toLocaleString('en-US', { minimumFractionDigits: 2 })}</p>
            </div>
            <div className="space-y-2">
              <p className="text-sm text-muted-foreground">Bitcoin Accrued</p>
              <p className="text-2xl font-bold">{totalBitcoinAccrued.toFixed(8)} BTC</p>
              <p className="text-sm text-muted-foreground">${bitcoinValue.toLocaleString('en-US', { minimumFractionDigits: 2 })}</p>
            </div>
            <div className="space-y-2">
              <p className="text-sm text-muted-foreground">Daily Yield</p>
              <p className="text-2xl font-bold text-green-600">${totalDailyYield.toFixed(2)}</p>
            </div>
          </div>
          
          <Separator className="my-6" />
          
          <div className="flex gap-3">
            <Button onClick={onDeposit} className="flex items-center gap-2">
              <Plus className="h-4 w-4" />
              Deposit Stablecoins
            </Button>
            <Button variant="outline" onClick={onWithdraw} className="flex items-center gap-2">
              <Minus className="h-4 w-4" />
              Withdraw
            </Button>
          </div>
        </CardContent>
      </Card>

      {/* Stablecoin Holdings */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <DollarSign className="h-5 w-5" />
            Stablecoin Holdings
          </CardTitle>
          <CardDescription>
            Your yield-bearing deposits earning passive income
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            {stablecoinHoldings.map((holding) => (
              <div key={holding.symbol} className="flex items-center justify-between p-4 border rounded-lg hover:bg-muted/50 transition-colors">
                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 bg-primary/10 rounded-full flex items-center justify-center">
                    <span className="font-semibold text-primary">{holding.symbol}</span>
                  </div>
                  <div>
                    <p className="font-medium">{holding.amount.toLocaleString()} {holding.symbol}</p>
                    <p className="text-sm text-muted-foreground">${holding.value.toLocaleString('en-US', { minimumFractionDigits: 2 })}</p>
                  </div>
                </div>
                <div className="text-right">
                  <Badge variant="secondary" className="mb-1">
                    {holding.apy.toFixed(1)}% APY
                  </Badge>
                  <p className="text-sm text-green-600 font-medium">
                    +${holding.dailyYield.toFixed(2)}/day
                  </p>
                </div>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>

      {/* Bitcoin Accumulation */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Bitcoin className="h-5 w-5" />
            Bitcoin Accumulation
          </CardTitle>
          <CardDescription>
            Yield automatically converted to Bitcoin via yield skimming
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-muted-foreground">Current Bitcoin Holdings</p>
                <p className="text-2xl font-bold">{totalBitcoinAccrued.toFixed(8)} BTC</p>
              </div>
              <div className="text-right">
                <p className="text-sm text-muted-foreground">USD Value</p>
                <p className="text-2xl font-bold">${bitcoinValue.toLocaleString('en-US', { minimumFractionDigits: 2 })}</p>
              </div>
            </div>
            
            <div className="space-y-3">
              <div className="flex justify-between text-sm">
                <span className="text-muted-foreground">Daily Bitcoin Accumulation</span>
                <span className="text-green-600 font-medium">≈ 0.00001234 BTC</span>
              </div>
              <Progress value={75} className="h-2" />
              <p className="text-xs text-muted-foreground">
                Next yield skim in ~4 hours
              </p>
            </div>

            <div className="bg-muted/50 p-4 rounded-lg">
              <div className="flex items-center justify-between mb-2">
                <span className="text-sm font-medium">Total Yield Generated</span>
                <span className="text-lg font-bold text-green-600">
                  ${totalYieldGenerated.toLocaleString('en-US', { minimumFractionDigits: 2 })}
                </span>
              </div>
              <p className="text-xs text-muted-foreground">
                All yield has been automatically converted to Bitcoin
              </p>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
} 
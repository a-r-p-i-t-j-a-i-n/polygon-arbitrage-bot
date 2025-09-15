import React from 'react';
import { formatTime, formatCurrency } from '../utils/formatters';

const ArbitrageTable = ({ opportunities, loading }) => {
  console.log('ArbitrageTable re-render with', opportunities.length, 'opportunities');

  if (loading) {
    return (
      <div className="opportunities-section">
        <h2 className="section-title">
          Recent Arbitrage Opportunities
        </h2>
        <div className="loading">Loading opportunities...</div>
      </div>
    );
  }

  if (!opportunities || opportunities.length === 0) {
    return (
      <div className="opportunities-section">
        <h2 className="section-title">
          Recent Arbitrage Opportunities
        </h2>
        <div className="no-opportunities">
          <p>No arbitrage opportunities detected yet.</p>
          <p>The bot is actively monitoring for profitable trades...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="opportunities-section">
      <h2 className="section-title">
        Recent Arbitrage Opportunities ({opportunities.length})
      </h2>
      
      <div style={{ overflowX: 'auto' }}>
        <table className="opportunities-table">
          <thead>
            <tr>
              <th>Timestamp</th>
              <th>Buy Exchange</th>
              <th>Sell Exchange</th>
              <th>Buy Price</th>
              <th>Sell Price</th>
              <th>Est. Profit</th>
              <th>Profit %</th>
            </tr>
          </thead>
          <tbody>
            {opportunities.map((opp, index) => (
              <tr key={`${opp.id}-${index}`}>
                <td className="timestamp-cell">{formatTime(opp.timestamp || opp.created_at)}</td>
                <td className="exchange-cell">{opp.buy_exchange}</td>
                <td className="exchange-cell">{opp.sell_exchange}</td>
                <td>{formatCurrency(opp.buy_price)}</td>
                <td>{formatCurrency(opp.sell_price)}</td>
                <td className="profit-cell">{formatCurrency(opp.estimated_profit)}</td>
                <td className="profit-cell">{opp.profit_percentage?.toFixed(2)}%</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default ArbitrageTable;

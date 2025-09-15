import React from 'react';

const ConfigPanel = () => {
  const config = {
    tradeAmount: '1.0 WETH',
    minProfit: '5.0 USDC',
    gasEstimate: '1.0 USDC',
    checkInterval: '10 seconds',
    network: 'Polygon Mainnet',
    chainId: '137'
  };

  return (
    <div className="config-panel">
      <h2 className="section-title">
        Bot Configuration
      </h2>
      
      <div className="config-grid">
        <div className="config-item">
          <span className="config-label">Trade Amount</span>
          <span className="config-value">{config.tradeAmount}</span>
        </div>
        <div className="config-item">
          <span className="config-label">Min Profit Threshold</span>
          <span className="config-value">{config.minProfit}</span>
        </div>
        <div className="config-item">
          <span className="config-label">Gas Cost Estimate</span>
          <span className="config-value">{config.gasEstimate}</span>
        </div>
        <div className="config-item">
          <span className="config-label">Check Interval</span>
          <span className="config-value">{config.checkInterval}</span>
        </div>
        <div className="config-item">
          <span className="config-label">Network</span>
          <span className="config-value">{config.network}</span>
        </div>
        <div className="config-item">
          <span className="config-label">Chain ID</span>
          <span className="config-value">{config.chainId}</span>
        </div>
      </div>
    </div>
  );
};

export default ConfigPanel;

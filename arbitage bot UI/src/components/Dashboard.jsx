import React from 'react';
import StatusBar from './StatusBar';
import StatsCards from './StatsCards';
import ArbitrageTable from './ArbitrageTable';
import ConfigPanel from './ConfigPanel';
import useArbitrageData from '../hooks/useArbitrageData';

const Dashboard = () => {
  const { opportunities, stats, status, loading, error } = useArbitrageData();

  if (error) {
    return (
      <div className="error">
        <h3>⚠️ Error connecting to bot</h3>
        <p>{error}</p>
        <p>Make sure your Rust bot is running and accessible.</p>
      </div>
    );
  }

  return (
    <div className="dashboard">
      <StatusBar status={status} />
      <StatsCards stats={stats} loading={loading} />
      <ArbitrageTable opportunities={opportunities} loading={loading} />
      <ConfigPanel />
    </div>
  );
};

export default Dashboard;

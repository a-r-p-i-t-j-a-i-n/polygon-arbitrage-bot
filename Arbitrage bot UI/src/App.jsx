import React from 'react';
import Dashboard from './components/Dashboard';
import './styles.css';

function App() {
  return (
    <div className="app">
      <header className="header">
        <h1>Polygon Arbitrage Bot</h1>
        <p>Real-time arbitrage opportunities on QuickSwap vs SushiSwap</p>
      </header>
      <Dashboard />
    </div>
  );
}

export default App;

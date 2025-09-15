import React from 'react';

const StatusBar = ({ status }) => {
  return (
    <div className="status-bar">
      <div className="status-indicator">
        <div className="status-dot"></div>
        <span>Bot Status: <strong>{status?.running ? 'Running' : 'Stopped'}</strong></span>
      </div>
      <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
        <span>Last Check: {status?.last_check || 'Never'}</span>
      </div>
    </div>
  );
};

export default StatusBar;

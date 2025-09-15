import React from 'react';

const StatsCards = ({ stats, loading }) => {
  if (loading) {
    return (
      <div className="stats-grid">
        {[...Array(4)].map((_, i) => (
          <div key={i} className="stat-card">
            <div className="loading">Loading...</div>
          </div>
        ))}
      </div>
    );
  }

  const statCards = [
    {
      title: 'Total Opportunities',
      value: stats?.totalOpportunities || '0'
    },
    {
      title: 'Average Profit',
      value: `$${stats?.averageProfit || '0.00'}`
    },
    {
      title: 'Best Opportunity',
      value: `$${stats?.bestProfit || '0.00'}`
    },
    {
      title: 'Runtime',
      value: stats?.runtime || 'Active'
    }
  ];

  return (
    <div className="stats-grid">
      {statCards.map((stat, index) => (
        <div key={index} className="stat-card">
          <div className="stat-title">{stat.title}</div>
          <div className="stat-value">{stat.value}</div>
        </div>
      ))}
    </div>
  );
};

export default StatsCards;

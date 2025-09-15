import { useState, useEffect } from 'react';
import axios from 'axios';

const API_BASE_URL = 'http://127.0.0.1:8081/api'; // Changed from 8080 to 8081

const useArbitrageData = () => {
  const [opportunities, setOpportunities] = useState([]);
  const [stats, setStats] = useState({});
  const [status, setStatus] = useState({ running: false, last_check: 'Never' });
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    const loadData = async () => {
      setLoading(true);
      setError(null);

      try {
        console.log('Fetching data from API at:', API_BASE_URL);
        
        const [opportunitiesRes, statsRes, statusRes] = await Promise.all([
          axios.get(`${API_BASE_URL}/opportunities?t=${Date.now()}`),
          axios.get(`${API_BASE_URL}/stats`),
          axios.get(`${API_BASE_URL}/status`)
        ]);

        console.log('✅ API Response received:', opportunitiesRes.data);

        if (opportunitiesRes.data.success) {
          const newOpportunities = opportunitiesRes.data.data || [];
          console.log('💰 New opportunities count:', newOpportunities.length);
          setOpportunities(newOpportunities);
        }

        if (statsRes.data.success) {
          const statsData = statsRes.data.data;
          setStats({
            totalOpportunities: statsData.total_opportunities,
            averageProfit: (statsData.average_profit || 0).toFixed(2),
            bestProfit: (statsData.best_profit || 0).toFixed(2),
            runtime: statsData.runtime || 'Active'
          });
        }

        if (statusRes.data.success) {
          setStatus({
            running: statusRes.data.data.running,
            last_check: statusRes.data.data.last_check || 'Just now'
          });
        }

        setLoading(false);
      } catch (err) {
        console.error('❌ Error fetching data:', err);
        setError(`Failed to connect to arbitrage bot at ${API_BASE_URL}. Make sure it's running.`);
        setLoading(false);
      }
    };

    loadData();
    
    const interval = setInterval(() => {
      console.log('⏰ Polling for new arbitrage opportunities...');
      loadData();
    }, 5000);
    
    return () => clearInterval(interval);
  }, []);

  return { opportunities, stats, status, loading, error };
};

export default useArbitrageData;

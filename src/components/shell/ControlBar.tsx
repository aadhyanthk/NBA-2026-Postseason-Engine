import React from 'react';
import { useEngineStore } from '../../store/engineStore';

export const ControlBar: React.FC = () => {
  const { config, status, setConfig, runSimulation, verifyDeterminism } = useEngineStore();

  const isRunning = status === 'running' || status === 'verifying';

  return (
    <div className="panel flex justify-between items-center w-full" style={{ borderTop: 0 }}>
      <div className="flex gap-4 items-center">
        <div className="flex-col gap-2">
          <label className="mono text-secondary" style={{ fontSize: '0.75rem' }}>YEAR</label>
          <select 
            value={config.year} 
            onChange={(e) => setConfig('year', parseInt(e.target.value))}
            disabled={isRunning}
            style={{ width: '90px' }}
          >
            {[2016, 2017, 2018, 2019, 2020, 2021, 2022, 2023, 2024, 2025, 2026].map(y => (
              <option key={y} value={y}>{y}</option>
            ))}
          </select>
        </div>

        <div className="flex-col gap-2">
          <label className="mono text-secondary" style={{ fontSize: '0.75rem' }}>SEED</label>
          <input 
            type="number" 
            value={config.seed} 
            onChange={(e) => setConfig('seed', parseInt(e.target.value))} 
            disabled={isRunning}
            style={{ width: '120px' }}
          />
        </div>

        <div className="flex-col gap-2">
          <label className="mono text-secondary" style={{ fontSize: '0.75rem' }}>SIMULATIONS</label>
          <input 
            type="number" 
            value={config.simulations} 
            onChange={(e) => setConfig('simulations', parseInt(e.target.value))} 
            disabled={isRunning}
            style={{ width: '150px' }}
          />
        </div>

        <div className="flex-col gap-2">
          <label className="mono text-secondary" style={{ fontSize: '0.75rem' }}>THREADS</label>
          <select 
            value={config.threads} 
            onChange={(e) => setConfig('threads', parseInt(e.target.value))}
            disabled={isRunning}
            style={{ width: '100px' }}
          >
            {[1, 2, 4, 8, 16, 32].map(t => (
              <option key={t} value={t}>{t}</option>
            ))}
          </select>
        </div>

        <div className="flex-col gap-2">
          <label className="mono text-secondary" style={{ fontSize: '0.75rem' }}>SCHEDULER</label>
          <select 
            value={config.scheduler} 
            onChange={(e) => setConfig('scheduler', e.target.value)}
            disabled={isRunning}
            style={{ width: '120px' }}
          >
            <option value="rayon">Rayon</option>
            <option value="custom">Chase-Lev</option>
          </select>
        </div>

        <div className="flex-col gap-2" style={{ paddingLeft: '1rem', paddingTop: '1.25rem' }}>
          <label className="mono text-secondary flex items-center gap-2" style={{ fontSize: '0.75rem', cursor: 'pointer' }}>
            <input 
              type="checkbox" 
              checked={config.pinThreads}
              onChange={(e) => setConfig('pinThreads', e.target.checked)}
              disabled={isRunning}
            />
            PIN THREADS
          </label>
        </div>
      </div>

      <div className="flex gap-4 items-center">
        <button 
          onClick={verifyDeterminism} 
          disabled={isRunning}
          style={{ fontFamily: 'var(--font-mono)' }}
        >
          [ VERIFY DETERMINISM ]
        </button>
        <button 
          className="primary" 
          onClick={runSimulation} 
          disabled={isRunning}
          style={{ fontFamily: 'var(--font-mono)' }}
        >
          [ RUN SIMULATION ]
        </button>
      </div>
    </div>
  );
};

import React from 'react';
import { useEngineStore } from '../../store/engineStore';

export const PerformancePanel: React.FC = () => {
  const { determinismResult, telemetry, simulationResult, config } = useEngineStore();

  const renderContent = () => {
    if (determinismResult) {
      // Use determinism data for scaling analysis
      const baseline = determinismResult.runs.find(r => r.threads === 1)?.elapsed_ns || 1;
      
      return (
        <table style={{ width: '100%', textAlign: 'left', borderCollapse: 'collapse', marginTop: '1rem' }}>
          <thead>
            <tr style={{ borderBottom: '1px solid var(--border)', color: 'var(--text-secondary)' }}>
              <th className="mono font-normal pb-2">THREADS</th>
              <th className="mono font-normal pb-2" style={{ textAlign: 'right' }}>TIME (ms)</th>
              <th className="mono font-normal pb-2" style={{ textAlign: 'right' }}>SPEEDUP</th>
              <th className="mono font-normal pb-2" style={{ textAlign: 'right' }}>EFFICIENCY</th>
            </tr>
          </thead>
          <tbody>
            {determinismResult.runs.map((run, i) => {
              const speedup = baseline / run.elapsed_ns;
              const efficiency = (speedup / run.threads) * 100;
              const timeMs = run.elapsed_ns / 1_000_000;
              
              return (
                <tr key={i} style={{ borderBottom: '1px solid var(--border-subtle)' }}>
                  <td className="mono py-2">{run.threads}</td>
                  <td className="mono py-2" style={{ textAlign: 'right' }}>
                    {timeMs.toLocaleString(undefined, { maximumFractionDigits: 1 })}
                  </td>
                  <td className="mono py-2" style={{ textAlign: 'right', color: 'var(--accent)' }}>
                    {speedup.toFixed(2)}x
                  </td>
                  <td className="mono py-2" style={{ textAlign: 'right', color: efficiency > 80 ? 'var(--success)' : 'var(--text-primary)' }}>
                    {efficiency.toFixed(1)}%
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      );
    }

    if (simulationResult && telemetry) {
       // Just show latest telemetry metrics for single run
       const elapsedSecs = telemetry.elapsed_ns / 1_000_000_000;
       
       return (
         <div className="flex-col gap-4 mt-4">
           <div className="flex justify-between" style={{ borderBottom: '1px solid var(--border-subtle)', paddingBottom: '0.5rem' }}>
             <span className="mono text-secondary">FINAL THROUGHPUT</span>
             <span className="mono text-success">{telemetry.games_per_second.toLocaleString(undefined, { maximumFractionDigits: 0 })} games/s</span>
           </div>
           <div className="flex justify-between" style={{ borderBottom: '1px solid var(--border-subtle)', paddingBottom: '0.5rem' }}>
             <span className="mono text-secondary">WALL TIME</span>
             <span className="mono">{elapsedSecs.toFixed(3)}s</span>
           </div>
           <div className="flex justify-between" style={{ borderBottom: '1px solid var(--border-subtle)', paddingBottom: '0.5rem' }}>
             <span className="mono text-secondary">WORKER THREADS</span>
             <span className="mono">{config.threads} ({config.pinThreads ? 'PINNED' : 'UNPINNED'})</span>
           </div>
           <div className="flex justify-between" style={{ borderBottom: '1px solid var(--border-subtle)', paddingBottom: '0.5rem' }}>
             <span className="mono text-secondary">TOTAL GAMES</span>
             <span className="mono">{simulationResult.total_games.toLocaleString()}</span>
           </div>
         </div>
       );
    }

    return (
      <div className="flex justify-center items-center h-full text-muted mono">
        AWAITING BENCHMARK / RUN DATA
      </div>
    );
  };

  return (
    <div className="h-full flex-col">
      <div className="mono text-secondary" style={{ marginBottom: '1rem' }}>
        PERFORMANCE METRICS
      </div>
      
      {renderContent()}
    </div>
  );
};

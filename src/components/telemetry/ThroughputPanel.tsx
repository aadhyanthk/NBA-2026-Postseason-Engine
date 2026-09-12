import React from 'react';
import { useEngineStore } from '../../store/engineStore';

export const ThroughputPanel: React.FC = () => {
  const { status, telemetry } = useEngineStore();

  const isRunning = status === 'running';

  return (
    <div className="h-full flex-col">
      <div className="mono text-secondary" style={{ marginBottom: '1rem' }}>
        LIVE THROUGHPUT
      </div>

      {!isRunning && !telemetry && (
        <div className="flex justify-center items-center h-full text-muted mono">
          NO RUN DATA
        </div>
      )}

      {telemetry && (
        <div className="flex-col gap-4">
          <div className="flex justify-between items-center pb-2" style={{ borderBottom: '1px solid var(--border)' }}>
            <div className="flex-col gap-1">
              <div className="mono text-secondary" style={{ fontSize: '0.75rem' }}>THROUGHPUT</div>
              <div className="mono" style={{ fontSize: '1.25rem' }}>
                {telemetry.games_per_second.toLocaleString(undefined, { maximumFractionDigits: 0 })} <span className="text-muted" style={{ fontSize: '0.875rem' }}>games/s</span>
              </div>
            </div>
            
            <div className="flex-col gap-1 items-end">
              <div className="mono text-secondary" style={{ fontSize: '0.75rem' }}>COMPLETED</div>
              <div className="mono" style={{ fontSize: '1.25rem' }}>
                {telemetry.completed.toLocaleString()} / <span className="text-muted">{telemetry.total.toLocaleString()}</span>
              </div>
            </div>
          </div>

          <div className="flex justify-between items-center">
             <div className="flex-col gap-1">
                <div className="mono text-secondary" style={{ fontSize: '0.75rem' }}>ELAPSED</div>
                <div className="mono">
                  {(telemetry.elapsed_ns / 1_000_000_000).toFixed(3)}s
                </div>
             </div>
             <div className="flex-col gap-1 items-end">
                <div className="mono text-secondary" style={{ fontSize: '0.75rem' }}>WORKERS</div>
                <div className="mono">
                  {telemetry.worker_utilization.length}
                </div>
             </div>
          </div>

          <div className="mt-4">
             <div className="mono text-secondary" style={{ fontSize: '0.75rem', marginBottom: '0.5rem' }}>WORKER UTILIZATION</div>
             <div className="flex-col gap-1" style={{ maxHeight: '120px', overflowY: 'auto' }}>
                {telemetry.worker_utilization.map((util, i) => (
                  <div key={i} className="flex items-center gap-2 mono" style={{ fontSize: '0.75rem' }}>
                    <div style={{ width: '24px' }}>{(i + 1).toString().padStart(2, '0')}</div>
                    <div style={{ flex: 1, backgroundColor: 'var(--bg-primary)', height: '8px' }}>
                       <div style={{ width: `${util * 100}%`, backgroundColor: 'var(--accent)', height: '100%' }}></div>
                    </div>
                    <div style={{ width: '36px', textAlign: 'right' }}>{(util * 100).toFixed(0)}%</div>
                  </div>
                ))}
             </div>
          </div>
        </div>
      )}
    </div>
  );
};

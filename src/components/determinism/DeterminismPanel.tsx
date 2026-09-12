import React from 'react';
import { useEngineStore } from '../../store/engineStore';

export const DeterminismPanel: React.FC = () => {
  const { status, determinismResult } = useEngineStore();

  return (
    <div className="h-full flex-col">
      <div className="mono text-secondary" style={{ marginBottom: '1rem' }}>
        DETERMINISM VERIFICATION
      </div>

      {status === 'verifying' && !determinismResult && (
        <div className="flex justify-center items-center h-full text-muted mono">
          VERIFYING ACROSS MULTIPLE THREAD COUNTS...
        </div>
      )}

      {status === 'idle' && !determinismResult && (
        <div className="flex justify-center items-center h-full text-muted mono">
          NO RUN DATA
        </div>
      )}

      {determinismResult && (
        <div className="flex-col gap-4">
          <div className="flex gap-4">
            <div className="mono" style={{ fontSize: '0.875rem' }}>
              <span className="text-secondary">SEED: </span>
              {determinismResult.seed}
            </div>
            <div className="mono" style={{ fontSize: '0.875rem' }}>
              <span className="text-secondary">SIMULATIONS: </span>
              {determinismResult.simulations.toLocaleString()}
            </div>
          </div>

          <table style={{ width: '100%', textAlign: 'left', borderCollapse: 'collapse', marginTop: '1rem' }}>
            <thead>
              <tr style={{ borderBottom: '1px solid var(--border)', color: 'var(--text-secondary)' }}>
                <th className="mono font-normal pb-2" style={{ width: '80px' }}>THREADS</th>
                <th className="mono font-normal pb-2">SHA-256 DIGEST</th>
                <th className="mono font-normal pb-2" style={{ width: '120px' }}>STATUS</th>
              </tr>
            </thead>
            <tbody>
              {determinismResult.runs.map((run, i) => (
                <tr key={i} style={{ borderBottom: '1px solid var(--border-subtle)' }}>
                  <td className="mono py-2">{run.threads}</td>
                  <td className="mono py-2 text-muted" style={{ fontSize: '0.875rem' }}>
                    {run.digest}
                  </td>
                  <td className="mono py-2">
                    {run.match_status ? (
                      <span className="text-success">✓ MATCH</span>
                    ) : (
                      <span className="text-error">✗ MISMATCH</span>
                    )}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>

          <div className="mt-4 p-4 mono" style={{ backgroundColor: 'var(--bg-primary)', border: '1px solid var(--border)' }}>
            <div style={{ fontSize: '0.75rem', color: 'var(--text-secondary)', marginBottom: '0.5rem' }}>RESULT</div>
            {determinismResult.all_match ? (
              <div className="text-success" style={{ fontWeight: 'bold' }}>
                ✓ ALL CONFIGURATIONS PRODUCED IDENTICAL OUTPUT
              </div>
            ) : (
              <div className="text-error" style={{ fontWeight: 'bold' }}>
                ✗ DETERMINISM FAILURE
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
};

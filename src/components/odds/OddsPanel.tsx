import React, { useEffect, useState } from 'react';
import { useEngineStore } from '../../store/engineStore';
import { invoke } from '@tauri-apps/api/core';

interface Team {
  id: number;
  name: string;
  conf: 'East' | 'West';
  seed: number;
}

export const OddsPanel: React.FC = () => {
  const { status, simulationResult, config } = useEngineStore();
  const [teams, setTeams] = useState<Team[]>([]);

  useEffect(() => {
    invoke<Team[]>('get_teams').then(setTeams).catch(console.error);
  }, []);

  const isIdle = status === 'idle' || status === 'running' || status === 'verifying';

  if (isIdle && !simulationResult) {
    return (
      <div className="h-full flex-col">
        <div className="mono text-secondary" style={{ marginBottom: '1rem' }}>
          CHAMPIONSHIP ODDS
        </div>
        <div className="flex justify-center items-center h-full text-muted mono">
          WAITING FOR SIMULATION RUN
        </div>
      </div>
    );
  }

  // Calculate odds
  const totalSims = simulationResult ? simulationResult.total_games > 0 ? config.simulations : 1 : 1;
  
  const teamOdds = teams
    .filter(t => t.seed > 0)
    .map(team => {
      const idx = team.id;
      const playIn = simulationResult?.play_in[idx] || 0;
      const playoffs = simulationResult?.playoffs[idx] || 0;
      const confFinals = simulationResult?.conf_finals[idx] || 0;
      const finals = simulationResult?.finals[idx] || 0;
      const championships = simulationResult?.championships[idx] || 0;

      return {
        ...team,
        playInPct: (playIn / totalSims) * 100,
        playoffsPct: (playoffs / totalSims) * 100,
        confFinalsPct: (confFinals / totalSims) * 100,
        finalsPct: (finals / totalSims) * 100,
        champPct: (championships / totalSims) * 100,
      };
    })
    .sort((a, b) => b.champPct - a.champPct); // Sort by highest championship %

  return (
    <div className="h-full flex-col">
      <div className="flex justify-between items-center" style={{ marginBottom: '1rem' }}>
        <div className="mono text-secondary">
          CHAMPIONSHIP ODDS
        </div>
        <div className="mono text-muted" style={{ fontSize: '0.75rem' }}>
          n={totalSims.toLocaleString()}
        </div>
      </div>
      
      <div style={{ flex: 1, overflowY: 'auto', border: '1px solid var(--border-subtle)' }}>
        <table style={{ width: '100%', textAlign: 'left', borderCollapse: 'collapse' }}>
          <thead style={{ position: 'sticky', top: 0, backgroundColor: 'var(--bg-panel)' }}>
            <tr style={{ borderBottom: '1px solid var(--border)', color: 'var(--text-secondary)', fontSize: '0.75rem' }}>
              <th className="mono font-normal p-2">TEAM</th>
              <th className="mono font-normal p-2" style={{ textAlign: 'right' }}>PLAY-IN</th>
              <th className="mono font-normal p-2" style={{ textAlign: 'right' }}>PLAYOFFS</th>
              <th className="mono font-normal p-2" style={{ textAlign: 'right' }}>CONF FINALS</th>
              <th className="mono font-normal p-2" style={{ textAlign: 'right' }}>FINALS</th>
              <th className="mono font-normal p-2" style={{ textAlign: 'right' }}>CHAMPION</th>
            </tr>
          </thead>
          <tbody>
            {teamOdds.map((team) => (
              <tr key={team.id} style={{ borderBottom: '1px solid var(--border-subtle)' }}>
                <td className="p-2">
                  <div className="flex items-center gap-2">
                    <span className="mono" style={{ fontSize: '0.75rem', color: 'var(--text-secondary)' }}>
                      {team.conf.substring(0, 1)}{team.seed}
                    </span>
                    <span className="mono">{team.name}</span>
                  </div>
                </td>
                <td className="mono p-2" style={{ textAlign: 'right', color: team.playInPct > 0 ? 'var(--text-primary)' : 'var(--text-muted)' }}>
                  {team.playInPct.toFixed(1)}%
                </td>
                <td className="mono p-2" style={{ textAlign: 'right', color: team.playoffsPct > 0 ? 'var(--text-primary)' : 'var(--text-muted)' }}>
                  {team.playoffsPct.toFixed(1)}%
                </td>
                <td className="mono p-2" style={{ textAlign: 'right', color: team.confFinalsPct > 0 ? 'var(--text-primary)' : 'var(--text-muted)' }}>
                  {team.confFinalsPct.toFixed(1)}%
                </td>
                <td className="mono p-2" style={{ textAlign: 'right', color: team.finalsPct > 0 ? 'var(--text-primary)' : 'var(--text-muted)' }}>
                  {team.finalsPct.toFixed(1)}%
                </td>
                <td className="mono p-2" style={{ textAlign: 'right', color: team.champPct > 0 ? 'var(--accent)' : 'var(--text-muted)', fontWeight: team.champPct > 0 ? 'bold' : 'normal' }}>
                  {team.champPct.toFixed(1)}%
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};

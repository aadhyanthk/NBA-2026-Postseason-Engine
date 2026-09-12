import React from 'react';

export const Header: React.FC = () => {
  return (
    <header className="flex justify-between items-center p-4" style={{
      borderBottom: '1px solid var(--border)',
      backgroundColor: 'var(--bg-secondary)'
    }}>
      <div className="flex items-center gap-4">
        <h1 className="mono" style={{ fontSize: '1rem', color: 'var(--text-primary)' }}>
          BASKETBALL SIMULATION ENGINE
        </h1>
      </div>
      <div className="flex items-center gap-4 mono" style={{ fontSize: '0.875rem' }}>
        <div style={{ color: 'var(--text-secondary)' }}>
          ENGINE: <span className="text-success">READY</span>
        </div>
        <div className="text-muted">v0.1.0 (x86_64 native)</div>
      </div>
    </header>
  );
};

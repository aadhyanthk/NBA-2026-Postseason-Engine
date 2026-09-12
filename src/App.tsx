import React from 'react';
import { Header } from './components/shell/Header';
import { ControlBar } from './components/shell/ControlBar';
import { DeterminismPanel } from './components/determinism/DeterminismPanel';
import { ThroughputPanel } from './components/telemetry/ThroughputPanel';
import { EngineLog } from './components/logs/EngineLog';
import { OddsPanel } from './components/odds/OddsPanel';
import { PerformancePanel } from './components/benchmarks/PerformancePanel';

function App() {
  return (
    <div className="h-full flex-col">
      <Header />
      <main className="p-4 flex-col gap-4" style={{ flex: 1, overflowY: 'auto' }}>
        <ControlBar />
        <div className="flex gap-4" style={{ height: '300px' }}>
          <div className="panel" style={{ flex: 1 }}>
            <ThroughputPanel />
          </div>
          <div className="panel" style={{ flex: 1 }}>
            <DeterminismPanel />
          </div>
        </div>
        <div className="flex gap-4" style={{ height: '300px' }}>
          <div className="panel" style={{ flex: 1 }}>
            <OddsPanel />
          </div>
          <div className="panel" style={{ flex: 1 }}>
            <PerformancePanel />
          </div>
        </div>
        <div className="panel" style={{ height: '200px' }}>
          <EngineLog />
        </div>
      </main>
    </div>
  );
}

export default App;

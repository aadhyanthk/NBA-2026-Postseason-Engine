import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export type EngineStatus = 'idle' | 'running' | 'verifying' | 'complete' | 'error';

export interface TelemetrySnapshot {
  completed: number;
  total: number;
  games_per_second: number;
  elapsed_ns: number;
  worker_utilization: number[];
}

export interface DeterminismRun {
  threads: number;
  digest: string;
  elapsed_ns: number;
  match_status: boolean;
}

export interface DeterminismResult {
  seed: number;
  simulations: number;
  runs: DeterminismRun[];
  all_match: boolean;
}

export interface SimAccumulator {
  play_in: number[];
  playoffs: number[];
  conf_finals: number[];
  finals: number[];
  championships: number[];
  total_games: number;
}

export interface LogEntry {
  timestamp: string;
  message: string;
}

interface EngineState {
  status: EngineStatus;
  config: {
    seed: number;
    simulations: number;
    threads: number;
    scheduler: string;
    pinThreads: boolean;
    year: number;
  };
  telemetry: TelemetrySnapshot | null;
  determinismResult: DeterminismResult | null;
  simulationResult: SimAccumulator | null;
  logs: LogEntry[];
  error: string | null;

  // Actions
  setConfig: (key: keyof EngineState['config'], value: number | string | boolean) => void;
  runSimulation: () => Promise<void>;
  verifyDeterminism: () => Promise<void>;
  addLog: (message: string) => void;
}

export const useEngineStore = create<EngineState>((set, get) => ({
  status: 'idle',
  config: {
    seed: 42,
    simulations: 100000,
    threads: 8,
    scheduler: 'rayon',
    pinThreads: false,
    year: 2026,
  },
  telemetry: null,
  determinismResult: null,
  simulationResult: null,
  logs: [],
  error: null,

  setConfig: (key, value) => set((state) => ({
    config: { ...state.config, [key]: value }
  })),

  addLog: (message) => set((state) => {
    const entry = { timestamp: new Date().toISOString(), message };
    return { logs: [...state.logs, entry].slice(-1000) }; // Keep last 1000
  }),

  runSimulation: async () => {
    const { config, addLog } = get();
    set({ status: 'running', error: null, simulationResult: null, telemetry: null });
    addLog(`RUN seed=${config.seed} sims=${config.simulations} threads=${config.threads}`);

    try {
      const result: SimAccumulator = await invoke('run_simulation', {
        seed: config.seed,
        simulations: config.simulations,
        threads: config.threads,
        scheduler: config.scheduler,
        pinThreads: config.pinThreads,
        year: config.year,
      });

      set({ status: 'complete', simulationResult: result });
    } catch (e: any) {
      set({ status: 'error', error: e.toString() });
      addLog(`ERROR: ${e.toString()}`);
    }
  },

  verifyDeterminism: async () => {
    const { config, addLog } = get();
    set({ status: 'verifying', error: null, determinismResult: null });
    addLog(`VERIFY DETERMINISM seed=${config.seed} sims=${config.simulations}`);

    try {
      const threadCounts = [1, 2, 4, 8, 16, 32].filter(t => t <= 32); // Adjust based on physical system max
      const result: DeterminismResult = await invoke('verify_determinism', {
        seed: config.seed,
        simulations: config.simulations,
        threadCounts,
        year: config.year,
      });

      set({ status: 'complete', determinismResult: result });
      addLog(`VERIFICATION COMPLETE: all_match=${result.all_match}`);
    } catch (e: any) {
      set({ status: 'error', error: e.toString() });
      addLog(`ERROR: ${e.toString()}`);
    }
  },
}));

// Setup Tauri event listeners
if (typeof window !== 'undefined') {
  listen<TelemetrySnapshot>('simulation://progress', (event) => {
    useEngineStore.setState({ telemetry: event.payload });
  });

  listen<string>('simulation://log', (event) => {
    useEngineStore.getState().addLog(event.payload);
  });
}

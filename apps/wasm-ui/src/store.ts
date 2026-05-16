import { create } from 'zustand';

interface SuperAlignState {
  isWasmLoaded: boolean;
  setWasmLoaded: (loaded: boolean) => void;
  recentRuns: any[];
  addRun: (run: any) => void;
}

export const useStore = create<SuperAlignState>((set) => ({
  isWasmLoaded: false,
  setWasmLoaded: (loaded) => set({ isWasmLoaded: loaded }),
  recentRuns: [],
  addRun: (run) => set((state) => ({ recentRuns: [run, ...state.recentRuns] })),
}));

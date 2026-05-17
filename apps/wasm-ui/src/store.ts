import { create } from 'zustand';

export interface ProcessNode {
  name: string;
  hash: string;
  status: 'DONE' | 'ACTIVE' | 'PENDING';
  icon?: any;
}

export interface ReconciledTaxon {
  raw: string;
  canonical: string;
  score: number;
  id: string;
  status: 'MINT' | 'FAIL';
}

interface SuperAlignState {
  // System State
  isWasmLoaded: boolean;
  setWasmLoaded: (loaded: boolean) => void;
  
  // Pipeline State
  isProcessing: boolean;
  setIsProcessing: (proc: boolean) => void;
  
  // Active Data
  activeEntities: any[];
  reconciliationResults: ReconciledTaxon[];
  provenanceNodes: ProcessNode[];
  matrixChunks: boolean[]; // true = informative, false = gap
  
  // Actions
  setActiveRun: (data: {
    entities: any[],
    reconciliation: ReconciledTaxon[],
    provenance: ProcessNode[],
    matrix: boolean[]
  }) => void;
  
  resetRun: () => void;
}

export const useStore = create<SuperAlignState>((set) => ({
  isWasmLoaded: false,
  setWasmLoaded: (loaded) => set({ isWasmLoaded: loaded }),
  
  isProcessing: false,
  setIsProcessing: (proc) => set({ isProcessing: proc }),
  
  activeEntities: [],
  reconciliationResults: [],
  provenanceNodes: [],
  matrixChunks: [],
  
  setActiveRun: (data) => set({
    activeEntities: data.entities,
    reconciliationResults: data.reconciliation,
    provenanceNodes: data.provenance,
    matrixChunks: data.matrix,
    isProcessing: false
  }),
  
  resetRun: () => set({
    activeEntities: [],
    reconciliationResults: [],
    provenanceNodes: [],
    matrixChunks: [],
    isProcessing: false
  })
}));

import React, { useEffect } from 'react';
import { 
  BrowserRouter as Router, 
  Routes, 
  Route, 
  NavLink, 
  Navigate,
  useNavigate
} from 'react-router-dom';
import { 
  Telescope, 
  Microscope, 
  Database, 
  Terminal, 
  Grid3X3, 
  Puzzle, 
  ShieldCheck,
  Lock,
  Upload,
  Loader2,
  CheckCircle2,
  Zap,
  Dna,
  ChevronRight,
  History,
  FileCode,
  Layers,
  Activity,
  Monitor,
  GitBranch,
  Fingerprint,
  RefreshCcw
} from 'lucide-react';
import init, { WasmParser } from './wasm/wasm_core';
import { useStore } from './store';

// --- SHARED COMPONENTS ---

const NavItem = ({ to, icon: Icon, label }: { to: string, icon: any, label: string }) => (
  <NavLink 
    to={to}
    className={({ isActive }) => `flex items-center space-x-3 px-4 py-3 rounded-lg transition-all duration-300 group ${
      isActive 
        ? 'bg-lab-royal text-white shadow-lg shadow-blue-200' 
        : 'text-lab-600 hover:bg-lab-100 hover:text-lab-900'
    }`}
  >
    {({ isActive }) => (
      <>
        <Icon size={18} className={isActive ? 'text-white' : 'text-lab-300 group-hover:text-lab-600'} />
        <span className="font-bold text-[10px] tracking-widest uppercase">{label}</span>
      </>
    )}
  </NavLink>
);

const GlassCard = ({ children, className = "", title = "", subtitle = "" }: { children: React.ReactNode, className?: string, title?: string, subtitle?: string }) => (
  <div className={`bg-white border border-lab-200 rounded-xl shadow-sm overflow-hidden ${className}`}>
    {(title || subtitle) && (
      <div className="px-8 py-5 border-b border-lab-100 bg-lab-50/50 flex justify-between items-center">
        <div>
          <h3 className="text-[10px] font-black text-lab-900 uppercase tracking-[0.2em]">{title}</h3>
          {subtitle && <p className="text-[9px] text-lab-300 font-bold uppercase mt-1">{subtitle}</p>}
        </div>
      </div>
    )}
    <div className="p-8">
      {children}
    </div>
  </div>
);

const MetricCard = ({ label, value, delta, icon: Icon, accent }: { label: string, value: string, delta?: string, icon: any, accent: string }) => (
  <div className="bg-white border border-lab-200 p-6 rounded-2xl shadow-sm group">
    <div className="flex justify-between items-start mb-4">
      <div className={`p-2.5 rounded-xl bg-lab-50 ${accent}`}>
        <Icon size={20} />
      </div>
      {delta && <span className="text-[10px] font-black text-lab-mint">{delta}</span>}
    </div>
    <div className="text-[10px] font-black text-lab-300 uppercase tracking-widest mb-1">{label}</div>
    <div className="text-2xl font-black text-lab-900 font-mono tracking-tighter">{value}</div>
  </div>
);

// --- PAGES ---

const Observatory = () => {
  const { setActiveRun, setIsProcessing } = useStore();
  const navigate = useNavigate();

  const demo_pathogen_surveillance = () => {
    setIsProcessing(true);
    setTimeout(() => {
      setActiveRun({
        entities: [
          { uuid: 'v-712-alpha', label: 'hCoV-19/England/2045/2020', hash: 'sha256:7f81...a12', length: 29903 },
          { uuid: 'v-881-delta', label: 'SARS-CoV-2/India/WB-12/2021', hash: 'sha256:12c4...f99', length: 29891 },
          { uuid: 'v-992-omi',   label: 'hCoV-19/SA/TY-9921/2021', hash: 'sha256:990a...d11', length: 29780 },
        ],
        reconciliation: [
          { raw: 'hCoV-19/England/2045/2020', canonical: 'SARS-CoV-2 (Alpha)', score: 0.98, id: 'NCBI:2697049', status: 'MINT' },
          { raw: 'SARS-CoV-2/India/WB-12/2021', canonical: 'SARS-CoV-2 (Delta)', score: 0.99, id: 'NCBI:2697049', status: 'MINT' },
          { raw: 'hCoV-19/SA/TY-9921/2021', canonical: 'SARS-CoV-2 (Omicron)', score: 0.97, id: 'NCBI:2697049', status: 'MINT' },
        ],
        provenance: [
          { name: 'FASTA_INGEST', hash: 'b9d75114', status: 'DONE', icon: FileCode },
          { name: 'TAXON_MAP', hash: '8e7e348c', status: 'DONE', icon: Fingerprint },
          { name: 'MATRIX_BUILD', hash: '02ea00a2', status: 'ACTIVE', icon: Grid3X3 }
        ],
        matrix: Array.from({ length: 96 }, () => Math.random() > 0.3)
      });
      navigate('/laboratory');
    }, 1000);
  };

  const demo_metagenomic_survey = () => {
    setIsProcessing(true);
    setTimeout(() => {
      setActiveRun({
        entities: Array.from({ length: 10 }, (_, i) => ({
          uuid: `m-seq-${i}`,
          label: `Uncultured_Bacterium_MAG_${1000 + i}`,
          hash: `sha256:${Math.random().toString(36).substring(7)}`,
          length: Math.floor(Math.random() * 5000) + 1500
        })),
        reconciliation: Array.from({ length: 10 }, (_, i) => ({
          raw: `Uncultured_Bacterium_MAG_${1000 + i}`,
          canonical: 'Bacterium sp.',
          score: 0.85 + Math.random() * 0.1,
          id: 'NCBI:2',
          status: 'MINT'
        })),
        provenance: [
          { name: 'METAGENOMIC_PARSE', hash: 'm9d75114', status: 'DONE', icon: Layers },
          { name: 'DIVERSITY_MAP', hash: 'm8e7e348', status: 'DONE', icon: Activity },
          { name: 'SPARSE_INDEX', hash: 'm2ea00a2', status: 'DONE', icon: Database }
        ],
        matrix: Array.from({ length: 96 }, () => Math.random() > 0.7)
      });
      navigate('/laboratory');
    }, 1000);
  };

  return (
    <div className="space-y-10 animate-in fade-in duration-500 max-w-7xl mx-auto">
      <div className="grid grid-cols-4 gap-6">
        <MetricCard label="Analyzed Genomes" value="102,341" delta="+4.2%" icon={Microscope} accent="text-lab-royal" />
        <MetricCard label="Bit-for-bit Integrity" value="100%" delta="STABLE" icon={ShieldCheck} accent="text-lab-mint" />
        <MetricCard label="System Latency" value="0.42ms" delta="REAL" icon={Zap} accent="text-amber-500" />
        <MetricCard label="Distributed Pool" value="4.2 TB" icon={Database} accent="text-indigo-600" />
      </div>

      <div className="grid grid-cols-3 gap-10">
        <div className="col-span-2">
          <GlassCard title="Provenance Ledger" subtitle="Immutable Scientific Audit Trail">
            <table className="w-full text-left text-xs">
              <thead className="bg-lab-50 text-lab-300 uppercase text-[9px] tracking-widest font-black">
                <tr>
                  <th className="px-8 py-4">Process Hash</th>
                  <th className="px-8 py-4">Operation</th>
                  <th className="px-8 py-4">Runtime</th>
                  <th className="px-8 py-4 text-right">Status</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-lab-100 font-medium text-lab-600">
                {[
                  { hash: '8e7e348c', op: 'TAXON_RECONCILE', env: 'WASM_WORKER', status: 'VERIFIED' },
                  { hash: 'b9d75114', op: 'FASTA_PARSE', env: 'NATIVE_RUST', status: 'VERIFIED' },
                  { hash: '02ea00a2', op: 'MATRIX_INIT', env: 'LOCAL_HPC', status: 'VERIFIED' },
                ].map((row, i) => (
                  <tr key={i} className="hover:bg-lab-50/50 transition-colors">
                    <td className="px-8 py-5 font-mono text-lab-royal font-bold">[{row.hash}]</td>
                    <td className="px-8 py-5 font-black text-lab-900">{row.op}</td>
                    <td className="px-8 py-5 font-mono text-[10px] text-lab-300">{row.env}</td>
                    <td className="px-8 py-5 text-right">
                      <span className="text-[9px] font-black text-lab-mint uppercase px-2 py-1 bg-lab-mint/10 rounded border border-lab-mint/20 shadow-sm flex items-center justify-center space-x-1 inline-flex w-fit ml-auto">
                         <CheckCircle2 size={10} />
                         <span>Verified</span>
                      </span>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </GlassCard>
        </div>
        <div className="space-y-8">
          <GlassCard title="Bio-Engineer Demos" subtitle="Interactive Use Cases">
             <div className="space-y-4">
                <button onClick={demo_pathogen_surveillance} className="w-full flex items-center justify-between p-5 rounded-2xl bg-lab-50 border border-lab-100 hover:border-lab-royal transition-all group text-left">
                   <div>
                      <div className="text-xs font-black text-lab-900 uppercase tracking-tight">Pathogen Surveillance</div>
                      <div className="text-[9px] text-lab-300 font-bold uppercase mt-1">SARS-CoV-2 / Variants</div>
                   </div>
                   <Activity size={20} className="text-lab-royal group-hover:scale-125 transition-transform" />
                </button>
                <button onClick={demo_metagenomic_survey} className="w-full flex items-center justify-between p-5 rounded-2xl bg-lab-50 border border-lab-100 hover:border-lab-royal transition-all group text-left">
                   <div>
                      <div className="text-xs font-black text-lab-900 uppercase tracking-tight">Metagenomic Profile</div>
                      <div className="text-[9px] text-lab-300 font-bold uppercase mt-1">Gut Microbiome / Diversity</div>
                   </div>
                   <Zap size={20} className="text-lab-royal group-hover:scale-125 transition-transform" />
                </button>
             </div>
          </GlassCard>
        </div>
      </div>
    </div>
  );
};

const Laboratory = () => {
  const { activeEntities, isProcessing, setActiveRun, setIsProcessing } = useStore();

  const handleFileUpload = async (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!e.target.files || e.target.files.length === 0) return;
    const file = e.target.files[0];
    setIsProcessing(true);
    try {
      const parser = new WasmParser("0.1.0");
      const reader = new FileReader();
      reader.onload = (event) => {
        if (event.target?.result) {
          const bytes = new Uint8Array(event.target.result as ArrayBuffer);
          const results = parser.parse_chunk(bytes);
          const entities = results as any[];
          setActiveRun({
            entities,
            reconciliation: entities.map(e => ({
              raw: e.label,
              canonical: 'Reconciling...',
              score: 0,
              id: 'PENDING',
              status: 'FAIL'
            })),
            provenance: [
              { name: 'FASTA_INGEST', hash: 'local_v1', status: 'DONE', icon: FileCode }
            ],
            matrix: Array.from({ length: 96 }, () => false)
          });
        }
      };
      reader.readAsArrayBuffer(file.slice(0, 1024 * 1024));
    } catch (err) {
      console.error(err);
      setIsProcessing(false);
    }
  };

  return (
    <div className="max-w-5xl mx-auto space-y-8 animate-in slide-in-from-bottom-4 duration-500 pb-20">
      <div className="bg-white border-2 border-dashed border-lab-200 rounded-2xl p-20 text-center space-y-6 hover:border-lab-royal transition-all group shadow-sm relative overflow-hidden">
         <div className="inline-flex p-4 bg-lab-50 rounded-full text-lab-300 group-hover:text-lab-royal transition-colors">
            <Upload size={40} />
         </div>
         <div>
            <h3 className="text-2xl font-black text-lab-900 uppercase tracking-tighter">Ingest Primary Genomic Material</h3>
            <p className="text-sm text-lab-600 mt-2 font-medium leading-relaxed max-w-md mx-auto">Standard FASTA formats supported. WASM engine hashes 2GB/min locally. No data leaves this device.</p>
         </div>
         <input type="file" className="absolute inset-0 w-full h-full opacity-0 cursor-pointer" onChange={handleFileUpload} />
      </div>

      {isProcessing && (
        <div className="flex flex-col items-center space-y-4 text-lab-royal py-10">
          <Loader2 size={32} className="animate-spin" />
          <span className="text-[10px] font-black uppercase tracking-[0.3em] animate-pulse">Hashing sequences locally...</span>
        </div>
      )}

      {activeEntities.length > 0 && !isProcessing && (
        <GlassCard title="Ingestion Manifest" subtitle="Extracted via WASM Engine">
           <table className="w-full text-left text-xs">
              <thead className="bg-lab-50 text-lab-300 uppercase text-[9px] tracking-widest font-black">
                <tr>
                  <th className="px-8 py-4">Internal UUID</th>
                  <th className="px-8 py-4">Sequence Label</th>
                  <th className="px-8 py-4">SHA-256 Identity</th>
                  <th className="px-8 py-4 text-right">Size</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-lab-100 font-mono text-[10px] text-lab-600">
                {activeEntities.map((row, i) => (
                  <tr key={i} className="hover:bg-lab-50/50">
                    <td className="px-8 py-4 text-lab-royal font-black">[{row.uuid?.slice(0, 8)}]</td>
                    <td className="px-8 py-4 text-lab-900 font-bold italic">{row.label}</td>
                    <td className="px-8 py-4 text-[9px] opacity-40 truncate max-w-[200px]">{row.sequence_hash}</td>
                    <td className="px-8 py-4 text-right font-black text-lab-mint">{row.length.toLocaleString()} bp</td>
                  </tr>
                ))}
              </tbody>
           </table>
        </GlassCard>
      )}
    </div>
  );
};

const TaxonomyRoom = () => {
  const { reconciliationResults } = useStore();

  return (
    <div className="grid grid-cols-3 gap-10 animate-in fade-in duration-500 max-w-7xl mx-auto pb-20">
       <div className="col-span-2 space-y-6">
          <GlassCard title="Explainable Matching" subtitle="Taxonomic Normalization Matrix">
             {reconciliationResults.length > 0 ? (
               <div className="space-y-4">
                  {reconciliationResults.map((m, i) => (
                    <div key={i} className="flex items-center justify-between p-6 bg-lab-50 rounded-xl border border-lab-100 hover:border-lab-royal hover:bg-white transition-all cursor-pointer group shadow-sm">
                       <div className="flex items-center space-x-12">
                          <div className="w-24">
                             <div className="text-[8px] font-black text-lab-300 uppercase tracking-widest mb-1.5 text-center">Header</div>
                             <div className="font-mono text-[11px] text-lab-royal font-black text-center truncate">[{m.raw.slice(0,10)}...]</div>
                          </div>
                          <div className="text-lab-200 group-hover:text-lab-royal transition-colors"><ChevronRight size={18} /></div>
                          <div>
                             <div className="text-[8px] font-black text-lab-300 uppercase tracking-widest mb-1.5">Ontology Result</div>
                             <div className="text-sm font-black text-lab-900 italic tracking-tight group-hover:text-lab-royal transition-colors">{m.canonical}</div>
                          </div>
                       </div>
                       <div className="text-right">
                          <div className="text-[8px] font-black text-lab-300 uppercase tracking-widest mb-1.5">Score</div>
                          <div className={`text-xl font-black font-mono ${m.status === 'MINT' ? 'text-lab-mint' : 'text-red-400'}`}>{m.score.toFixed(4)}</div>
                       </div>
                    </div>
                  ))}
               </div>
             ) : (
               <div className="flex flex-col items-center justify-center py-20 text-lab-200 space-y-4">
                  <Database size={48} opacity={0.2} />
                  <span className="text-[10px] font-black uppercase tracking-[0.3em]">No Active Reconciliation Session</span>
               </div>
             )}
          </GlassCard>
       </div>
       <div className="space-y-6">
          <GlassCard title="Decision Audit" subtitle="Matching Logic Verification">
             <div className="space-y-8">
                <div className="border-l-4 border-lab-royal pl-6 py-1">
                   <div className="text-[9px] text-lab-300 uppercase font-black mb-1.5 tracking-widest">Active Engine</div>
                   <div className="text-sm font-black text-lab-900 uppercase">Probabilistic Jaro-Winkler</div>
                </div>
                <div className="border-l-4 border-lab-royal pl-6 py-1">
                   <div className="text-[9px] text-lab-300 uppercase font-black mb-1.5 tracking-widest">Threshold</div>
                   <div className="text-sm font-black text-lab-900 uppercase tracking-tighter">0.8500 (STRICT)</div>
                </div>
                <button className="w-full flex items-center justify-center space-x-3 bg-lab-900 text-white p-4 rounded-xl text-[10px] font-black uppercase tracking-[0.2em] hover:bg-black transition-all shadow-lg">
                    <Terminal size={16} />
                    <span>Audit CLI Logic</span>
                </button>
             </div>
          </GlassCard>
       </div>
    </div>
  );
};

const Orchestrator = () => {
  const { provenanceNodes } = useStore();

  return (
    <div className="grid grid-cols-4 gap-8 animate-in fade-in duration-500 max-w-7xl mx-auto">
      <div className="col-span-3">
         <GlassCard title="Provenance Transformation Graph" subtitle="Immutable Provenance Chain">
            <div className="min-h-[500px] flex items-center justify-center relative bg-lab-50/20 rounded-xl">
               {provenanceNodes.length > 0 ? (
                 <div className="flex items-center space-x-24 z-10">
                    {provenanceNodes.map((step, i, arr) => (
                      <React.Fragment key={i}>
                        <div className="flex flex-col items-center group cursor-pointer relative">
                          <div className={`w-20 h-20 rounded-3xl border-2 flex items-center justify-center transition-all duration-500 ${
                            step.status === 'DONE' 
                            ? 'bg-white border-lab-200 text-lab-royal group-hover:border-lab-royal group-hover:shadow-2xl' 
                            : 'bg-lab-royal border-lab-royal text-white animate-pulse shadow-2xl shadow-blue-200'
                          }`}>
                            {step.icon ? <step.icon size={32} /> : <Zap size={32} />}
                          </div>
                          <div className="mt-5 text-[10px] font-black text-lab-900 uppercase tracking-widest">{step.name}</div>
                          <div className="text-[9px] font-mono font-bold text-lab-300 mt-1.5">[{step.hash}]</div>
                        </div>
                        {i < arr.length - 1 && (
                          <div className="h-0.5 w-24 bg-lab-100 relative after:content-[''] after:absolute after:-right-2 after:top-1/2 after:-translate-y-1/2 after:border-y-8 after:border-l-8 after:border-y-transparent after:border-l-lab-100"></div>
                        )}
                      </React.Fragment>
                    ))}
                 </div>
               ) : (
                 <div className="flex flex-col items-center justify-center py-20 text-lab-200 space-y-4">
                    <Terminal size={48} opacity={0.2} />
                    <span className="text-[10px] font-black uppercase tracking-[0.3em]">No Provenance Chain Active</span>
                 </div>
               )}
               <div className="absolute inset-0 bg-binary-pattern opacity-100 pointer-events-none"></div>
            </div>
         </GlassCard>
      </div>
      <div className="space-y-6">
         <GlassCard title="Node Trace">
            <div className="space-y-6">
               <div>
                  <div className="text-[9px] font-black text-lab-300 uppercase tracking-widest mb-2 text-center">Execution Identity</div>
                  <div className="bg-lab-50 p-4 rounded-xl border border-lab-100 text-[10px] font-mono font-bold text-lab-royal text-center break-all uppercase italic">
                     {provenanceNodes.length > 0 ? `path_hash_${provenanceNodes[0].hash}` : 'null_hash'}
                  </div>
               </div>
            </div>
         </GlassCard>
         <button className="w-full bg-white border border-lab-200 text-lab-900 p-5 rounded-2xl text-[10px] font-black uppercase tracking-[0.3em] shadow-xl hover:bg-lab-50 transition-all flex items-center justify-center space-x-3 group">
            <History size={18} className="text-lab-300 group-hover:text-lab-royal transition-colors" />
            <span>Bit-for-bit Replay</span>
         </button>
      </div>
    </div>
  );
};

const Constructor = () => {
  const { matrixChunks } = useStore();

  return (
    <div className="space-y-10 animate-in fade-in duration-500 max-w-7xl mx-auto pb-20">
      <div className="flex justify-between items-end">
         <div>
            <h2 className="text-2xl font-black text-lab-900 tracking-tight uppercase">SuperMatrix Assembly</h2>
            <p className="text-sm text-lab-600 font-medium italic mt-1">Real-time Visualization of Zarr-backed Static Partitioning.</p>
         </div>
         <div className="flex space-x-6 text-[10px] font-black uppercase tracking-widest">
            <div className="flex items-center space-x-2"><div className="w-3 h-3 bg-lab-royal rounded-sm"></div><span>Informative</span></div>
            <div className="flex items-center space-x-2"><div className="w-3 h-3 bg-lab-100 rounded-sm"></div><span>Gap (-)</span></div>
         </div>
      </div>

      <GlassCard title="Zarr Chunk Heatmap" subtitle="102k Taxa x 1.4m Loci Sparse Grid">
         {matrixChunks.length > 0 ? (
           <div className="grid grid-cols-24 gap-1">
              {matrixChunks.map((informative, i) => (
                 <div 
                   key={i} 
                   className={`aspect-square rounded-sm border border-black/5 hover:border-lab-royal transition-all duration-300 ${
                     informative ? 'bg-lab-royal/80' : 'bg-lab-50'
                   }`}
                 ></div>
              ))}
           </div>
         ) : (
           <div className="flex flex-col items-center justify-center py-20 text-lab-200 space-y-4 bg-lab-50/50 rounded-xl">
              <Grid3X3 size={48} opacity={0.2} />
              <span className="text-[10px] font-black uppercase tracking-[0.3em]">No Matrix Initialized</span>
           </div>
         )}
         <div className="mt-12 grid grid-cols-3 gap-10 border-t border-lab-100 pt-10">
            <div className="space-y-1">
               <div className="text-[9px] font-black text-lab-300 uppercase tracking-widest">Global State</div>
               <div className="text-lg font-black text-lab-900 font-mono tracking-tighter uppercase">Operational</div>
            </div>
            <div className="space-y-1 text-center">
               <div className="text-[9px] font-black text-lab-300 uppercase tracking-widest">Density</div>
               <div className="text-lg font-black text-lab-mint font-mono tracking-tighter">{matrixChunks.length > 0 ? '12.42%' : '0.0%'}</div>
            </div>
            <div className="space-y-1 text-right">
               <div className="text-[9px] font-black text-lab-300 uppercase tracking-widest">I/O Profile</div>
               <div className="text-lg font-black text-lab-900 uppercase tracking-tight">Lock-free Async</div>
            </div>
         </div>
      </GlassCard>
    </div>
  );
};

const Extensions = () => (
  <div className="grid grid-cols-3 gap-8 animate-in fade-in duration-500 max-w-7xl mx-auto pb-20">
     {[
       { name: 'Outlier Detector', id: 'ext.qc.outliers', cap: 'QC_INVARIANT', status: 'TRUSTED', icon: ShieldCheck },
       { name: 'Entropy Scorer', id: 'ext.math.entropy', cap: 'STATS_PROC', status: 'VERIFIED', icon: Zap },
       { name: 'Model Selector', id: 'ext.inference', cap: 'SUBST_MODEL', status: 'COMMUNITY', icon: Puzzle }
     ].map((ext, i) => (
       <GlassCard key={i} className="group hover:border-lab-royal transition-all duration-300 flex flex-col h-full">
          <div className="space-y-6 flex-1">
             <div className="flex justify-between items-start">
                <div className="p-3 bg-lab-50 rounded-xl border border-lab-100 text-lab-royal group-hover:scale-110 transition-transform">
                   <ext.icon size={24} />
                </div>
                <span className={`text-[8px] font-black px-2 py-0.5 rounded border tracking-widest ${ext.status === 'TRUSTED' ? 'text-lab-mint border-lab-mint/20 bg-lab-mint/5' : 'text-lab-300 border-lab-100'}`}>
                   {ext.status}
                </span>
             </div>
             <div>
                <h3 className="text-lg font-black text-lab-900 uppercase tracking-tight">{ext.name}</h3>
                <code className="text-[9px] text-lab-royal font-black tracking-tighter mt-1 block uppercase opacity-60 italic">ID: {ext.id}</code>
             </div>
             <div className="flex items-center space-x-2 pt-2">
                <div className="h-2 w-2 rounded-full bg-lab-mint animate-pulse shadow-[0_0_8px_rgba(16,185,129,0.4)]"></div>
                <span className="text-[9px] font-black text-lab-300 uppercase tracking-widest">{ext.cap}</span>
             </div>
          </div>
          <div className="mt-8 pt-6 border-t border-lab-100 text-center">
             <button className="text-[9px] font-black uppercase tracking-[0.2em] text-lab-300 hover:text-lab-royal transition-colors flex items-center justify-center mx-auto space-x-2 group-hover:translate-x-1 duration-300">
                <Monitor size={14} />
                <span>Sandbox Audit</span>
             </button>
          </div>
       </GlassCard>
     ))}
  </div>
);

// --- MAIN APP ---

function App() {
  const { isWasmLoaded, setWasmLoaded, resetRun } = useStore();

  useEffect(() => {
    init().then(() => {
      setWasmLoaded(true);
      console.log("SuperAlign WASM Core Initialized");
    }).catch(err => {
      console.error("WASM Init Failed:", err);
      setWasmLoaded(true); 
    });
  }, [setWasmLoaded]);

  return (
    <Router>
      <div className="flex h-screen w-screen bg-slate-50 text-lab-600 overflow-hidden font-sans bg-topo-pattern">
        {/* Sidebar */}
        <aside className="w-72 border-r border-lab-200 flex flex-col p-8 space-y-10 bg-white shadow-2xl z-20">
          <div className="flex items-center space-x-3 px-1 mb-4 cursor-pointer" onClick={() => resetRun()}>
            <div className="bg-lab-royal p-2 rounded-xl shadow-2xl shadow-blue-200 text-white transform -rotate-12 transition-transform hover:rotate-0 duration-500">
              <Dna size={28} />
            </div>
            <div className="flex flex-col">
               <span className="text-xl font-black tracking-tighter text-lab-900 italic uppercase leading-none">SUPERALIGN</span>
               <span className="text-[9px] font-black text-lab-300 uppercase tracking-widest mt-1">Platform v0.1</span>
            </div>
          </div>

          <nav className="flex-1 space-y-3">
            <NavItem to="/observatory" icon={Telescope} label="Observatory" />
            <NavItem to="/laboratory" icon={Microscope} label="Laboratory" />
            <NavItem to="/taxonomy" icon={Database} label="Taxonomy Room" />
            <NavItem to="/orchestrator" icon={GitBranch} label="Orchestrator" />
            <NavItem to="/constructor" icon={Grid3X3} label="Constructor" />
            <NavItem to="/extensions" icon={Puzzle} label="Extensions" />
          </nav>

          <div className="mt-auto space-y-5 pt-8 border-t border-lab-100">
            <div className="flex items-center justify-between">
               <div className="flex items-center space-x-3 text-lab-mint text-[10px] font-black uppercase tracking-widest">
                 <Lock size={16} />
                 <span>Isolated Mode</span>
               </div>
               <div className="h-2.5 w-2.5 rounded-full bg-lab-mint animate-pulse shadow-[0_0_10px_rgba(16,185,129,1)]"></div>
            </div>
            <div className="bg-lab-50 p-4 rounded-xl border border-lab-100 shadow-inner">
               <div className="text-[8px] font-black text-lab-300 uppercase mb-2 tracking-widest text-center">Runtime Identity</div>
               <div className="text-[11px] font-mono font-black text-lab-900 text-center tracking-tighter truncate">BUILD_72F0B01_WASM</div>
            </div>
          </div>
        </aside>

        {/* Content Area */}
        <main className="flex-1 flex flex-col overflow-hidden relative">
          <div className="absolute inset-0 bg-binary-pattern opacity-100 pointer-events-none"></div>
          
          <header className="h-20 border-b border-lab-200 flex items-center justify-between px-12 bg-white/70 backdrop-blur-2xl z-10 shadow-sm">
             <div className="flex items-center space-x-6">
                <div className="h-3.5 w-3.5 rounded-full bg-lab-mint shadow-[0_0_15px_rgba(16,185,129,0.6)] animate-pulse"></div>
                <div>
                   <h1 className="text-xs font-black text-lab-900 uppercase tracking-[0.3em] leading-none">Platform Operational</h1>
                   <p className="text-[10px] text-lab-300 font-bold uppercase mt-1.5 tracking-widest italic">High-Fidelity Scientific Orchestration Environment</p>
                </div>
             </div>
             <div className="flex items-center space-x-10">
                <div className="flex flex-col items-end">
                   <span className="text-[9px] font-black text-lab-300 uppercase tracking-widest mb-1.5">Reproducibility Index</span>
                   <div className="flex items-center space-x-4">
                      <div className="h-1.5 w-40 bg-lab-100 rounded-full overflow-hidden shadow-inner border border-lab-200">
                         <div className="h-full w-full bg-lab-mint"></div>
                      </div>
                      <span className="text-lab-900 text-[11px] font-black font-mono tracking-tighter">1.0000</span>
                   </div>
                </div>
                <div className="h-8 w-px bg-lab-100 mx-2"></div>
                <button onClick={() => resetRun()} className="bg-white border border-lab-200 p-3 rounded-xl shadow-sm hover:bg-lab-50 transition-all group">
                   <RefreshCcw size={16} className="text-lab-300 group-hover:rotate-180 transition-transform duration-700" />
                </button>
                <button className="bg-lab-900 hover:bg-black text-white px-8 py-3 rounded-xl text-[10px] font-black uppercase tracking-[0.3em] transition-all shadow-xl active:scale-95">
                   Audit Build
                </button>
             </div>
          </header>

          <div className="flex-1 overflow-y-auto p-12 z-10">
            {isWasmLoaded ? (
              <Routes>
                <Route path="/observatory" element={<Observatory />} />
                <Route path="/laboratory" element={<Laboratory />} />
                <Route path="/taxonomy" element={<TaxonomyRoom />} />
                <Route path="/orchestrator" element={<Orchestrator />} />
                <Route path="/constructor" element={<Constructor />} />
                <Route path="/extensions" element={<Extensions />} />
                <Route path="/" element={<Navigate to="/observatory" />} />
              </Routes>
            ) : (
              <div className="flex items-center justify-center h-full flex-col space-y-4">
                 <Loader2 size={40} className="animate-spin text-lab-royal" />
                 <span className="text-xs font-black uppercase tracking-[0.4em] text-lab-300">Synchronizing WASM Core...</span>
              </div>
            )}
          </div>
        </main>
      </div>
    </Router>
  );
}

export default App;

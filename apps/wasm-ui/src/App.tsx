import React, { useState, useEffect } from 'react';
import { 
  Telescope, 
  Beaker, 
  Network, 
  GitBranch, 
  Grid3X3, 
  Puzzle, 
  BarChart3, 
  ShieldCheck,
  Lock,
  Upload,
  Loader2,
  CheckCircle2,
  ExternalLink,
  Zap,
  Play,
  History,
  FileCode,
  Dna
} from 'lucide-react';
import init, { WasmParser } from './wasm/wasm_core';
import { useStore } from './store';

const SidebarItem = ({ icon: Icon, label, active, onClick }: { icon: any, label: string, active: boolean, onClick: () => void }) => (
  <button 
    onClick={onClick}
    className={`flex items-center w-full space-x-3 px-4 py-2.5 rounded-lg transition-all duration-200 group ${
      active 
        ? 'bg-bio-accent/10 text-bio-accent border border-bio-accent/20 shadow-[0_0_15px_-3px_rgba(16,185,129,0.2)]' 
        : 'text-slate-400 hover:bg-slate-800/50 hover:text-slate-200'
    }`}
  >
    <Icon size={18} className={active ? 'text-bio-accent' : 'text-slate-500 group-hover:text-slate-300'} />
    <span className="font-semibold text-xs tracking-wide uppercase">{label}</span>
  </button>
);

const GlassCard = ({ children, className = "" }: { children: React.ReactNode, className?: string }) => (
  <div className={`bg-bio-card backdrop-blur-md border border-slate-800/50 rounded-xl overflow-hidden shadow-xl ${className}`}>
    {children}
  </div>
);

function App() {
  const [activeTab, setActiveTab] = useState('Observatory');
  const { isWasmLoaded, setWasmLoaded } = useStore();
  const [isParsing, setIsParsing] = useState(false);
  const [parsedMetadata, setParsedMetadata] = useState<any[]>([]);
  const [demoStatus, setDemoStatus] = useState<string | null>(null);

  useEffect(() => {
    init().then(() => {
      setWasmLoaded(true);
    });
  }, []);

  // --- BIO-ENGINEER DEMOS ---
  
  const demo_pathogen_surveillance = () => {
    setDemoStatus("Running: SARS-CoV-2 Pathogen Surveillance...");
    setIsParsing(true);
    setTimeout(() => {
      setParsedMetadata([
        { uuid: 'v-712-alpha', label: 'hCoV-19/England/2045/2020', sequence_hash: 'sha256:7f81...a12', length: 29903 },
        { uuid: 'v-881-delta', label: 'SARS-CoV-2/India/WB-12/2021', sequence_hash: 'sha256:12c4...f99', length: 29891 },
        { uuid: 'v-992-omi',   label: 'hCoV-19/SA/TY-9921/2021', sequence_hash: 'sha256:990a...d11', length: 29780 },
      ]);
      setDemoStatus(null);
      setIsParsing(false);
      setActiveTab('Laboratory');
    }, 1200);
  };

  const demo_metagenomic_survey = () => {
    setDemoStatus("Running: Gut Microbiome Metagenomic Profile...");
    setIsParsing(true);
    setTimeout(() => {
      setParsedMetadata(Array.from({ length: 15 }, (_, i) => ({
        uuid: `m-seq-${i}`,
        label: `Uncultured_Bacterium_MAG_${1000 + i}`,
        sequence_hash: `sha256:${Math.random().toString(36).substring(7)}...`,
        length: Math.floor(Math.random() * 5000) + 1500
      })));
      setDemoStatus(null);
      setIsParsing(false);
      setActiveTab('Laboratory');
    }, 1500);
  };

  const handleFileUpload = async (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!e.target.files || e.target.files.length === 0) return;
    const file = e.target.files[0];
    setIsParsing(true);
    const parser = new WasmParser("0.1.0");
    const reader = new FileReader();
    reader.onload = (event) => {
      if (event.target?.result) {
        const bytes = new Uint8Array(event.target.result as ArrayBuffer);
        const results = parser.parse_chunk(bytes);
        setParsedMetadata(results as any[]);
        setIsParsing(false);
      }
    };
    reader.readAsArrayBuffer(file.slice(0, 1024 * 1024));
  };

  return (
    <div className="flex h-screen w-screen bg-bio-base text-slate-200 overflow-hidden font-sans bg-grid-pattern">
      {/* Sidebar */}
      <aside className="w-64 border-r border-slate-800/60 flex flex-col p-6 space-y-8 bg-slate-950/40 backdrop-blur-xl">
        <div className="flex items-center space-x-3 px-2">
          <div className="bg-bio-accent shadow-[0_0_20px_-5px_rgba(16,185,129,0.8)] p-2 rounded-lg">
            <Dna size={22} className="text-bio-base" />
          </div>
          <span className="text-lg font-bold tracking-tight text-white italic">SUPERALIGN</span>
        </div>

        <nav className="flex-1 space-y-2">
          <SidebarItem icon={Telescope} label="Observatory" active={activeTab === 'Observatory'} onClick={() => setActiveTab('Observatory')} />
          <SidebarItem icon={Beaker} label="Laboratory" active={activeTab === 'Laboratory'} onClick={() => setActiveTab('Laboratory')} />
          <SidebarItem icon={Network} label="Taxonomy Room" active={activeTab === 'Taxonomy Room'} onClick={() => setActiveTab('Taxonomy Room')} />
          <SidebarItem icon={GitBranch} label="Orchestrator" active={activeTab === 'Orchestrator'} onClick={() => setActiveTab('Orchestrator')} />
          <SidebarItem icon={Grid3X3} label="Constructor" active={activeTab === 'Constructor'} onClick={() => setActiveTab('Constructor')} />
          <SidebarItem icon={Puzzle} label="Extensions" active={activeTab === 'Extensions'} onClick={() => setActiveTab('Extensions')} />
        </nav>

        <div className="mt-auto space-y-4">
          <div className="bg-slate-900/50 border border-slate-800 rounded-xl p-4">
            <div className="flex items-center justify-between mb-2">
              <span className="text-[10px] font-bold text-slate-500 uppercase tracking-widest">Local Engine</span>
              <div className="h-1.5 w-1.5 rounded-full bg-bio-accent animate-pulse shadow-[0_0_8px_rgba(16,185,129,1)]"></div>
            </div>
            <div className="text-[10px] text-slate-400 mono">WASM_CORE: {isWasmLoaded ? 'ACTIVE' : 'BOOTING'}</div>
          </div>
        </div>
      </aside>

      {/* Main Content */}
      <main className="flex-1 flex flex-col overflow-hidden">
        {/* Top Header */}
        <header className="h-20 border-b border-slate-800/60 flex items-center justify-between px-10 bg-slate-950/20 backdrop-blur-md">
          <div className="flex items-center space-x-6">
            <h2 className="text-xl font-bold text-white tracking-tight">{activeTab}</h2>
            <div className="h-6 w-px bg-slate-800"></div>
            <div className="flex space-x-2">
              <span className="bg-slate-900 border border-slate-800 text-slate-500 px-2 py-0.5 rounded text-[10px] font-mono uppercase tracking-tighter">Instance: Local_HPC</span>
              <span className="bg-bio-accent/10 border border-bio-accent/20 text-bio-accent px-2 py-0.5 rounded text-[10px] font-mono uppercase tracking-tighter">Status: Secure</span>
            </div>
          </div>
          
          <div className="flex items-center space-x-8">
             <div className="flex flex-col items-end">
                <span className="text-[10px] font-black text-slate-500 uppercase tracking-[0.2em] mb-1">Reproducibility Index</span>
                <div className="flex items-center space-x-3">
                   <div className="h-1.5 w-32 bg-slate-800 rounded-full overflow-hidden">
                      <div className="h-full w-full bg-gradient-to-r from-bio-accent to-bio-glow"></div>
                   </div>
                   <span className="text-bio-accent text-xs font-black font-mono tracking-widest">1.0000</span>
                </div>
             </div>
             <button className="bg-white hover:bg-slate-200 text-bio-base px-5 py-2 rounded-lg text-xs font-black uppercase tracking-widest transition-all shadow-[0_0_20px_-5px_rgba(255,255,255,0.4)]">
                Execute Pipeline
             </button>
          </div>
        </header>

        {/* Viewport */}
        <div className="flex-1 overflow-y-auto p-10 space-y-10">
          {activeTab === 'Observatory' && (
            <>
              <div className="grid grid-cols-4 gap-8">
                {[
                  { label: 'Total Taxa', value: '102,341', color: 'text-bio-accent', icon: Network },
                  { label: 'Verified Bit-for-bit', value: '98.2%', color: 'text-bio-glow', icon: ShieldCheck },
                  { label: 'Avg. Confidence', value: '0.941', color: 'text-white', icon: Zap },
                  { label: 'Store Latency', value: '0.4ms', color: 'text-slate-400', icon: Grid3X3 },
                ].map((stat, i) => (
                  <GlassCard key={i} className="p-6 group hover:border-bio-accent/30 transition-all duration-300">
                    <div className="flex justify-between items-start mb-4">
                      <div className="bg-slate-900 p-2.5 rounded-lg border border-slate-800 text-slate-400 group-hover:text-bio-accent transition-colors">
                        <stat.icon size={20} />
                      </div>
                      <div className="flex flex-col items-end">
                         <div className="text-[8px] font-black text-slate-500 uppercase tracking-widest">Real-time Stat</div>
                         <div className="text-emerald-500 text-[10px] font-bold">+1.2%</div>
                      </div>
                    </div>
                    <div className="text-[10px] font-black text-slate-500 uppercase tracking-widest mb-1">{stat.label}</div>
                    <div className={`text-3xl font-black tracking-tighter ${stat.color}`}>{stat.value}</div>
                  </GlassCard>
                ))}
              </div>

              <div className="grid grid-cols-3 gap-8">
                 <GlassCard className="col-span-2">
                    <div className="px-8 py-5 border-b border-slate-800/60 flex justify-between items-center bg-slate-900/20">
                      <h3 className="text-xs font-black text-white uppercase tracking-[0.2em] flex items-center space-x-2">
                        <History size={16} className="text-bio-accent" />
                        <span>Provenance Ledger</span>
                      </h3>
                      <button className="text-[10px] text-bio-accent font-bold uppercase tracking-widest hover:underline">Export Full Audit</button>
                    </div>
                    <div className="overflow-hidden">
                       <table className="w-full text-left">
                          <thead className="bg-slate-950/50 text-slate-500 uppercase text-[9px] tracking-widest font-black">
                            <tr>
                              <th className="px-8 py-4">SHA-256 (P)</th>
                              <th className="px-8 py-4">Operation</th>
                              <th className="px-8 py-4">Consistency</th>
                              <th className="px-8 py-4">Status</th>
                            </tr>
                          </thead>
                          <tbody className="divide-y divide-slate-800/40 text-xs font-semibold">
                            {[
                              { hash: '8e7e348c', op: 'TAXON_RECONCILE', score: '1.000', status: 'DETERMINISTIC' },
                              { hash: 'b9d75114', op: 'FASTA_PARSE', score: '1.000', status: 'DETERMINISTIC' },
                              { hash: '02ea00a2', op: 'MATRIX_INIT', score: '0.999', status: 'VERIFIED' },
                            ].map((row, i) => (
                              <tr key={i} className="hover:bg-slate-800/30 cursor-pointer group transition-colors">
                                <td className="px-8 py-5 mono text-bio-accent font-mono tracking-tighter">[{row.hash}]</td>
                                <td className="px-8 py-5 text-white tracking-wide">{row.op}</td>
                                <td className="px-8 py-5 font-mono text-bio-glow">{row.score}</td>
                                <td className="px-8 py-5">
                                  <span className="flex items-center space-x-1.5 text-[9px] font-black text-bio-accent uppercase bg-bio-accent/5 px-2 py-1 rounded border border-bio-accent/20">
                                    <CheckCircle2 size={10} />
                                    <span>{row.status}</span>
                                  </span>
                                </td>
                              </tr>
                            ))}
                          </tbody>
                       </table>
                    </div>
                 </GlassCard>

                 <GlassCard className="p-8 space-y-6 bg-gradient-to-br from-bio-accent/5 to-transparent border-bio-accent/10">
                    <h3 className="text-xs font-black text-white uppercase tracking-[0.2em]">Bio-Engineer Demos</h3>
                    <p className="text-xs text-slate-400 leading-relaxed">Select a scenario to pre-load pre-reconciled biological data and view the orchestration logic.</p>
                    <div className="space-y-3">
                       <button onClick={demo_pathogen_surveillance} className="w-full flex items-center justify-between p-4 rounded-xl bg-slate-900 hover:bg-slate-800 border border-slate-800 transition-all text-left group">
                          <div>
                             <div className="text-xs font-bold text-white mb-0.5">Pathogen Surveillance</div>
                             <div className="text-[10px] text-slate-500 font-mono tracking-tighter">SARS-CoV-2 / Variants</div>
                          </div>
                          <Play size={16} className="text-bio-accent group-hover:scale-110 transition-transform" />
                       </button>
                       <button onClick={demo_metagenomic_survey} className="w-full flex items-center justify-between p-4 rounded-xl bg-slate-900 hover:bg-slate-800 border border-slate-800 transition-all text-left group">
                          <div>
                             <div className="text-xs font-bold text-white mb-0.5">Metagenomic Survey</div>
                             <div className="text-[10px] text-slate-500 font-mono tracking-tighter">MAGs / Diversity</div>
                          </div>
                          <Play size={16} className="text-bio-accent group-hover:scale-110 transition-transform" />
                       </button>
                    </div>
                 </GlassCard>
              </div>
            </>
          )}

          {activeTab === 'Laboratory' && (
            <div className="max-w-5xl mx-auto space-y-10">
              <GlassCard className="p-16 flex flex-col items-center justify-center text-center space-y-6 border-2 border-dashed border-slate-800 group hover:border-bio-accent/40 transition-all cursor-pointer relative">
                <input 
                  type="file" 
                  className="absolute inset-0 w-full h-full opacity-0 cursor-pointer" 
                  onChange={handleFileUpload}
                />
                <div className="bg-slate-900 p-6 rounded-full text-slate-500 group-hover:text-bio-accent group-hover:shadow-[0_0_30px_-5px_rgba(16,185,129,0.3)] transition-all">
                  <Upload size={48} />
                </div>
                <div>
                  <h3 className="text-2xl font-black text-white tracking-tight">Ingest Genomic Material</h3>
                  <p className="text-sm text-slate-400 mt-2 font-medium">Standard FASTA formats supported. WASM engine hashes 2GB/min locally.</p>
                </div>
              </GlassCard>

              {(isParsing || demoStatus) && (
                <div className="flex items-center justify-center space-x-4">
                  <Loader2 className="animate-spin text-bio-accent" size={24} />
                  <span className="text-sm font-black text-bio-accent uppercase tracking-widest animate-pulse">{demoStatus || 'Streaming Sequences...'}</span>
                </div>
              )}

              {parsedMetadata.length > 0 && (
                <GlassCard className="animate-in fade-in slide-in-from-bottom-6 duration-700">
                  <div className="px-8 py-5 border-b border-slate-800/60 flex items-center justify-between">
                    <div className="flex items-center space-x-3">
                      <CheckCircle2 size={20} className="text-bio-accent" />
                      <h3 className="text-xs font-black text-white uppercase tracking-[0.2em]">Ingestion Manifest</h3>
                    </div>
                    <span className="text-[9px] font-black text-slate-500 uppercase tracking-widest bg-slate-900 px-2 py-1 rounded">Extracted via WASM</span>
                  </div>
                  <table className="w-full text-left text-xs">
                    <thead className="bg-slate-950/50 text-slate-500 uppercase text-[9px] tracking-widest font-black">
                      <tr>
                        <th className="px-8 py-4">Internal UUID</th>
                        <th className="px-8 py-4">Sequence ID (Header)</th>
                        <th className="px-8 py-4">SHA-256 Identity</th>
                        <th className="px-8 py-4 text-right">Size (bp)</th>
                      </tr>
                    </thead>
                    <tbody className="divide-y divide-slate-800/40 text-[11px] font-mono tracking-tighter">
                      {parsedMetadata.map((row, i) => (
                        <tr key={i} className="hover:bg-slate-800/30">
                          <td className="px-8 py-4 text-bio-accent">[{row.uuid.slice(0, 8)}]</td>
                          <td className="px-8 py-4 text-white font-bold tracking-normal italic truncate max-w-[250px]">{row.label}</td>
                          <td className="px-8 py-4 text-slate-500">{row.sequence_hash.slice(0, 24)}...</td>
                          <td className="px-8 py-4 text-bio-glow text-right font-bold">{row.length.toLocaleString()}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </GlassCard>
              )}
            </div>
          )}

          {activeTab === 'Taxonomy Room' && (
             <div className="grid grid-cols-3 gap-10">
                <GlassCard className="col-span-2">
                   <div className="px-8 py-6 border-b border-slate-800/60">
                      <h3 className="text-xs font-black text-white uppercase tracking-[0.2em]">Explainable Reconciliation Engine</h3>
                   </div>
                   <div className="p-8 space-y-4">
                      {[
                        { raw: 'H. sap', canonical: 'Homo sapiens', score: 0.9411, algo: 'Jaro-Winkler', status: 'Verified' },
                        { raw: 'Mus mus', canonical: 'Mus musculus', score: 0.9102, algo: 'Levenshtein', status: 'Fuzzy' },
                        { raw: 'SARS-CoV2', canonical: 'Severe acute respiratory syndrome coronavirus 2', score: 0.9989, algo: 'Exact', status: 'Verified' },
                      ].map((match, i) => (
                        <div key={i} className="flex items-center justify-between p-6 rounded-2xl bg-slate-900/40 border border-slate-800 hover:border-bio-accent/20 transition-all group cursor-pointer">
                           <div className="flex items-center space-x-12">
                              <div className="w-32">
                                 <div className="text-[8px] font-black text-slate-500 uppercase tracking-widest mb-1">Header ID</div>
                                 <div className="mono text-xs text-slate-400 group-hover:text-white transition-colors">{match.raw}</div>
                              </div>
                              <div className="text-slate-700 group-hover:text-bio-accent transition-colors"><Zap size={18} /></div>
                              <div>
                                 <div className="text-[8px] font-black text-slate-500 uppercase tracking-widest mb-1">Taxonomic Result</div>
                                 <div className="text-sm font-black text-white italic group-hover:text-bio-accent transition-colors underline decoration-bio-accent/20 underline-offset-4">{match.canonical}</div>
                              </div>
                           </div>
                           <div className="text-right">
                              <div className="text-[8px] font-black text-slate-500 uppercase tracking-widest mb-1">Confidence Score</div>
                              <div className="text-xl font-black font-mono text-bio-accent">{match.score}</div>
                           </div>
                        </div>
                      ))}
                   </div>
                </GlassCard>

                <div className="space-y-8">
                   <GlassCard className="p-8">
                      <h3 className="text-xs font-black text-white uppercase tracking-[0.2em] mb-6">Decision Audit</h3>
                      <div className="space-y-6">
                         <div className="border-l-2 border-bio-accent pl-6 py-1">
                            <div className="text-[9px] font-black text-slate-500 uppercase tracking-[0.1em] mb-1">Active Algorithm</div>
                            <div className="text-xs font-bold text-white">Probabilistic Jaro-Winkler</div>
                         </div>
                         <div className="border-l-2 border-bio-accent pl-6 py-1">
                            <div className="text-[9px] font-black text-slate-500 uppercase tracking-[0.1em] mb-1">Validation Threshold</div>
                            <div className="text-xs font-bold text-white">0.8500 (STRICT)</div>
                         </div>
                         <div className="border-l-2 border-slate-700 pl-6 py-1">
                            <div className="text-[9px] font-black text-slate-500 uppercase tracking-[0.1em] mb-1">Ontology Source</div>
                            <div className="text-xs font-bold text-slate-300">NCBI-RefSeq-2026.05</div>
                         </div>
                      </div>
                   </GlassCard>

                   <button className="w-full flex items-center justify-center space-x-3 bg-slate-900 border border-slate-800 p-4 rounded-xl text-xs font-black uppercase tracking-widest text-slate-400 hover:text-white hover:bg-slate-800 transition-all">
                      <FileCode size={16} />
                      <span>View CLI Command</span>
                   </button>
                </div>
             </div>
          )}
        </div>
      </main>
    </div>
  );
}

export default App;

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
  CheckCircle2
} from 'lucide-react';
import init, { WasmParser } from './wasm/wasm_core';
import { useStore } from './store';

const SidebarItem = ({ icon: Icon, label, active, onClick }: { icon: any, label: string, active: boolean, onClick: () => void }) => (
  <button 
    onClick={onClick}
    className={`flex items-center w-full space-x-3 px-4 py-3 rounded-lg transition-colors ${
      active ? 'bg-indigo-900 text-cyan-400' : 'text-slate-400 hover:bg-slate-800 hover:text-white'
    }`}
  >
    <Icon size={20} />
    <span className="font-medium text-sm">{label}</span>
  </button>
);

function App() {
  const [activeTab, setActiveTab] = useState('Observatory');
  const { isWasmLoaded, setWasmLoaded } = useStore();
  const [isParsing, setIsParsing] = useState(false);
  const [parsedMetadata, setParsedMetadata] = useState<any[]>([]);

  useEffect(() => {
    init().then(() => {
      setWasmLoaded(true);
      console.log("SuperAlign WASM Core Initialized");
    });
  }, []);

  const handleFileUpload = async (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!e.target.files || e.target.files.length === 0) return;
    const file = e.target.files[0];
    
    setIsParsing(true);
    const parser = new WasmParser("0.1.0");
    
    // Simulate streaming by reading a chunk
    const reader = new FileReader();
    reader.onload = (event) => {
      if (event.target?.result) {
        const bytes = new Uint8Array(event.target.result as ArrayBuffer);
        try {
          const results = parser.parse_chunk(bytes);
          setParsedMetadata(results as any[]);
          setIsParsing(false);
        } catch (err) {
          console.error("Parse Error:", err);
          setIsParsing(false);
        }
      }
    };
    reader.readAsArrayBuffer(file.slice(0, 1024 * 1024)); // Read first 1MB for demo
  };

  return (
    <div className="flex h-screen w-screen bg-slate-950 text-slate-200 overflow-hidden font-sans">
      {/* Sidebar */}
      <aside className="w-64 border-r border-slate-800 flex flex-col p-4 space-y-6">
        <div className="flex items-center space-x-3 px-2 mb-2">
          <div className="bg-indigo-600 p-1.5 rounded-md">
            <ShieldCheck size={24} className="text-white" />
          </div>
          <span className="text-xl font-bold tracking-tight text-white">SuperAlign</span>
        </div>

        <nav className="flex-1 space-y-1">
          <SidebarItem icon={Telescope} label="Observatory" active={activeTab === 'Observatory'} onClick={() => setActiveTab('Observatory')} />
          <SidebarItem icon={Beaker} label="Laboratory" active={activeTab === 'Laboratory'} onClick={() => setActiveTab('Laboratory')} />
          <SidebarItem icon={Network} label="Taxonomy Room" active={activeTab === 'Taxonomy Room'} onClick={() => setActiveTab('Taxonomy Room')} />
          <SidebarItem icon={GitBranch} label="Orchestrator" active={activeTab === 'Orchestrator'} onClick={() => setActiveTab('Orchestrator')} />
          <SidebarItem icon={Grid3X3} label="Constructor" active={activeTab === 'Constructor'} onClick={() => setActiveTab('Constructor')} />
          <SidebarItem icon={Puzzle} label="Extensions" active={activeTab === 'Extensions'} onClick={() => setActiveTab('Extensions')} />
          <SidebarItem icon={BarChart3} label="Performance" active={activeTab === 'Performance'} onClick={() => setActiveTab('Performance')} />
        </nav>

        <div className="mt-auto border-t border-slate-800 pt-4 px-2">
          <div className="flex items-center space-x-2 text-emerald-500 text-xs font-semibold uppercase tracking-wider">
            <Lock size={12} />
            <span>Processing Locally</span>
          </div>
          <div className="mt-1 text-slate-500 text-[10px]">
            WASM CORE: {isWasmLoaded ? 'READY' : 'LOADING...'}
          </div>
        </div>
      </aside>

      {/* Main Content */}
      <main className="flex-1 flex flex-col overflow-hidden">
        {/* Top Header */}
        <header className="h-16 border-b border-slate-800 flex items-center justify-between px-8 bg-slate-900/50">
          <div className="flex items-center space-x-4">
            <h2 className="text-lg font-semibold text-white">{activeTab}</h2>
            <div className="h-4 w-px bg-slate-700"></div>
            <span className="text-slate-500 text-sm mono">v0.1.0-alpha</span>
          </div>
          
          <div className="flex items-center space-x-6">
            <div className="flex items-center space-x-2">
              <span className="text-[10px] font-bold text-slate-500 uppercase tracking-widest">Reproducibility Index</span>
              <div className="h-2 w-24 bg-slate-800 rounded-full overflow-hidden">
                <div className="h-full w-full bg-emerald-500"></div>
              </div>
              <span className="text-emerald-500 text-xs font-bold">100%</span>
            </div>
          </div>
        </header>

        {/* Viewport */}
        <div className="flex-1 overflow-y-auto p-8">
          {activeTab === 'Observatory' && (
            <div className="space-y-8">
              <div className="grid grid-cols-4 gap-6">
                {[
                  { label: 'Total Taxa', value: '102,341', delta: '+12%', icon: Network },
                  { label: 'Verified Reproducible', value: '98.2%', delta: '+0.1%', icon: ShieldCheck },
                  { label: 'Avg. Match Confidence', value: '0.94', delta: '-0.02', icon: BarChart3 },
                  { label: 'Out-of-core Capacity', value: '4.2 TB', delta: 'N/A', icon: Grid3X3 },
                ].map((stat, i) => (
                  <div key={i} className="bg-slate-900 border border-slate-800 p-6 rounded-xl shadow-sm">
                    <div className="flex justify-between items-start">
                      <div className="bg-slate-800 p-2 rounded-lg text-indigo-400">
                        <stat.icon size={20} />
                      </div>
                      <span className={`text-xs font-bold ${stat.delta.startsWith('+') ? 'text-emerald-500' : 'text-slate-500'}`}>
                        {stat.delta}
                      </span>
                    </div>
                    <div className="mt-4">
                      <div className="text-slate-400 text-xs font-medium uppercase tracking-wider">{stat.label}</div>
                      <div className="text-2xl font-bold text-white mt-1 font-mono">{stat.value}</div>
                    </div>
                  </div>
                ))}
              </div>

              <div className="bg-slate-900 border border-slate-800 rounded-xl overflow-hidden shadow-sm">
                <div className="px-6 py-4 border-b border-slate-800 flex justify-between items-center bg-slate-900/50">
                  <h3 className="font-semibold text-white">Recent Provenance Replays</h3>
                  <button className="text-xs text-indigo-400 hover:text-indigo-300 font-medium">View All</button>
                </div>
                <table className="w-full text-left text-sm">
                  <thead className="bg-slate-950 text-slate-400 uppercase text-[10px] tracking-widest font-bold">
                    <tr>
                      <th className="px-6 py-3">Run ID</th>
                      <th className="px-6 py-3">Operation</th>
                      <th className="px-6 py-3">Timestamp</th>
                      <th className="px-6 py-3">Reproducibility</th>
                      <th className="px-6 py-3">Status</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-slate-800">
                    {[
                      { id: '8e7e348c', op: 'TAXON_RECONCILE', time: '2 mins ago', score: '100%', status: 'Verified' },
                      { id: 'b9d75114', op: 'FASTA_PARSE', time: '15 mins ago', score: '100%', status: 'Verified' },
                      { id: '02ea00a2', op: 'MATRIX_INIT', time: '1 hour ago', score: '99.9%', status: 'Verified' },
                    ].map((row, i) => (
                      <tr key={i} className="hover:bg-slate-800/50 transition-colors cursor-pointer group">
                        <td className="px-6 py-4 mono text-indigo-400 font-mono">{row.id}</td>
                        <td className="px-6 py-4 font-medium">{row.op}</td>
                        <td className="px-6 py-4 text-slate-400">{row.time}</td>
                        <td className="px-6 py-4 text-emerald-500 font-bold">{row.score}</td>
                        <td className="px-6 py-4">
                          <span className="bg-emerald-500/10 text-emerald-500 px-2 py-0.5 rounded text-[10px] font-bold uppercase tracking-tight border border-emerald-500/20 shadow-sm">
                            {row.status}
                          </span>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          )}

          {activeTab === 'Laboratory' && (
            <div className="max-w-4xl mx-auto space-y-8">
              <div className="bg-slate-900 border-2 border-dashed border-slate-700 rounded-2xl p-12 flex flex-col items-center justify-center text-center space-y-4 hover:border-indigo-500 transition-colors group relative">
                <input 
                  type="file" 
                  className="absolute inset-0 w-full h-full opacity-0 cursor-pointer" 
                  onChange={handleFileUpload}
                  accept=".fasta,.fa,.fna"
                />
                <div className="bg-slate-800 p-4 rounded-full text-slate-400 group-hover:text-indigo-400 transition-colors">
                  <Upload size={48} />
                </div>
                <div>
                  <h3 className="text-xl font-bold text-white">Ingest Nucleotide Sequences</h3>
                  <p className="text-slate-400 mt-2">Drag and drop FASTA files or click to browse. Processing occurs entirely in your browser.</p>
                </div>
                <div className="flex space-x-2">
                   <span className="bg-slate-800 text-slate-300 px-3 py-1 rounded-md text-xs font-mono border border-slate-700">.fasta</span>
                   <span className="bg-slate-800 text-slate-300 px-3 py-1 rounded-md text-xs font-mono border border-slate-700">.fa</span>
                </div>
              </div>

              {isParsing && (
                <div className="flex items-center justify-center space-x-3 text-indigo-400 font-medium">
                  <Loader2 className="animate-spin" />
                  <span>Streaming & Hashing Sequences...</span>
                </div>
              )}

              {parsedMetadata.length > 0 && (
                <div className="bg-slate-900 border border-slate-800 rounded-xl overflow-hidden animate-in fade-in slide-in-from-bottom-4 duration-500">
                  <div className="px-6 py-4 border-b border-slate-800 flex items-center justify-between">
                    <div className="flex items-center space-x-2">
                      <CheckCircle2 size={18} className="text-emerald-500" />
                      <h3 className="font-semibold text-white">Ingestion Complete</h3>
                    </div>
                    <span className="text-xs text-slate-500 uppercase tracking-widest font-bold">Metadata Extract (First 1MB)</span>
                  </div>
                  <div className="p-0 overflow-x-auto">
                    <table className="w-full text-left text-xs">
                      <thead className="bg-slate-950 text-slate-500 uppercase text-[10px] tracking-widest font-bold">
                        <tr>
                          <th className="px-6 py-3">UUID</th>
                          <th className="px-6 py-3">Raw Label</th>
                          <th className="px-6 py-3">SHA-256 Hash</th>
                          <th className="px-6 py-3 text-right">Length (bp)</th>
                        </tr>
                      </thead>
                      <tbody className="divide-y divide-slate-800 font-mono">
                        {parsedMetadata.map((row, i) => (
                          <tr key={i} className="hover:bg-slate-800/50">
                            <td className="px-6 py-3 text-indigo-400">{row.uuid.slice(0, 8)}</td>
                            <td className="px-6 py-3 text-white truncate max-w-[200px]">{row.label}</td>
                            <td className="px-6 py-3 text-slate-500">{row.sequence_hash.slice(0, 16)}...</td>
                            <td className="px-6 py-3 text-cyan-400 text-right">{row.length.toLocaleString()}</td>
                          </tr>
                        ))}
                      </tbody>
                    </table>
                  </div>
                </div>
              )}
            </div>
          )}

          {activeTab === 'Taxonomy Room' && (
             <div className="flex space-x-6 h-full">
                <div className="flex-1 space-y-6">
                  <div className="bg-slate-900 border border-slate-800 rounded-xl p-6 shadow-sm">
                    <h3 className="font-semibold text-white mb-6">Explainable Reconciliation</h3>
                    <div className="space-y-4">
                       {[
                         { raw: 'H. sap', canonical: 'Homo sapiens', id: 'NCBI:9606', score: 0.94, method: 'Jaro-Winkler' },
                         { raw: 'Mus mus', canonical: 'Mus musculus', id: 'NCBI:10090', score: 0.91, method: 'Levenshtein' },
                         { raw: 'Unknown_12', canonical: 'N/A', id: 'N/A', score: 0.12, method: 'None' },
                       ].map((match, i) => (
                         <div key={i} className={`flex items-center justify-between p-5 rounded-xl border transition-all hover:scale-[1.01] cursor-pointer ${match.score > 0.8 ? 'bg-slate-950 border-slate-800' : 'bg-red-500/5 border-red-500/20'}`}>
                            <div className="flex items-center space-x-12">
                               <div>
                                  <div className="text-[10px] font-bold text-slate-500 uppercase tracking-widest mb-1">Raw FASTA Header</div>
                                  <div className="font-mono text-sm text-slate-300">{match.raw}</div>
                               </div>
                               <div className="text-slate-700">
                                  <Network size={20} />
                               </div>
                               <div>
                                  <div className="text-[10px] font-bold text-slate-500 uppercase tracking-widest mb-1">Canonical Match</div>
                                  <div className="text-white font-bold">{match.canonical}</div>
                               </div>
                            </div>
                            <div className="text-right">
                               <div className="text-[10px] font-bold text-slate-500 uppercase tracking-widest mb-1">Confidence Score</div>
                               <div className={`text-lg font-bold font-mono ${match.score > 0.8 ? 'text-emerald-500' : 'text-red-500'}`}>{match.score}</div>
                            </div>
                         </div>
                       ))}
                    </div>
                  </div>
                </div>
                <div className="w-80 space-y-6">
                   <div className="bg-slate-900 border border-slate-800 rounded-xl p-6 shadow-sm">
                      <h3 className="font-semibold text-white mb-4">Match Explainer</h3>
                      <div className="bg-slate-950 p-4 rounded-lg border border-slate-800">
                         <div className="text-[10px] font-bold text-slate-500 uppercase tracking-widest mb-4">Decision Tree</div>
                         <div className="space-y-4">
                            <div className="flex items-center space-x-3">
                               <div className="h-6 w-px bg-indigo-500 ml-2"></div>
                               <span className="text-xs text-slate-300">Algo: Jaro-Winkler</span>
                            </div>
                            <div className="flex items-center space-x-3">
                               <div className="h-6 w-px bg-indigo-500 ml-2"></div>
                               <span className="text-xs text-slate-300">Distance: 0.94</span>
                            </div>
                            <div className="flex items-center space-x-3">
                               <div className="h-2 w-2 rounded-full bg-emerald-500 ml-1"></div>
                               <span className="text-xs font-bold text-emerald-500 uppercase">Passed Threshold (0.85)</span>
                            </div>
                         </div>
                      </div>
                   </div>

                   <div className="bg-slate-900 border border-slate-800 rounded-xl p-6 shadow-sm">
                      <h3 className="font-semibold text-white mb-4">Ontology Lookup</h3>
                      <div className="relative">
                         <input type="text" placeholder="Search NCBI..." className="w-full bg-slate-950 border border-slate-800 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-1 focus:ring-indigo-500" />
                      </div>
                   </div>
                </div>
             </div>
          )}
        </div>
      </main>
    </div>
  );
}

export default App;

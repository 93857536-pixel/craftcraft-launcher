import { useState, useEffect, useRef } from 'react';
import { getInstances, readLogTail } from '../api';
import type { Instance } from '../types';

interface P { selectedId: string | null; onSel: (id: string | null) => void; }
export default function LogPage({ selectedId, onSel }: P) {
  const [items, setItems] = useState<Instance[]>([]);
  const [lines, setLines] = useState<string[]>([]);
  const auto = useRef(true);
  useEffect(() => {
    getInstances().then(setItems).catch(() => {});
    if (selectedId) poll(selectedId);
    else setLines([]);
    const t = setInterval(() => { if (selectedId && auto.current) poll(selectedId); }, 2000);
    return () => clearInterval(t);
  }, [selectedId]);
  const poll = (id: string) => readLogTail(id, 300).then(setLines).catch(() => {});
  if (!selectedId) return <div className="card" style={{textAlign:'center',color:'var(--muted)'}}>Select an instance from the Instances tab to view logs.</div>;
  const inst = items.find((i) => i.id === selectedId);
  const highlight = (line: string) => {
    if (/ERROR|Exception|Caused by/.test(line)) return <span className="err">{line}</span>;
    if (/WARN/.test(line)) return <span className="warn">{line}</span>;
    if (/Done/.test(line)) return <span className="info">{line}</span>;
    return line;
  };
  return (
    <div>
      <div style={{display:'flex',alignItems:'center',gap:8,marginBottom:8}}>
        <select value={selectedId} onChange={(e) => onSel(e.target.value || null)} style={{minWidth:200}}>
          {items.map((i) => <option key={i.id} value={i.id}>{i.name} · {i.version}</option>)}
        </select>
        <label style={{display:'inline-flex',alignItems:'center',gap:4,fontSize:12,color:'var(--muted)',cursor:'pointer'}}>
          <input type="checkbox" checked={auto.current} onChange={(e) => { auto.current = e.target.checked; }} /> auto-refresh
        </label>
        <button onClick={() => poll(selectedId)}>Refresh</button>
      </div>
      <pre className="log">
        {inst && <div style={{color:'var(--muted)',fontSize:11}}>{inst.game_directory}/logs/latest.log</div>}
        {lines.map((l, i) => <div key={i}>{highlight(l)}</div>)}
        {!lines.length && <div style={{color:'var(--muted)'}}>No log yet. Launch the game to generate one.</div>}
      </pre>
    </div>
  );
}

import { useState, useEffect } from 'react';
import { getInstances, deleteInstance, launchInstance, listMinecraftVersions } from '../api';
import type { Instance, MojangVersionInfo } from '../types';

interface P {
  onLaunch: (id: string) => void;
  onSelect: (id: string | null) => void;
}
export default function InstancesPage({ onLaunch, onSelect }: P) {
  const [items, setItems] = useState<Instance[]>([]);
  const [versions, setVersions] = useState<MojangVersionInfo[]>([]);
  const [loading, setLoading] = useState(false);
  useEffect(() => { load(); list(); }, []);
  const load = () => getInstances().then(setItems).catch(console.error);
  const list = () => listMinecraftVersions().then((v) => setVersions(v.filter((x) => x.type === 'release'))).catch(() => {});
  const del = async (id: string) => {
    if (!confirm('Delete this instance? Game data will be kept.')) return;
    await deleteInstance(id);
    load();
  };
  const launch = async (id: string) => {
    setLoading(true);
    try {
      const r = await launchInstance(id);
      if (r.success) load(); else alert('Launch failed: ' + r.message);
    } finally { setLoading(false); }
  };
  if (!items.length) return <div className="card" style={{textAlign:'center',padding:'48px',color:'var(--muted)'}}>
    No instances yet.<br/><button className="primary" style={{marginTop:12}} onClick={()=>onSelect('x')}>Create one</button>
  </div>;
  return <div className="grid">
    {items.map((i) => (
      <div key={i.id} className="card">
        <div style={{display:'flex',alignItems:'center',gap:8}}>
          <span style={{fontSize:28}}>⛏</span>
          <div style={{flex:1}}>
            <div style={{fontWeight:600}}>{i.name}</div>
            <div style={{fontSize:12,color:'var(--muted)'}}>{i.version} {i.loader!=='vanilla'?' ('+i.loader+')':''}</div>
          </div>
          <span className="badge green">{i.memory_max_mb}MB</span>
        </div>
        <div style={{marginTop:8,fontSize:12,color:'var(--muted)'}}>
          {i.account_name} · last played: {i.last_played_at ? new Date(i.last_played_at * 1000).toLocaleString() : '—'}
        </div>
        <div className="btn-row">
          <button className="primary" disabled={loading} onClick={() => launch(i.id)}>▶ Launch</button>
          <button className="danger" onClick={() => del(i.id)}>🗑</button>
        </div>
      </div>
    ))}
  </div>;
}

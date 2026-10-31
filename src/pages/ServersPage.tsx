import { useState, useEffect } from 'react';
import { listServers, addServer, removeServer, toggleServerFavorite, queryServer } from '../api';
import type { ServerBookmark, ServerStatus } from '../types';

export default function ServersPage() {
  const [items, setItems] = useState<ServerBookmark[]>([]);
  const [status, setStatus] = useState<Record<string, ServerStatus>>({});
  const [addr, setAddr] = useState('');
  const [name, setName] = useState('');
  const [busy, setBusy] = useState<string | null>(null);
  useEffect(() => { listServers().then(setItems).catch(() => {}); }, []);
  const query = async (a: string) => {
    setBusy(a);
    try {
      const st = await queryServer(a);
      setStatus((s) => ({ ...s, [a]: st }));
    } finally { setBusy(null); }
  };
  const add = () => {
    if (!addr.trim()) return;
    addServer(addr.trim(), name.trim() || addr.trim()).then((b) => { setItems((x) => [...x, b]); setAddr(''); setName(''); });
  };
  return (
    <div className="card">
      <h2 style={{marginTop:0}}>Servers</h2>
      <form onSubmit={(e) => { e.preventDefault(); add(); }} style={{display:'flex',gap:8,marginBottom:12}}>
        <input placeholder="address:port" value={addr} onChange={(e) => setAddr(e.target.value)} style={{flex:1}} />
        <input placeholder="name (optional)" value={name} onChange={(e) => setName(e.target.value)} />
        <button type="submit" className="primary">Add</button>
      </form>
      {items.map((s) => {
        const st = status[s.address];
        return (
          <div key={s.id} style={{display:'flex',alignItems:'center',gap:8,borderTop:'1px solid var(--card2)',paddingTop:8}}>
            <span>{s.is_favorite ? '❤' : '♡'}</span>
            <div style={{flex:1}}>
              <div>{s.name} <span className="tag">{s.address}</span></div>
              {st ? (
                <div style={{fontSize:12,color:'var(--muted)'}}>
                  {st.reachable ? `${st.motd} · ${st.online}/${st.max} · ${st.version}` : `offline (${st.error})`}
                </div>
              ) : null}
            </div>
            <button className="primary" disabled={!!busy} onClick={() => query(s.address)}>{busy === s.address ? '...' : 'Test'}</button>
            <button onClick={() => toggleServerFavorite(s.id).then(() => setItems((x) => x.map((i) => i.id === s.id ? { ...i, is_favorite: !i.is_favorite } : i)))}>★</button>
            <button className="danger" onClick={() => { removeServer(s.id); setItems((x) => x.filter((i) => i.id !== s.id)); }}>✗</button>
          </div>
        );
      })}
      {!items.length && <div style={{color:'var(--muted)',fontSize:13}}>No servers. Add one above.</div>}
    </div>
  );
}

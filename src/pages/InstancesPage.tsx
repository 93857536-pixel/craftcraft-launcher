import { useState, useEffect, useRef } from 'react';
import { getInstances, deleteInstance, launchInstance } from '../api';
import type { Instance } from '../types';

interface P {
  onLaunch: (id: string) => void;
  onSelect: (id: string | null) => void;
  onNew: () => void;
}

export default function InstancesPage({ onLaunch, onSelect, onNew }: P) {
  const [items, setItems] = useState<Instance[]>([]);
  const [loading, setLoading] = useState(false);
  const [selId, setSelId] = useState<string | null>(null);

  useEffect(() => { getInstances().then(setItems).catch(console.error); }, []);

  const del = async (id: string) => {
    if (!confirm('Delete this instance? Game data will be kept.')) return;
    await deleteInstance(id);
    setItems((x) => x.filter((i) => i.id !== id));
    if (selId === id) setSelId(null);
  };

  const launch = async (id: string) => {
    setLoading(true);
    try {
      const r = await launchInstance(id);
      if (r.success) {
        setItems((x) => x.map((i) => i.id === id ? { ...i, last_played_at: Date.now() / 1000 } : i));
        onLaunch(id);
      } else alert('Launch failed: ' + r.message);
    } finally { setLoading(false); }
  };

  if (!items.length) {
    return (
      <div className="empty">
        <div className="empty-icon">⛏</div>
        <h3>No instances yet</h3>
        <p>Create your first Minecraft instance to get started.</p>
        <button className="primary" onClick={onNew} style={{ marginTop: 4, padding: '8px 20px', fontSize: 13 }}>
          + New Instance
        </button>
      </div>
    );
  }

  return (
    <>
      <div className="page-title">
        Instances
        <span className="text-muted" style={{ fontWeight: 400, marginLeft: 6, fontSize: 12.5 }}>{items.length} instance{items.length > 1 ? 's' : ''}</span>
      </div>
      <div className="grid">
        {items.map((i) => (
          <div
            key={i.id}
            className={`inst-card${selId === i.id ? ' selected' : ''}`}
            onClick={() => onSelect(i.id)}
          >
            <div className="inst-header">
              <div className="inst-icon">⛏</div>
              <div style={{ flex: 1, minWidth: 0 }}>
                <div className="inst-name">{i.name}</div>
                <div className="inst-version">{i.version}{i.loader !== 'vanilla' ? ` · ${i.loader}` : ''}</div>
              </div>
              <span className="badge green">{i.memory_max_mb}MB</span>
            </div>
            <div className="inst-meta">
              <span className="tag">{i.account_name}</span>
              {i.last_played_at && (
                <span className="text-xs text-muted" style={{ alignSelf: 'center' }}>
                  {new Date(i.last_played_at * 1000).toLocaleString()}
                </span>
              )}
            </div>
            <div style={{ display: 'flex', gap: 6, marginTop: 12, paddingTop: 10, borderTop: '1px solid var(--border)' }}>
              <button className="primary" disabled={loading} onClick={(e) => { e.stopPropagation(); launch(i.id); }} style={{ flex: 1, justifyContent: 'center' }}>
                ▶ Launch
              </button>
              <button className="secondary" onClick={(e) => { e.stopPropagation(); onSelect(i.id); }}>Edit</button>
              <button className="danger" onClick={(e) => { e.stopPropagation(); del(i.id); }}>Del</button>
            </div>
          </div>
        ))}
        <div
          className="inst-card"
          style={{ borderStyle: 'dashed', cursor: 'pointer', display: 'flex', alignItems: 'center', justifyContent: 'center', minHeight: 140, color: 'var(--text-dim)' }}
          onClick={onNew}
        >
          <div style={{ textAlign: 'center' }}>
            <div style={{ fontSize: 20, marginBottom: 4, opacity: 0.5 }}>+</div>
            <div style={{ fontSize: 12 }}>New Instance</div>
          </div>
        </div>
      </div>
    </>
  );
}

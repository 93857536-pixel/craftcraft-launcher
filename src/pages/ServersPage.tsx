import { useState, useEffect } from 'react';
import { listServers, addServer, removeServer, toggleServerFavorite, queryServer } from '../api';
import { useT } from '../i18n';
import type { ServerBookmark, ServerStatus } from '../types';

export default function ServersPage() {
  const t = useT();
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
    addServer(addr.trim(), name.trim() || addr.trim())
      .then((b) => { setItems((x) => [...x, b]); setAddr(''); setName(''); });
  };

  return (
    <>
      <div className="page-title">{t('servers.title')}</div>
      <div className="card">
        <div className="card-header"><h2>{t('servers.addServer')}</h2></div>
        <form onSubmit={(e) => { e.preventDefault(); add(); }} className="flex gap-8">
          <input placeholder={t('servers.address')} value={addr} onChange={(e) => setAddr(e.target.value)} style={{ flex: 1 }} />
          <input placeholder={t('servers.name')} value={name} onChange={(e) => setName(e.target.value)} style={{ width: 140 }} />
          <button type="submit" className="primary" style={{ padding: '6px 14px' }}>{t('common.add')}</button>
        </form>
      </div>

      <div className="card" style={{ padding: 0 }}>
        {items.map((s, idx) => {
          const st = status[s.address];
          return (
            <div key={s.id} style={{
              display: 'flex', alignItems: 'center', gap: 10,
              padding: '10px 14px',
              borderBottom: idx < items.length - 1 ? '1px solid var(--border)' : 'none',
            }}>
              <button className="secondary" style={{ padding: '3px 7px', fontSize: 11 }}
                onClick={() => toggleServerFavorite(s.id)
                  .then(() => setItems((x) => x.map((i) => i.id === s.id ? { ...i, is_favorite: !i.is_favorite } : i)))}
              >
                {s.is_favorite ? '♥' : '♡'}
              </button>
              <div style={{ flex: 1, minWidth: 0 }}>
                <div className="flex items-center gap-8">
                  <span style={{ fontWeight: 500, color: 'var(--text)', fontSize: 12.5 }}>{s.name}</span>
                  <span className="tag">{s.address}</span>
                </div>
                {st ? (
                  <div className="text-xs mt-8" style={{ color: 'var(--text-secondary)' }}>
                    {st.reachable
                      ? <><span className="badge green">{t('servers.online')}</span><span style={{ marginLeft: 6 }}>{t('servers.players', { online: st.online ?? 0, max: st.max ?? 0 })}</span></>
                      : <><span className="badge red">{t('servers.offline')}</span><span style={{ marginLeft: 6 }}>{st.error}</span></>}
                  </div>
                ) : (
                  <div className="text-xs mt-8" style={{ color: 'var(--text-tertiary)' }}>{t('servers.clickTest')}</div>
                )}
              </div>
              <button className="secondary" disabled={!!busy} style={{ padding: '3px 10px', fontSize: 11 }} onClick={() => query(s.address)}>
                {busy === s.address ? '…' : t('common.test')}
              </button>
              <button className="danger" style={{ padding: '3px 7px', fontSize: 11 }}
                onClick={() => { removeServer(s.id); setItems((x) => x.filter((i) => i.id !== s.id)); }}>✕</button>
            </div>
          );
        })}
        {!items.length && (
          <div className="text-muted" style={{ textAlign: 'center', padding: '28px 0', fontSize: 12.5 }}>
            {t('servers.empty')}
          </div>
        )}
      </div>
    </>
  );
}

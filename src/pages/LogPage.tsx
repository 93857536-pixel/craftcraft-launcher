import { useState, useEffect, useRef } from 'react';
import { getInstances, readLogTail } from '../api';
import { useT } from '../i18n';
import type { Instance } from '../types';

interface P { selectedId: string | null; onSel: (id: string | null) => void; onNew: () => void; }

export default function LogPage({ selectedId, onSel, onNew }: P) {
  const t = useT();
  const [items, setItems] = useState<Instance[]>([]);
  const [lines, setLines] = useState<string[]>([]);
  const auto = useRef(true);
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    getInstances().then(setItems).catch(() => {});
    if (selectedId) poll(selectedId);
    else setLines([]);
    const t2 = setInterval(() => { if (selectedId && auto.current) poll(selectedId); }, 2000);
    return () => clearInterval(t2);
  }, [selectedId]);

  useEffect(() => {
    if (auto.current && bottomRef.current) bottomRef.current.scrollIntoView({ behavior: 'smooth' });
  }, [lines]);

  const poll = (id: string) => readLogTail(id, 300).then(setLines).catch(() => {});

  if (!selectedId) {
    return (
      <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', height: '60vh', gap: 16 }}>
        <div style={{
          width: 56, height: 56,
          borderRadius: 14,
          background: 'linear-gradient(135deg, rgba(94,106,210,0.15), rgba(94,106,210,0.03))',
          border: '1px solid var(--border-bright)',
          display: 'flex', alignItems: 'center', justifyContent: 'center',
          fontSize: 22, opacity: 0.7,
        }}>📋</div>
        <div style={{ textAlign: 'center' }}>
          <div style={{ fontSize: 15, fontWeight: 600, color: 'var(--text)', marginBottom: 4 }}>{t('logs.noSelection')}</div>
          <div style={{ fontSize: 12.5, color: 'var(--text-secondary)' }}>{t('logs.noSelectionDesc')}</div>
        </div>
        {!items.length
          ? <button className="primary" onClick={onNew} style={{ marginTop: 8, padding: '8px 20px', fontSize: 13 }}>{t('logs.createInstance')}</button>
          : <button className="secondary" onClick={() => onSel(items[0].id)} style={{ marginTop: 8, padding: '8px 20px', fontSize: 13 }}>{t('logs.selectInstance', { name: items[0].name })}</button>
        }
      </div>
    );
  }

  const inst = items.find((i) => i.id === selectedId);
  const highlight = (line: string) => {
    if (/ERROR|Exception|Caused by/.test(line)) return <span className="err">{line}</span>;
    if (/WARN/.test(line)) return <span className="warn">{line}</span>;
    if (/Done|Saving|Loaded/.test(line)) return <span className="info">{line}</span>;
    if (/debug|Debug/.test(line)) return <span className="debug">{line}</span>;
    return line;
  };

  return (
    <>
      <div className="page-title">
        {t('logs.title')}
        {inst && <span className="text-muted" style={{ fontWeight: 400, marginLeft: 6, fontSize: 12.5 }}>{inst.name}</span>}
      </div>
      <div className="card" style={{ padding: 0, overflow: 'hidden' }}>
        <div className="card-header" style={{ padding: '10px 14px', borderBottom: '1px solid var(--border)' }}>
          <div className="flex items-center gap-12">
            <select value={selectedId || ''} onChange={(e) => onSel(e.target.value || null)} style={{ minWidth: 180, maxWidth: 240 }}>
              {items.map((i) => <option key={i.id} value={i.id}>{i.name} · {i.version}</option>)}
            </select>
            <label className="flex items-center gap-8" style={{ flexDirection: 'row', fontSize: 11.5, color: 'var(--text-secondary)', cursor: 'pointer', textTransform: 'none', letterSpacing: 0 }}>
              <input type="checkbox" checked={auto.current} onChange={(e) => { auto.current = e.target.checked; }} />
              {t('logs.autoRefresh')}
            </label>
          </div>
          <button className="secondary" style={{ padding: '4px 10px', fontSize: 11 }} onClick={() => poll(selectedId!)}>↻ {t('common.refresh')}</button>
        </div>
        <pre className="log" style={{ margin: 0, borderRadius: 0, borderTop: '1px solid var(--border)' }}>
          {inst && (
            <div className="text-xs text-muted" style={{ marginBottom: 6, opacity: 0.6, fontFamily: 'ui-monospace, monospace' }}>
              {inst.game_directory}/logs/latest.log
            </div>
          )}
          {lines.map((l, i) => <div key={i}>{highlight(l)}</div>)}
          {!lines.length && (
            <div className="text-muted" style={{ fontStyle: 'italic' }}>{t('logs.empty')}</div>
          )}
          <div ref={bottomRef} />
        </pre>
      </div>
    </>
  );
}

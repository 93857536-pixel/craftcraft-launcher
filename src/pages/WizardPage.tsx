import { useState, useEffect, useMemo } from 'react';
import { createInstance, listMinecraftVersions, listJava, installJava } from '../api';
import type { MojangVersionInfo, JavaVersion } from '../types';

interface P { onDone: () => void; }

export default function WizardPage({ onDone }: P) {
  const [name, setName] = useState('');
  const [ver, setVer] = useState('');
  const [loader, setLoader] = useState('vanilla' as const);
  const [account, setAccount] = useState('Player');
  const [memoryMax, setMemoryMax] = useState(6144);
  const [versions, setVersions] = useState<MojangVersionInfo[]>([]);
  const [javas, setJavas] = useState<JavaVersion[]>([]);
  const [downloading, setDownloading] = useState<string | null>(null);

  useEffect(() => {
    listMinecraftVersions().then((v) => setVersions(v.filter((x) => x.type === 'release'))).catch(() => {});
    listJava().then(setJavas).catch(() => {});
  }, []);

  const picked = useMemo(() => versions.find((v) => v.id === ver), [versions, ver]);
  const hasJava = javas.length > 0;

  const start = () => {
    if (!name.trim()) return;
    createInstance({ name: name.trim(), version: picked?.id ?? ver, loader, account_name: account })
      .then(() => onDone())
      .catch((e: any) => alert('Create failed: ' + e));
  };

  const doInstall = async (verStr: string) => {
    setDownloading(verStr);
    try {
      const installed = await installJava(verStr);
      setJavas((j) => [...j, installed]);
    } finally { setDownloading(null); }
  };

  return (
    <>
      <div className="page-title">Create Instance</div>
      <div className="card" style={{ maxWidth: 560 }}>
        <form onSubmit={(e) => { e.preventDefault(); start(); }}>
          <div className="form-grid">
            <div className="full">
              <label>Name
                <input value={name} onChange={(e) => setName(e.target.value)} placeholder="My Survival World" required />
              </label>
            </div>
            <div>
              <label>Version
                <input value={ver} onChange={(e) => setVer(e.target.value)} placeholder="1.21" />
              </label>
              {picked && (
                <div className="text-xs mt-8" style={{ color: 'var(--text-dim)' }}>
                  <span className="badge green">{picked.type}</span> · {picked.id}
                </div>
              )}
              {!picked && ver && (
                <div className="text-xs mt-8" style={{ color: 'var(--warn)' }}>⚠ Not in manifest — used as-is</div>
              )}
            </div>
            <div>
              <label>Loader
                <select value={loader} onChange={(e) => setLoader(e.target.value as any)}>
                  <option value="vanilla">Vanilla</option>
                  <option value="forge">Forge</option>
                  <option value="fabric">Fabric</option>
                  <option value="quilt">Quilt</option>
                  <option value="neoforge">NeoForge</option>
                </select>
              </label>
            </div>
            <div>
              <label>Account Name
                <input value={account} onChange={(e) => setAccount(e.target.value)} />
              </label>
            </div>
            <div>
              <label>
                Max Memory (MB)
                <input type="range" min={512} max={16384} step={512} value={memoryMax}
                  onChange={(e) => setMemoryMax(Number(e.target.value))}
                  style={{ width: '100%', accentColor: 'var(--accent)', marginTop: 4 }} />
              </label>
              <div className="text-xs mt-8" style={{ textAlign: 'right', color: 'var(--text-dim)', fontFamily: 'ui-monospace, monospace' }}>
                {memoryMax} MB
              </div>
            </div>
            <div>
              <label>Java Runtime</label>
              <div className="text-xs mt-8 mb-8" style={{ color: 'var(--text-dim)' }}>
                {hasJava ? `${javas.length} JDK(s) available` : 'No JDK found — download one below'}
              </div>
              {!hasJava && (
                <div className="flex gap-8" style={{ marginBottom: 8 }}>
                  {[8, 17, 21].map((v) => (
                    <button key={v} className="secondary" disabled={!!downloading}
                      onClick={() => doInstall(String(v))}>
                      {downloading === String(v) ? '…' : `Temurin ${v}`}
                    </button>
                  ))}
                </div>
              )}
              {hasJava && javas.slice(0, 3).map((j) => (
                <div key={j.id} className="flex items-center gap-8" style={{ padding: '4px 0', fontSize: 11.5, color: 'var(--text-sub)' }}>
                  <span className="badge green">✓</span>
                  <span style={{ fontFamily: 'ui-monospace, monospace' }}>{j.version}</span>
                  <span style={{ color: 'var(--text-dim)' }}>{j.path.split('/').pop()}</span>
                </div>
              ))}
            </div>
          </div>
          <div className="divider" />
          <div className="flex justify-between items-center">
            <button type="button" className="secondary" onClick={onDone}>Cancel</button>
            <button type="submit" className="primary" disabled={!name.trim()}>Create Instance</button>
          </div>
        </form>
      </div>
    </>
  );
}

import { useState, useEffect, useRef } from 'react';
import { listJava, installJava } from '../api';
import type { JavaVersion } from '../types';

export default function JavaPage() {
  const [javas, setJavas] = useState<JavaVersion[]>([]);
  const [downloading, setDownloading] = useState<string | null>(null);
  const didLoad = useRef(false);

  useEffect(() => {
    if (!didLoad.current) { didLoad.current = true; listJava().then(setJavas).catch(() => {}); }
  }, []);

  const doInstall = async (ver: string) => {
    setDownloading(ver);
    try {
      const installed = await installJava(ver);
      setJavas((j) => [...j, installed]);
    } finally { setDownloading(null); }
  };

  return (
    <>
      <div className="page-title">Java Runtime</div>
      <div className="card">
        <div className="card-header">
          <h2>Download Temurin</h2>
        </div>
        <div className="flex gap-8">
          {[8, 17, 21].map((v) => (
            <button key={v} className="secondary" disabled={!!downloading}
              onClick={() => doInstall(String(v))}>
              {downloading === String(v) ? 'Downloading…' : `Temurin ${v}`}
            </button>
          ))}
        </div>
      </div>

      <div className="card" style={{ padding: 0 }}>
        <div className="card-header" style={{ padding: '12px 16px', borderBottom: '1px solid var(--border)' }}>
          <h2>Installed JDKs</h2>
          <span className="badge accent">{javas.length}</span>
        </div>
        <table>
          <thead>
            <tr>
              <th>Version</th>
              <th>Path</th>
              <th>Vendor</th>
              <th>Arch</th>
              <th>Type</th>
            </tr>
          </thead>
          <tbody>
            {javas.map((j) => (
              <tr key={j.id}>
                <td><span className="badge green">{j.version}</span></td>
                <td style={{ fontFamily: 'ui-monospace, monospace', fontSize: 11, color: 'var(--text-secondary)' }}>
                  {j.path.length > 44 ? j.path.slice(0, 44) + '…' : j.path}
                </td>
                <td>{j.vendor}</td>
                <td>{j.architecture}</td>
                <td>
                  {j.is_managed
                    ? <span className="badge green">Managed</span>
                    : <span className="badge blue">System</span>}
                </td>
              </tr>
            ))}
            {!javas.length && (
              <tr>
                <td colSpan={5} style={{ textAlign: 'center', padding: '20px 0', color: 'var(--text-tertiary)' }}>
                  No JDKs found. Download one above.
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
    </>
  );
}

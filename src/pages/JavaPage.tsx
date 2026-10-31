import { useState, useEffect, useRef } from 'react';
import { listJava, installJava } from '../api';
import type { JavaVersion } from '../types';

export default function JavaPage() {
  const [javas, setJavas] = useState<JavaVersion[]>([]);
  const [downloading, setDownloading] = useState<string | null>(null);
  const didLoad = useRef(false);
  const load = () => { if (!didLoad.current) { didLoad.current = true; listJava().then(setJavas).catch(() => {}); } };
  useEffect(load, []);
  const doInstall = async (ver: string) => {
    setDownloading(ver);
    try {
      const installed = await installJava(ver);
      setJavas((j) => [...j, installed]);
    } finally { setDownloading(null); }
  };
  return (
    <div className="card">
      <h2 style={{marginTop:0}}>Managed Java</h2>
      <div style={{display:'flex',gap:8,flexWrap:'wrap'}}>
        {[8, 17, 21].map((v) => (
          <button key={v} className="primary" disabled={!!downloading} onClick={() => doInstall(String(v))}>
            {downloading === String(v) ? 'Downloading...' : `Download Temurin ${v}`}
          </button>
        ))}
      </div>
      <table style={{width:'100%',borderCollapse:'collapse',marginTop:16,fontSize:13}}>
        <thead><tr style={{textAlign:'left',color:'var(--muted)'}}><th>Version</th><th>Path</th><th>Vendor</th><th>Arch</th><th>Managed</th></tr></thead>
        <tbody>
          {javas.map((j) => (
            <tr key={j.id} style={{borderTop:'1px solid var(--card2)'}}>
              <td>{j.version}</td>
              <td style={{fontFamily:'var(--mono)',fontSize:11}}>{j.path.slice(0, 40)}...</td>
              <td>{j.vendor}</td>
              <td>{j.architecture}</td>
              <td><span className="badge green">{j.is_managed ? 'yes' : 'no'}</span></td>
            </tr>
          ))}
          {!javas.length && <tr><td colSpan={5} style={{textAlign:'center',color:'var(--muted)'}}>No managed JDKs. Download one above.</td></tr>}
        </tbody>
      </table>
    </div>
  );
}

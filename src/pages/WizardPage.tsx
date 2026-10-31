import { useState, useEffect, useMemo } from 'react';
import { createInstance, listMinecraftVersions, listJava, installJava, recommendJava } from '../api';
import type { MojangVersionInfo, JavaVersion, JavaRecommendation } from '../types';

interface P { onDone: () => void; }
export default function WizardPage({ onDone }: P) {
  const [step, setStep] = useState(0);
  const [name, setName] = useState('');
  const [ver, setVer] = useState('');
  const [loader, setLoader] = useState('vanilla');
  const [account, setAccount] = useState('Player');
  const [memoryMax, setMemoryMax] = useState(6144);
  const [versions, setVersions] = useState<MojangVersionInfo[]>([]);
  const [javas, setJavas] = useState<JavaVersion[]>([]);
  const [rec, setRec] = useState<JavaRecommendation | null>(null);
  const [downloading, setDownloading] = useState<string | null>(null);
  const [search, setSearch] = useState('');
  useEffect(() => { listMinecraftVersions().then((v) => setVersions(v.filter((x) => x.type === 'release'))).catch(() => {}); listJava().then(setJavas).catch(() => {}); }, []);
  const picked = useMemo(() => versions.find((v) => v.id === ver), [versions, ver]);
  const filtered = useMemo(() => search ? versions.filter((v) => v.id.includes(search)) : versions, [versions, search]);
  const start = () => createInstance({ name, version: picked?.id ?? ver, loader, account_name: account })
    .then((inst) => { setStep(99); onDone(); })
    .catch((e: any) => alert('Create failed: ' + e));
  const doInstall = async (ver: string) => {
    setDownloading(ver);
    try {
      const installed = await installJava(ver);
      setJavas((j) => [...j, installed]);
    } finally { setDownloading(null); }
  };
  return (
    <div className="card" style={{maxWidth:600}}>
      <h2 style={{marginTop:0}}>Create Instance</h2>
      <form onSubmit={(e) => { e.preventDefault(); start(); }}>
        <label>Name<input value={name} onChange={(e) => setName(e.target.value)} required /></label>
        <label>
          Version
          <input value={ver} onChange={(e) => { setVer(e.target.value); setRec(null); }} placeholder="e.g. 1.21" />
          {picked && <div style={{fontSize:12,color:'var(--muted)'}}>type: {picked.type} url: {picked.url?.slice(0, 40)}...</div>}
          {ver && !picked && <div style={{fontSize:12,color:'var(--warn)'}}>Not in list yet — will be used as-is.</div>}
        </label>
        <label>Minecraft account name<input value={account} onChange={(e) => setAccount(e.target.value)} /></label>
        <label>Memory (MB max)
          <input type="range" min={512} max={16384} step={512} value={memoryMax} onChange={(e) => setMemoryMax(Number(e.target.value))} />
          {memoryMax} MB
        </label>
        <label>
          Java version (auto)
          {rec ? <div style={{fontSize:12,color:'var(--muted)'}}>{rec.explanation} · recommended: Java {rec.recommended_java}</div> : null}
        </label>
        <button type="submit" className="primary">Create & Launch Later</button>
      </form>
    </div>
  );
}

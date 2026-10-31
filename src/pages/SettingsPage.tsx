import { useState, useEffect } from 'react';
import { getSettings, saveSettings } from '../api';
import type { AppSettings } from '../types';

interface P { settings: AppSettings; setSettings: (s: AppSettings) => void; }
export default function SettingsPage({ settings, setSettings }: P) {
  const [memMin, setMemMin] = useState(settings.default_memory_min_mb);
  const [memMax, setMemMax] = useState(settings.default_memory_max_mb);
  const [openOnLaunch, setOpenOnLaunch] = useState(settings.open_log_on_launch);
  useEffect(() => { getSettings().then((s) => { setSettings(s); setMemMax(s.default_memory_max_mb); setMemMin(s.default_memory_min_mb); setOpenOnLaunch(s.open_log_on_launch); }).catch(() => {}); }, []);
  const save = () => {
    const s: AppSettings = { ...settings, default_memory_min_mb: memMin, default_memory_max_mb: memMax, open_log_on_launch: openOnLaunch };
    setSettings(s);
    saveSettings(s).catch(console.error);
  };
  return (
    <div className="card" style={{maxWidth:480}}>
      <h2 style={{marginTop:0}}>Settings</h2>
      <label>Default max memory (MB)
        <input type="number" value={memMax} min={512} max={32768} step={512} onChange={(e) => setMemMax(Number(e.target.value))} />
      </label>
      <label>Default min memory (MB)
        <input type="number" value={memMin} min={256} max={memMax} step={256} onChange={(e) => setMemMin(Number(e.target.value))} />
      </label>
      <label style={{flexDirection:'row',alignItems:'center',gap:8}}>
        <input type="checkbox" checked={openOnLaunch} onChange={(e) => setOpenOnLaunch(e.target.checked)} />
        Auto-switch to Logs tab after launch
      </label>
      <button className="primary" onClick={save}>Save</button>
    </div>
  );
}

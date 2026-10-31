import { useState, useEffect, useRef } from 'react';
import InstancesPage from './pages/InstancesPage';
import WizardPage from './pages/WizardPage';
import JavaPage from './pages/JavaPage';
import LogPage from './pages/LogPage';
import ServersPage from './pages/ServersPage';
import SettingsPage from './pages/SettingsPage';
import type { AppSettings } from './types';

type Tab = 'instances' | 'new' | 'java' | 'logs' | 'servers' | 'settings';

export default function App() {
  const [tab, setTab] = useState<Tab>('instances');
  const [selId, setSelId] = useState<string | null>(null);
  const [settings, setSettings] = useState<AppSettings>({ theme: 'dark', java_home_override: null, default_memory_min_mb: 2048, default_memory_max_mb: 6144, open_log_on_launch: true });
  const onLaunch = (id: string) => {
    setSelId(id);
    if (settings.open_log_on_launch) setTab('logs');
  };
  return (
    <div className="app">
      <nav className="nav">
        <div className="brand">⛏ CraftCraft</div>
        <button className={tab==='instances'?'active':''} onClick={()=>setTab('instances')}>Instances</button>
        <button className={tab==='new'?'active':''} onClick={()=>setTab('new')}>New</button>
        <button className={tab==='java'?'active':''} onClick={()=>setTab('java')}>Java</button>
        <button className={tab==='logs'?'active':''} onClick={()=>{setTab('logs'); if(selId)setSelId(selId);}}>Logs</button>
        <button className={tab==='servers'?'active':''} onClick={()=>setTab('servers')}>Servers</button>
        <button className={tab==='settings'?'active':''} onClick={()=>setTab('settings')}>Settings</button>
      </nav>
      <main className="content">
        {tab==='instances' && <InstancesPage onLaunch={onLaunch} onSelect={setSelId} />}
        {tab==='new' && <WizardPage onDone={() => setTab('instances')} />}
        {tab==='java' && <JavaPage />}
        {tab==='logs' && <LogPage selectedId={selId} onSel={setSelId} />}
        {tab==='servers' && <ServersPage />}
        {tab==='settings' && <SettingsPage settings={settings} setSettings={setSettings} />}
      </main>
    </div>
  );
}

import { useState, useEffect, useRef } from 'react';
import InstancesPage from './pages/InstancesPage';
import WizardPage from './pages/WizardPage';
import JavaPage from './pages/JavaPage';
import LogPage from './pages/LogPage';
import ServersPage from './pages/ServersPage';
import SettingsPage from './pages/SettingsPage';
import type { AppSettings } from './types';

type Tab = 'instances' | 'new' | 'java' | 'logs' | 'servers' | 'settings';

const NAV_ITEMS: { id: Tab; label: string }[] = [
  { id: 'instances', label: 'Instances' },
  { id: 'new',       label: 'New Instance' },
  { id: 'java',      label: 'Java' },
  { id: 'logs',      label: 'Logs' },
  { id: 'servers',   label: 'Servers' },
  { id: 'settings',  label: 'Settings' },
];

export default function App() {
  const [tab, setTab] = useState<Tab>('instances');
  const [selId, setSelId] = useState<string | null>(null);
  const [settings, setSettings] = useState<AppSettings>({
    theme: 'dark', java_home_override: null,
    default_memory_min_mb: 2048, default_memory_max_mb: 6144,
    open_log_on_launch: true,
  });
  const [instances, setInstances] = useState<number>(0);

  const onLaunch = (id: string) => {
    setSelId(id);
    if (settings.open_log_on_launch) setTab('logs');
  };

  const goToNew = () => setTab('new');

  useEffect(() => {
    // Listen for instance count changes via custom event or poll
    const interval = setInterval(() => {
      fetch('http://localhost:32179/instances')
        .then(r => r.json() as any)
        .then(data => setInstances(Array.isArray(data) ? data.length : 0))
        .catch(() => {});
    }, 3000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="app">
      <nav className="nav">
        <div className="brand">
          <div className="brand-icon">⛏</div>
          CraftCraft
        </div>
        <div className="nav-items">
          {NAV_ITEMS.map((item) => (
            <button
              key={item.id}
              className={`nav-btn${tab === item.id ? ' active' : ''}`}
              onClick={() => setTab(item.id)}
            >
              {item.label}
              {item.id === 'instances' && instances > 0 && (
                <span className="badge accent" style={{ marginLeft: 'auto', fontSize: 10 }}>{instances}</span>
              )}
            </button>
          ))}
        </div>
        <div className="nav-footer">v0.1.0</div>
      </nav>

      <main className="content">
        {tab === 'instances' && (
          <InstancesPage
            onLaunch={onLaunch}
            onSelect={setSelId}
            onNew={goToNew}
          />
        )}
        {tab === 'new' && <WizardPage onDone={() => setTab('instances')} />}
        {tab === 'java' && <JavaPage />}
        {tab === 'logs' && (
          <LogPage
            selectedId={selId}
            onSel={setSelId}
            onNew={goToNew}
          />
        )}
        {tab === 'servers' && <ServersPage />}
        {tab === 'settings' && (
          <SettingsPage settings={settings} setSettings={setSettings} />
        )}
      </main>
    </div>
  );
}

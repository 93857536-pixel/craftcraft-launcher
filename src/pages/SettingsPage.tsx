import { useState, useEffect } from 'react';
import { getSettings, saveSettings } from '../api';
import { AVAILABLE_LANGS, useI18n } from '../i18n';
import type { AppSettings } from '../types';

interface P { settings: AppSettings; setSettings: (s: AppSettings) => void; }

export default function SettingsPage({ settings, setSettings }: P) {
  const { t, lang, setLang } = useI18n();
  const [memMin, setMemMin] = useState(settings.default_memory_min_mb);
  const [memMax, setMemMax] = useState(settings.default_memory_max_mb);
  const [openOnLaunch, setOpenOnLaunch] = useState(settings.open_log_on_launch);
  const [saved, setSaved] = useState(false);

  useEffect(() => {
    getSettings().then((s) => {
      setSettings(s);
      setMemMax(s.default_memory_max_mb);
      setMemMin(s.default_memory_min_mb);
      setOpenOnLaunch(s.open_log_on_launch);
    }).catch(() => {});
  }, []);

  const save = () => {
    const s: AppSettings = { ...settings, default_memory_min_mb: memMin, default_memory_max_mb: memMax, open_log_on_launch: openOnLaunch };
    setSettings(s);
    saveSettings(s).catch(console.error);
    setSaved(true);
    window.setTimeout(() => setSaved(false), 2000);
  };

  return (
    <>
      <div className="page-title">{t('settings.title')}</div>

      {/* Language — applies instantly, no save needed */}
      <div className="card" style={{ maxWidth: 420 }}>
        <div className="card-header"><h2>{t('settings.language')}</h2></div>
        <label style={{ display: 'block' }}>
          <select
            value={lang}
            onChange={(e) => setLang(e.target.value)}
            style={{ width: '100%' }}
          >
            {AVAILABLE_LANGS.map((l) => (
              <option key={l.code} value={l.code}>
                {l.native} — {l.name}
              </option>
            ))}
          </select>
        </label>
        <div className="text-xs mt-8" style={{ color: 'var(--text-dim)' }}>
          {t('settings.languageDesc')}
        </div>
      </div>

      <div className="card" style={{ maxWidth: 420 }}>
        <div className="form-grid" style={{ gridTemplateColumns: '1fr 1fr' }}>
          <div>
            <label>{t('settings.defaultMaxMemory')}
              <input type="number" value={memMax} min={512} max={32768} step={512} onChange={(e) => setMemMax(Number(e.target.value))} />
            </label>
          </div>
          <div>
            <label>{t('settings.defaultMinMemory')}
              <input type="number" value={memMin} min={256} max={memMax} step={256} onChange={(e) => setMemMin(Number(e.target.value))} />
            </label>
          </div>
          <div className="full">
            <label style={{ flexDirection: 'row', alignItems: 'center', gap: 10, textTransform: 'none', fontSize: 12.5, letterSpacing: 0 }}>
              <input type="checkbox" checked={openOnLaunch} onChange={(e) => setOpenOnLaunch(e.target.checked)} />
              <span style={{ color: 'var(--text-secondary)' }}>{t('settings.autoSwitchLogs')}</span>
            </label>
          </div>
        </div>
        <div className="divider" />
        <div className="flex items-center gap-12">
          <button className="primary" onClick={save}>{t('settings.save')}</button>
          {saved && <span className="text-xs" style={{ color: 'var(--ok, #3fb950)' }}>{t('settings.saved')}</span>}
        </div>
      </div>
    </>
  );
}

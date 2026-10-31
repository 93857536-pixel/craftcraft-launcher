import { useState, useEffect, useRef } from 'react';
import { listJava, installJava } from '../api';
import { useT } from '../i18n';
import type { JavaVersion } from '../types';

export default function JavaPage() {
  const t = useT();
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
      <div className="page-title">{t('java.title')}</div>
      <div className="card">
        <div className="card-header">
          <h2>{t('java.downloadTemurin')}</h2>
        </div>
        <div className="flex gap-8">
          {[8, 17, 21].map((v) => (
            <button key={v} className="secondary" disabled={!!downloading}
              onClick={() => doInstall(String(v))}>
              {downloading === String(v) ? t('common.downloading') : `Temurin ${v}`}
            </button>
          ))}
        </div>
        <div className="text-xs mt-8" style={{ color: 'var(--text-dim)', maxWidth: 620, lineHeight: 1.6 }}>
          {t('java.legacyNote')}
        </div>
      </div>

      <div className="card" style={{ padding: 0 }}>
        <div className="card-header" style={{ padding: '12px 16px', borderBottom: '1px solid var(--border)' }}>
          <h2>{t('java.installed')}</h2>
          <span className="badge accent">{javas.length}</span>
        </div>
        <table>
          <thead>
            <tr>
              <th>{t('java.col.version')}</th>
              <th>{t('java.col.path')}</th>
              <th>{t('java.col.vendor')}</th>
              <th>{t('java.col.arch')}</th>
              <th>{t('java.col.type')}</th>
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
                    ? <span className="badge green">{t('java.managed')}</span>
                    : <span className="badge blue">{t('java.system')}</span>}
                </td>
              </tr>
            ))}
            {!javas.length && (
              <tr>
                <td colSpan={5} style={{ textAlign: 'center', padding: '20px 0', color: 'var(--text-tertiary)' }}>
                  {t('java.empty')}
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
    </>
  );
}

import { useState, useEffect, useMemo } from 'react';
import { createInstance, listMinecraftVersions, listJava, installJava, matchGameLanguage, gameLanguages } from '../api';
import { useT } from '../i18n';
import type { MojangVersionInfo, JavaVersion, GameLanguage } from '../types';

interface P { onDone: () => void; }

/** 常用语言置顶，其余（含克林贡语、昆雅精灵语、道本语等小众语言）归入「更多」组。 */
const COMMON_LANGS = [
  'zh_cn', 'zh_tw', 'zh_hk', 'en_us', 'en_gb', 'ja_jp', 'ko_kr',
  'fr_fr', 'de_de', 'es_es', 'es_mx', 'pt_br', 'pt_pt', 'ru_ru',
  'it_it', 'nl_nl', 'pl_pl', 'tr_tr', 'uk_ua', 'vi_vn', 'th_th',
  'id_id', 'ar_sa', 'hi_in',
];

export default function WizardPage({ onDone }: P) {
  const t = useT();
  const [name, setName] = useState('');
  const [ver, setVer] = useState('');
  const [loader, setLoader] = useState('vanilla' as const);
  const [account, setAccount] = useState('Player');
  const [memoryMax, setMemoryMax] = useState(6144);
  const [versions, setVersions] = useState<MojangVersionInfo[]>([]);
  const [javas, setJavas] = useState<JavaVersion[]>([]);
  const [downloading, setDownloading] = useState<string | null>(null);
  const [langs, setLangs] = useState<GameLanguage[]>([]);
  const [lang, setLang] = useState<string>('');

  useEffect(() => {
    listMinecraftVersions().then((v) => setVersions(v.filter((x) => x.type === 'release'))).catch(() => {});
    listJava().then(setJavas).catch(() => {});
    gameLanguages().then(setLangs).catch(() => {});
    // 按系统界面语言预选游戏语言（如 zh-CN → zh_cn）
    matchGameLanguage(navigator.language)
      .then((l) => { if (l) setLang(l.code); })
      .catch(() => {});
  }, []);

  const picked = useMemo(() => versions.find((v) => v.id === ver), [versions, ver]);
  const hasJava = javas.length > 0;

  const start = () => {
    if (!name.trim()) return;
    createInstance({
      name: name.trim(),
      version: picked?.id ?? ver,
      loader,
      account_name: account,
      language: lang || undefined,
    })
      .then(() => onDone())
      .catch((e: any) => alert(t('wizard.createFailed', { msg: String(e) })));
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
      <div className="page-title">{t('wizard.title')}</div>
      <div className="card" style={{ maxWidth: 560 }}>
        <form onSubmit={(e) => { e.preventDefault(); start(); }}>
          <div className="form-grid">
            <div className="full">
              <label>{t('wizard.name')}
                <input value={name} onChange={(e) => setName(e.target.value)} placeholder={t('wizard.namePlaceholder')} required />
              </label>
            </div>
            <div>
              <label>{t('wizard.version')}
                <input value={ver} onChange={(e) => setVer(e.target.value)} placeholder={t('wizard.versionPlaceholder')} />
              </label>
              {picked && (
                <div className="text-xs mt-8" style={{ color: 'var(--text-dim)' }}>
                  <span className="badge green">{picked.type}</span> · {picked.id}
                </div>
              )}
              {!picked && ver && (
                <div className="text-xs mt-8" style={{ color: 'var(--warn)' }}>⚠ {t('wizard.notInManifest')}</div>
              )}
            </div>
            <div>
              <label>{t('wizard.loader')}
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
              <label>{t('wizard.accountName')}
                <input value={account} onChange={(e) => setAccount(e.target.value)} />
              </label>
            </div>
            {langs.length > 0 && (
              <div>
                <label>{t('instances.language')}
                  <select value={lang} onChange={(e) => setLang(e.target.value)}>
                    <option value="">{t('instances.languageAuto')}</option>
                    <optgroup label={t('instances.languageCommon')}>
                      {langs.filter((l) => COMMON_LANGS.includes(l.code)).map((l) => (
                        <option key={l.code} value={l.code}>
                          {l.region ? `${l.name} (${l.region})` : l.name}
                        </option>
                      ))}
                    </optgroup>
                    <optgroup label={t('instances.languageMore')}>
                      {langs.filter((l) => !COMMON_LANGS.includes(l.code)).map((l) => (
                        <option key={l.code} value={l.code}>
                          {l.region ? `${l.name} (${l.region})` : l.name}
                        </option>
                      ))}
                    </optgroup>
                  </select>
                </label>
              </div>
            )}
            <div>
              <label>
                {t('wizard.maxMemory')}
                <input type="range" min={512} max={16384} step={512} value={memoryMax}
                  onChange={(e) => setMemoryMax(Number(e.target.value))}
                  style={{ width: '100%', accentColor: 'var(--accent)', marginTop: 4 }} />
              </label>
              <div className="text-xs mt-8" style={{ textAlign: 'right', color: 'var(--text-dim)', fontFamily: 'ui-monospace, monospace' }}>
                {memoryMax} MB
              </div>
            </div>
            <div>
              <label>{t('wizard.javaRuntime')}</label>
              <div className="text-xs mt-8 mb-8" style={{ color: 'var(--text-dim)' }}>
                {hasJava ? t('wizard.jdkAvailable', { n: javas.length }) : t('wizard.noJdk')}
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
            <button type="button" className="secondary" onClick={onDone}>{t('common.cancel')}</button>
            <button type="submit" className="primary" disabled={!name.trim()}>{t('wizard.create')}</button>
          </div>
        </form>
      </div>
    </>
  );
}

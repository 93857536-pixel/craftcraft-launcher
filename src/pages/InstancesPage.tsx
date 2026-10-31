import { useState, useEffect } from 'react';
import {
  getInstances,
  deleteInstance,
  launchInstance,
  setTags,
  getAllTags,
  gameLanguages,
  updateInstance,
  listMinecraftVersions,
} from '../api';
import { useT } from '../i18n';
import type { Instance, GameLanguage, MojangVersionInfo } from '../types';

/** 常用语言置顶，其余（含克林贡语、昆雅精灵语、道本语等小众语言）归入「更多」组。 */
const COMMON_LANGS = [
  'zh_cn', 'zh_tw', 'zh_hk', 'en_us', 'en_gb', 'ja_jp', 'ko_kr',
  'fr_fr', 'de_de', 'es_es', 'es_mx', 'pt_br', 'pt_pt', 'ru_ru',
  'it_it', 'nl_nl', 'pl_pl', 'tr_tr', 'uk_ua', 'vi_vn', 'th_th',
  'id_id', 'ar_sa', 'hi_in',
];

const langLabel = (l: GameLanguage) =>
  l.region ? `${l.name} (${l.region})` : l.name;

interface P {
  onLaunch: (id: string) => void;
  onSelect: (id: string | null) => void;
  onNew: () => void;
}

export default function InstancesPage({ onLaunch, onSelect, onNew }: P) {
  const t = useT();
  const [items, setItems] = useState<Instance[]>([]);
  const [loading, setLoading] = useState(false);
  const [selId, setSelId] = useState<string | null>(null);
  const [tags, setTagsState] = useState<string[]>([]);
  const [filterTag, setFilterTag] = useState<string>('');
  const [editTagId, setEditTagId] = useState<string | null>(null);
  const [tagInput, setTagInput] = useState('');
  const [langs, setLangs] = useState<GameLanguage[]>([]);
  const [versions, setVersions] = useState<MojangVersionInfo[]>([]);

  useEffect(() => {
    Promise.all([getInstances(), getAllTags()]).then(([insts, allTags]) => {
      setItems(insts);
      setTagsState(allTags);
    });
    // 142 种游戏内语言一次性取回，纯本地常量表，无网络开销
    gameLanguages().then(setLangs).catch(() => setLangs([]));
    // 版本清单仅供「换版本」下拉使用，失败不影响卡片其它功能
    listMinecraftVersions()
      .then((v) => setVersions(v.filter((x) => x.type === 'release')))
      .catch(() => setVersions([]));
  }, []);

  const setVersion = async (id: string, code: string) => {
    if (!code) return;
    const updated = await updateInstance({ id, changes: { version: code } });
    setItems((x) => x.map((i) => (i.id === id ? updated : i)));
  };

  const setLanguage = async (id: string, code: string) => {
    const updated = await updateInstance({ id, changes: { language: code } });
    setItems((x) => x.map((i) => (i.id === id ? updated : i)));
  };

  const del = async (id: string) => {
    if (!confirm(t('instances.deleteConfirm'))) return;
    await deleteInstance(id);
    setItems((x) => x.filter((i) => i.id !== id));
    if (selId === id) setSelId(null);
  };

  const launch = async (id: string) => {
    setLoading(true);
    try {
      const r = await launchInstance(id);
      if (r.success) {
        setItems((x) =>
          x.map((i) => (i.id === id ? { ...i, last_played_at: Math.floor(Date.now() / 1000) } : i)),
        );
        onLaunch(id);
      } else alert(t('instances.launchFailed', { msg: r.message }));
    } finally { setLoading(false); }
  };

  const addTag = async (id: string, tag: string) => {
    if (!tag.trim()) return;
    const inst = items.find((i) => i.id === id);
    if (!inst) return;
    const newTags = [...inst.tags, tag.trim()];
    await setTags(id, newTags);
    setItems((x) => x.map((i) => (i.id === id ? { ...i, tags: newTags } : i)));
    setTagInput('');
  };

  const removeTag = async (id: string, tag: string) => {
    const inst = items.find((i) => i.id === id);
    if (!inst) return;
    const newTags = inst.tags.filter((t) => t !== tag);
    await setTags(id, newTags);
    setItems((x) => x.map((i) => (i.id === id ? { ...i, tags: newTags } : i)));
  };

  const filtered = filterTag
    ? items.filter((i) => i.tags.includes(filterTag))
    : items;

  if (!items.length) {
    return (
      <div className="empty">
        <div className="empty-icon">⛏</div>
        <h3>{t('instances.empty.title')}</h3>
        <p>{t('instances.empty.desc')}</p>
        <button className="primary" onClick={onNew} style={{ marginTop: 4, padding: '8px 20px', fontSize: 13 }}>
          + {t('instances.newCard')}
        </button>
      </div>
    );
  }

  return (
    <>
      <div className="page-title">
        {t('instances.title')}
        <span className="text-muted" style={{ fontWeight: 400, marginLeft: 6, fontSize: 12.5 }}>
          {t('instances.count', { n: items.length })}
          {filterTag && <span style={{ color: 'var(--accent-bright)', marginLeft: 6 }}>{t('instances.filteredBy', { tag: filterTag })}</span>}
        </span>
      </div>

      {/* Tag filter */}
      {tags.length > 0 && (
        <div style={{ marginBottom: 14, display: 'flex', alignItems: 'center', gap: 8, flexWrap: 'wrap' }}>
          <span className="text-xs text-muted">{t('common.filter')}:</span>
          <button
            className={filterTag ? 'secondary' : 'primary'}
            style={{ padding: '2px 10px', fontSize: 11 }}
            onClick={() => setFilterTag('')}
          >
            {t('common.all')}
          </button>
          {tags.map((tg) => (
            <button
              key={tg}
              className={filterTag === tg ? 'accent' : 'secondary'}
              style={{ padding: '2px 10px', fontSize: 11 }}
              onClick={() => setFilterTag(filterTag === tg ? '' : tg)}
            >
              {tg}
            </button>
          ))}
        </div>
      )}

      <div className="grid">
        {filtered.map((i) => (
          <div
            key={i.id}
            className={`inst-card${selId === i.id ? ' selected' : ''}`}
            onClick={() => onSelect(i.id)}
          >
            <div className="inst-header">
              <div className="inst-icon">⛏</div>
              <div style={{ flex: 1, minWidth: 0 }}>
                <div className="inst-name">{i.name}</div>
                <div className="inst-version">
                  {i.version}{i.loader !== 'vanilla' ? ` · ${i.loader}` : ''}
                </div>
              </div>
              <span className="badge green">{i.memory_max_mb}MB</span>
            </div>

            {/* Tags */}
            <div className="inst-meta" style={{ marginBottom: 8 }}>
              {i.tags.map((tg) => (
                <span key={tg} className="badge accent" style={{ cursor: 'pointer' }}
                  onClick={(e) => { e.stopPropagation(); setFilterTag(tg); }}>
                  {tg} ✕
                </span>
              ))}
            </div>

            {/* Tag input */}
            <div style={{ display: 'flex', gap: 4, marginBottom: 8 }}>
              <input
                className="text-xs"
                placeholder={t('instances.addTag')}
                style={{ flex: 1, padding: '2px 6px', fontSize: 11, background: 'var(--bg-input)', border: '1px solid var(--border)', borderRadius: 'var(--r-sm)', color: 'var(--text-sub)' }}
                value={tagInput}
                onChange={(e) => setTagInput(e.target.value)}
                onKeyDown={(e) => { if (e.key === 'Enter') addTag(i.id, tagInput); }}
              />
              <button
                className="secondary"
                style={{ padding: '2px 6px', fontSize: 11 }}
                onClick={() => addTag(i.id, tagInput)}
              >+</button>
            </div>

            {/* Game version — 换版本，下拉可选，不再手输 */}
            {versions.length > 0 && (
              <div style={{ display: 'flex', alignItems: 'center', gap: 6, marginBottom: 8 }}>
                <span className="text-xs text-muted" style={{ flexShrink: 0 }}>📦</span>
                <select
                  className="text-xs"
                  style={{ flex: 1 }}
                  value={i.version}
                  onClick={(e) => e.stopPropagation()}
                  onChange={(e) => { e.stopPropagation(); setVersion(i.id, e.target.value); }}
                  title={t('instances.versionDesc')}
                >
                  {!versions.some((v) => v.id === i.version) && (
                    <option value={i.version}>{i.version}</option>
                  )}
                  {versions.map((v) => (
                    <option key={v.id} value={v.id}>
                      {v.id}{v.id === versions[0]?.id ? ' ★' : ''}
                    </option>
                  ))}
                </select>
              </div>
            )}
            {langs.length > 0 && (
              <div style={{ display: 'flex', alignItems: 'center', gap: 6, marginBottom: 8 }}>
                <span className="text-xs text-muted" style={{ flexShrink: 0 }}>🌐</span>
                <select
                  className="text-xs"
                  style={{ flex: 1, padding: '2px 6px', fontSize: 11, background: 'var(--bg-input)', border: '1px solid var(--border)', borderRadius: 'var(--r-sm)', color: 'var(--text-sub)' }}
                  value={i.language ?? ''}
                  onClick={(e) => e.stopPropagation()}
                  onChange={(e) => { e.stopPropagation(); setLanguage(i.id, e.target.value); }}
                  title={t('instances.languageDesc')}
                >
                  <option value="">{t('instances.languageAuto')}</option>
                  <optgroup label={t('instances.languageCommon')}>
                    {langs.filter((l) => COMMON_LANGS.includes(l.code)).map((l) => (
                      <option key={l.code} value={l.code}>{langLabel(l)}</option>
                    ))}
                  </optgroup>
                  <optgroup label={t('instances.languageMore')}>
                    {langs.filter((l) => !COMMON_LANGS.includes(l.code)).map((l) => (
                      <option key={l.code} value={l.code}>{langLabel(l)}</option>
                    ))}
                  </optgroup>
                </select>
              </div>
            )}

            <div style={{ display: 'flex', gap: 6, paddingTop: 8, borderTop: '1px solid var(--border)' }}>
              <button className="primary" disabled={loading} onClick={(e) => { e.stopPropagation(); launch(i.id); }} style={{ flex: 1, justifyContent: 'center' }}>
                ▶ {t('common.launch')}
              </button>
              <button className="secondary" onClick={(e) => { e.stopPropagation(); onSelect(i.id); }}>{t('common.edit')}</button>
              <button className="danger" onClick={(e) => { e.stopPropagation(); del(i.id); }}>{t('common.del')}</button>
            </div>
          </div>
        ))}
        {/* Add new instance card */}
        <div
          className="inst-card"
          style={{ borderStyle: 'dashed', cursor: 'pointer', display: 'flex', alignItems: 'center', justifyContent: 'center', minHeight: 140, color: 'var(--text-dim)' }}
          onClick={onNew}
        >
          <div style={{ textAlign: 'center' }}>
            <div style={{ fontSize: 20, marginBottom: 4, opacity: 0.5 }}>+</div>
            <div style={{ fontSize: 12 }}>{t('instances.newCard')}</div>
          </div>
        </div>
      </div>
    </>
  );
}

import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from 'react';
import type { Dict } from './locales/en';

/**
 * Lightweight i18n for CraftCraft.
 *
 * Design notes:
 *  - Locale files are auto-discovered with `import.meta.glob`, so a missing or
 *    half-finished translation never breaks the build (it just doesn't show up
 *    in the language picker).
 *  - Keys missing from a locale fall back to English, then to the raw key.
 *  - `{name}` placeholders are interpolated; `key_one` / `key_other` variants
 *    are picked by a simple plural rule.
 *  - The chosen language is persisted in localStorage and re-detected from
 *    `navigator.languages` on first run.
 */

export interface LangMeta {
  /** BCP-47-ish tag, also the locale file name. */
  code: string;
  /** English name, used for sorting/搜索. */
  name: string;
  /** Endonym shown in the picker. */
  native: string;
  /** Right-to-left script. */
  rtl?: boolean;
}

/** Display metadata for every locale this app knows about. */
export const LANG_META: LangMeta[] = [
  { code: 'en', name: 'English', native: 'English' },
  { code: 'zh-CN', name: 'Chinese (Simplified)', native: '简体中文' },
  { code: 'zh-TW', name: 'Chinese (Traditional)', native: '繁體中文' },
  { code: 'ja', name: 'Japanese', native: '日本語' },
  { code: 'ko', name: 'Korean', native: '한국어' },

  { code: 'de', name: 'German', native: 'Deutsch' },
  { code: 'fr', name: 'French', native: 'Français' },
  { code: 'es', name: 'Spanish', native: 'Español' },
  { code: 'pt-BR', name: 'Portuguese (Brazil)', native: 'Português (Brasil)' },
  { code: 'it', name: 'Italian', native: 'Italiano' },
  { code: 'nl', name: 'Dutch', native: 'Nederlands' },
  { code: 'sv', name: 'Swedish', native: 'Svenska' },
  { code: 'da', name: 'Danish', native: 'Dansk' },
  { code: 'nb', name: 'Norwegian', native: 'Norsk bokmål' },
  { code: 'fi', name: 'Finnish', native: 'Suomi' },
  { code: 'is', name: 'Icelandic', native: 'Íslenska' },
  { code: 'ca', name: 'Catalan', native: 'Català' },
  { code: 'eo', name: 'Esperanto', native: 'Esperanto' },

  { code: 'ru', name: 'Russian', native: 'Русский' },
  { code: 'uk', name: 'Ukrainian', native: 'Українська' },
  { code: 'be', name: 'Belarusian', native: 'Беларуская' },
  { code: 'pl', name: 'Polish', native: 'Polski' },
  { code: 'cs', name: 'Czech', native: 'Čeština' },
  { code: 'sk', name: 'Slovak', native: 'Slovenčina' },
  { code: 'sl', name: 'Slovenian', native: 'Slovenščina' },
  { code: 'hr', name: 'Croatian', native: 'Hrvatski' },
  { code: 'sr', name: 'Serbian', native: 'Српски' },
  { code: 'bs', name: 'Bosnian', native: 'Bosanski' },
  { code: 'bg', name: 'Bulgarian', native: 'Български' },
  { code: 'mk', name: 'Macedonian', native: 'Македонски' },
  { code: 'sq', name: 'Albanian', native: 'Shqip' },
  { code: 'ro', name: 'Romanian', native: 'Română' },
  { code: 'hu', name: 'Hungarian', native: 'Magyar' },
  { code: 'el', name: 'Greek', native: 'Ελληνικά' },
  { code: 'lt', name: 'Lithuanian', native: 'Lietuvių' },
  { code: 'lv', name: 'Latvian', native: 'Latviešu' },
  { code: 'et', name: 'Estonian', native: 'Eesti' },

  { code: 'tr', name: 'Turkish', native: 'Türkçe' },
  { code: 'az', name: 'Azerbaijani', native: 'Azərbaycanca' },
  { code: 'kk', name: 'Kazakh', native: 'Қазақша' },
  { code: 'hy', name: 'Armenian', native: 'Հայերեն' },
  { code: 'ka', name: 'Georgian', native: 'ქართული' },

  { code: 'ar', name: 'Arabic', native: 'العربية', rtl: true },
  { code: 'he', name: 'Hebrew', native: 'עברית', rtl: true },
  { code: 'fa', name: 'Persian', native: 'فارسی', rtl: true },
  { code: 'ur', name: 'Urdu', native: 'اردو', rtl: true },

  { code: 'hi', name: 'Hindi', native: 'हिन्दी' },
  { code: 'bn', name: 'Bengali', native: 'বাংলা' },
  { code: 'ta', name: 'Tamil', native: 'தமிழ்' },
  { code: 'th', name: 'Thai', native: 'ไทย' },
  { code: 'vi', name: 'Vietnamese', native: 'Tiếng Việt' },
  { code: 'id', name: 'Indonesian', native: 'Bahasa Indonesia' },
  { code: 'ms', name: 'Malay', native: 'Bahasa Melayu' },
  { code: 'fil', name: 'Filipino', native: 'Filipino' },

  { code: 'af', name: 'Afrikaans', native: 'Afrikaans' },
  { code: 'sw', name: 'Swahili', native: 'Kiswahili' },
  { code: 'cy', name: 'Welsh', native: 'Cymraeg' },
  { code: 'ga', name: 'Irish', native: 'Gaeilge' },
];

const modules = import.meta.glob('./locales/*.ts', { eager: true }) as Record<
  string,
  { default: Dict }
>;

const DICTS: Record<string, Dict> = {};
for (const [path, mod] of Object.entries(modules)) {
  const code = path.replace('./locales/', '').replace(/\.ts$/, '');
  if (mod?.default) DICTS[code] = mod.default;
}

/** Locales that actually have a file on disk, ordered like LANG_META. */
export const AVAILABLE_LANGS: LangMeta[] = LANG_META.filter((l) => DICTS[l.code]);

const STORAGE_KEY = 'craftcraft.lang';
const FALLBACK = 'en';

function safeStorageGet(key: string): string | null {
  try {
    return window.localStorage.getItem(key);
  } catch {
    return null;
  }
}

function safeStorageSet(key: string, value: string) {
  try {
    window.localStorage.setItem(key, value);
  } catch {
    /* private mode — ignore */
  }
}

/** Persisted choice → exact match → base-language match → English. */
export function detectLanguage(): string {
  const saved = safeStorageGet(STORAGE_KEY);
  if (saved && DICTS[saved]) return saved;

  const candidates =
    navigator.languages && navigator.languages.length
      ? navigator.languages
      : [navigator.language];

  for (const raw of candidates) {
    if (!raw) continue;
    if (DICTS[raw]) return raw;
    const base = raw.split('-')[0].toLowerCase();
    if (DICTS[base]) return base;
    const prefixed = Object.keys(DICTS).find(
      (code) => code.toLowerCase().split('-')[0] === base,
    );
    if (prefixed) return prefixed;
  }
  return DICTS[FALLBACK] ? FALLBACK : Object.keys(DICTS)[0] ?? FALLBACK;
}

function interpolate(template: string, vars?: Record<string, string | number>) {
  if (!vars) return template;
  return template.replace(/\{(\w+)\}/g, (match, key) =>
    Object.prototype.hasOwnProperty.call(vars, key) ? String(vars[key]) : match,
  );
}

export type Translate = (key: string, vars?: Record<string, string | number>) => string;

interface I18nContextValue {
  lang: string;
  setLang: (code: string) => void;
  t: Translate;
  rtl: boolean;
}

const I18nContext = createContext<I18nContextValue>({
  lang: FALLBACK,
  setLang: () => {},
  t: (key) => key,
  rtl: false,
});

export function I18nProvider({ children }: { children: ReactNode }) {
  const [lang, setLangState] = useState<string>(() => detectLanguage());

  const setLang = useCallback((code: string) => {
    if (!DICTS[code]) return;
    setLangState(code);
    safeStorageSet(STORAGE_KEY, code);
  }, []);

  const rtl = useMemo(
    () => LANG_META.find((l) => l.code === lang)?.rtl ?? false,
    [lang],
  );

  useEffect(() => {
    document.documentElement.lang = lang;
    document.documentElement.dir = rtl ? 'rtl' : 'ltr';
  }, [lang, rtl]);

  const t = useCallback<Translate>(
    (key, vars) => {
      const dict = DICTS[lang];
      // Plural variants: `key_one` / `key_other` (English-style, good enough for
      // a UI with a single countable string).
      if (vars && typeof vars.n === 'number') {
        const variant = vars.n === 1 ? `${key}_one` : `${key}_other`;
        const hit = dict?.[variant] ?? DICTS[FALLBACK]?.[variant];
        if (hit) return interpolate(hit, vars);
      }
      const raw = dict?.[key] ?? DICTS[FALLBACK]?.[key] ?? key;
      return interpolate(raw, vars);
    },
    [lang],
  );

  const value = useMemo<I18nContextValue>(
    () => ({ lang, setLang, t, rtl }),
    [lang, setLang, t, rtl],
  );

  return <I18nContext.Provider value={value}>{children}</I18nContext.Provider>;
}

export function useI18n() {
  return useContext(I18nContext);
}

/** Shorthand for components that only need the translator. */
export function useT(): Translate {
  return useContext(I18nContext).t;
}

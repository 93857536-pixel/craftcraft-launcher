//! Minecraft 内置语言表（143 种，含克林贡语/昆雅精灵语/道本语/文言等小众语言）。
//!
//! 数据来源：Mojang 官方客户端 asset index 中的 `minecraft/lang/*.json`，
//! 逐条读取其 `language.name` / `language.region` 字段生成
//! （基准：1.21 的 asset index，id=17）。
//!
//! 之所以内置而非联网拉取：官方并未提供公开的 language.json 端点
//! （`launchermeta.mojang.com/mc/game/language.json` 返回 404），
//! 内置表可离线使用，也不拖慢启动。

use serde::Serialize;

/// 一条 Minecraft 语言条目。字段用 `&'static str`，因为整张表是编译期常量
/// （142 条全部内置，序列化给前端时零拷贝）。
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct GameLanguage {
    /// Mojang locale 代码，小写下划线形式，如 `zh_cn` / `tlh_aa`
    pub code: &'static str,
    /// 该语言自称的名称，如 `tlhIngan Hol`
    pub name: &'static str,
    /// 区域/国家，可能为空
    pub region: &'static str,
}

impl GameLanguage {
    /// 传给游戏的 `--language` 值
    pub fn launch_code(&self) -> &'static str {
        self.code
    }

    /// 展示用标签：`名称 (区域)`，区域为空时只显示名称
    pub fn label(&self) -> String {
        if self.region.is_empty() {
            self.name.to_string()
        } else {
            format!("{} ({})", self.name, self.region)
        }
    }
}

/// 全部支持的 Minecraft 语言（143 种，按 locale 代码排序）。
/// 142 条来自 asset index 的 `minecraft/lang/*.json`，另补 `en_us`
/// （默认语言，客户端 jar 内置，不在 asset index 里）。
pub const GAME_LANGUAGES: &[GameLanguage] = &[
    GameLanguage { code: "af_za", name: "Afrikaans", region: "Suid-Afrika" },
    GameLanguage { code: "ar_sa", name: "العربية", region: "العالم العربي" },
    GameLanguage { code: "ast_es", name: "Asturianu", region: "Asturies" },
    GameLanguage { code: "az_az", name: "Azərbaycanca", region: "Azərbaycan" },
    GameLanguage { code: "ba_ru", name: "Башҡортса", region: "Башҡортостан, Рәсәй" },
    GameLanguage { code: "bar", name: "Boarisch", region: "Bayern" },
    GameLanguage { code: "be_by", name: "Беларуская", region: "Беларусь" },
    GameLanguage { code: "be_latn", name: "Biełaruskaja", region: "Biełaruś" },
    GameLanguage { code: "bg_bg", name: "Български", region: "България" },
    GameLanguage { code: "br_fr", name: "Brezhoneg", region: "Breizh" },
    GameLanguage { code: "brb", name: "Braobans", region: "Braobant" },
    GameLanguage { code: "bs_ba", name: "Bosanski", region: "Bosna i Hercegovina" },
    GameLanguage { code: "ca_es", name: "Català", region: "Catalunya" },
    GameLanguage { code: "cs_cz", name: "Čeština", region: "Česko" },
    GameLanguage { code: "cv_cu", name: "Чӑвашла", region: "Чӑваш Ен, Раҫҫей" },
    GameLanguage { code: "cy_gb", name: "Cymraeg", region: "Cymru" },
    GameLanguage { code: "da_dk", name: "Dansk", region: "Danmark" },
    GameLanguage { code: "de_at", name: "Deitsch", region: "Österreich" },
    GameLanguage { code: "de_ch", name: "Schwiizerdütsch", region: "Schwiiz" },
    GameLanguage { code: "de_de", name: "Deutsch", region: "Deutschland" },
    GameLanguage { code: "el_gr", name: "Ελληνικά", region: "Ελλάδα" },
    GameLanguage { code: "en_au", name: "English", region: "Australia" },
    GameLanguage { code: "en_ca", name: "English", region: "Canada" },
    GameLanguage { code: "en_gb", name: "English", region: "United Kingdom" },
    GameLanguage { code: "en_nz", name: "English", region: "New Zealand" },
    GameLanguage { code: "en_pt", name: "Pirate Speak", region: "The Seven Seas" },
    GameLanguage { code: "en_ud", name: "ɥsᴉꞁᵷuƎ", region: "uʍoᗡ ǝpᴉsd∩" },
    GameLanguage { code: "en_us", name: "English", region: "United States" },
    GameLanguage { code: "enp", name: "Anglish", region: "Oned Riches" },
    GameLanguage { code: "enws", name: "Shakespearean English", region: "Kingdom of England" },
    GameLanguage { code: "eo_uy", name: "Esperanto", region: "Esperantujo" },
    GameLanguage { code: "es_ar", name: "Español", region: "Argentina" },
    GameLanguage { code: "es_cl", name: "Español", region: "Chile" },
    GameLanguage { code: "es_ec", name: "Español", region: "Ecuador" },
    GameLanguage { code: "es_es", name: "Español", region: "España" },
    GameLanguage { code: "es_mx", name: "Español", region: "México" },
    GameLanguage { code: "es_uy", name: "Español", region: "Uruguay" },
    GameLanguage { code: "es_ve", name: "Español", region: "Venezuela" },
    GameLanguage { code: "esan", name: "Andalûh", region: "Andaluçía" },
    GameLanguage { code: "et_ee", name: "Eesti keel", region: "Eesti" },
    GameLanguage { code: "eu_es", name: "Euskara", region: "Euskal Herria" },
    GameLanguage { code: "fa_ir", name: "فارسی", region: "ايران" },
    GameLanguage { code: "fi_fi", name: "Suomi", region: "Suomi" },
    GameLanguage { code: "fil_ph", name: "Filipino", region: "Pilipinas" },
    GameLanguage { code: "fo_fo", name: "Føroyskt", region: "Føroyar" },
    GameLanguage { code: "fr_ca", name: "Français", region: "Canada" },
    GameLanguage { code: "fr_ch", name: "Français", region: "Suisse" },
    GameLanguage { code: "fr_fr", name: "Français", region: "France" },
    GameLanguage { code: "fra_de", name: "Fränggisch", region: "Franggn" },
    GameLanguage { code: "fur_it", name: "Furlan", region: "Friûl" },
    GameLanguage { code: "fy_nl", name: "Frysk", region: "Fryslân" },
    GameLanguage { code: "ga_ie", name: "Gaeilge", region: "Éire" },
    GameLanguage { code: "gd_gb", name: "Gàidhlig", region: "Alba" },
    GameLanguage { code: "gl_es", name: "Galego", region: "Galicia / Galiza" },
    GameLanguage { code: "go_fr", name: "Galo", region: "Bertègn" },
    GameLanguage { code: "got_de", name: "𐌲𐌿𐍄𐍂𐌰𐌶𐌳𐌰", region: "𐌲𐌿𐍄𐌸𐌹𐌿𐌳𐌰" },
    GameLanguage { code: "hal_ua", name: "Галицка", region: "Галичина, Вкраїна" },
    GameLanguage { code: "haw_us", name: "ʻŌlelo Hawaiʻi", region: "Hawaiʻi" },
    GameLanguage { code: "he_il", name: "עברית", region: "ישראל" },
    GameLanguage { code: "hi_in", name: "हिंदी", region: "भारत" },
    GameLanguage { code: "hn_no", name: "Høgnorsk", region: "Norig" },
    GameLanguage { code: "hr_hr", name: "Hrvatski", region: "Hrvatska" },
    GameLanguage { code: "hu_hu", name: "Magyar", region: "Magyarország" },
    GameLanguage { code: "hy_am", name: "Հայերեն", region: "Հայաստան" },
    GameLanguage { code: "id_id", name: "Bahasa Indonesia", region: "Indonesia" },
    GameLanguage { code: "ig_ng", name: "Igbo", region: "Naigeria" },
    GameLanguage { code: "io_en", name: "Ido", region: "Idia" },
    GameLanguage { code: "is_is", name: "Íslenska", region: "Ísland" },
    GameLanguage { code: "isv", name: "Medžuslovjansky", region: "Slovjanščina" },
    GameLanguage { code: "it_it", name: "Italiano", region: "Italia" },
    GameLanguage { code: "ja_jp", name: "日本語", region: "日本" },
    GameLanguage { code: "jbo_en", name: "la .lojban.", region: "la jbogu'e" },
    GameLanguage { code: "ka_ge", name: "ქართული", region: "საქართველო" },
    GameLanguage { code: "kk_kz", name: "Қазақша", region: "Қазақстан" },
    GameLanguage { code: "kn_in", name: "ಕನ್ನಡ", region: "ಭಾರತ" },
    GameLanguage { code: "ko_kr", name: "한국어", region: "대한민국" },
    GameLanguage { code: "ksh", name: "Kölsch/Ripoarisch", region: "Rhingland" },
    GameLanguage { code: "kw_gb", name: "Kernewek", region: "Kernow" },
    GameLanguage { code: "ky_kg", name: "Кыргызча", region: "Кыргызстан" },
    GameLanguage { code: "la_la", name: "Latina", region: "Latium" },
    GameLanguage { code: "lb_lu", name: "Lëtzebuergesch", region: "Lëtzebuerg" },
    GameLanguage { code: "li_li", name: "Limburgs", region: "Limburg" },
    GameLanguage { code: "lmo", name: "Lombard", region: "Lombardia" },
    GameLanguage { code: "lo_la", name: "ລາວ", region: "ປະເທດລາວ" },
    GameLanguage { code: "lol_us", name: "LOLCAT", region: "Kingdom of Cats" },
    GameLanguage { code: "lt_lt", name: "Lietuvių", region: "Lietuva" },
    GameLanguage { code: "lv_lv", name: "Latviešu", region: "Latvija" },
    GameLanguage { code: "lzh", name: "文言", region: "華夏" },
    GameLanguage { code: "mk_mk", name: "Македонски", region: "Северна Македонија" },
    GameLanguage { code: "mn_mn", name: "Монгол", region: "Монгол Улс" },
    GameLanguage { code: "ms_my", name: "Bahasa Melayu", region: "Malaysia" },
    GameLanguage { code: "mt_mt", name: "Malti", region: "Malta" },
    GameLanguage { code: "nah", name: "Mēxikatlahtōlli", region: "Mēxiko" },
    GameLanguage { code: "nds_de", name: "Plattdüütsch", region: "Düütschland" },
    GameLanguage { code: "nl_be", name: "Vlaams", region: "België" },
    GameLanguage { code: "nl_nl", name: "Nederlands", region: "Nederland" },
    GameLanguage { code: "nn_no", name: "Norsk nynorsk", region: "Noreg" },
    GameLanguage { code: "no_no", name: "Norsk bokmål", region: "Norge" },
    GameLanguage { code: "oc_fr", name: "Occitan", region: "Occitània" },
    GameLanguage { code: "ovd", name: "Övdalska", region: "Swerre" },
    GameLanguage { code: "pl_pl", name: "Polski", region: "Polska" },
    GameLanguage { code: "pls", name: "Ngiiwà", region: "Ndanìꞌngà" },
    GameLanguage { code: "pt_br", name: "Português", region: "Brasil" },
    GameLanguage { code: "pt_pt", name: "Português", region: "Portugal" },
    GameLanguage { code: "qcb_es", name: "Cántabru/Montañés", region: "Cantabria" },
    GameLanguage { code: "qid", name: "Bhs. Indonesia edjaän lama", region: "Indonesia tempo doeloe" },
    GameLanguage { code: "qya_aa", name: "Quenya", region: "Arda" },
    GameLanguage { code: "ro_ro", name: "Română", region: "România" },
    GameLanguage { code: "rpr", name: "Русскій дореформенный", region: "Россійская имперія" },
    GameLanguage { code: "ru_ru", name: "Русский", region: "Россия" },
    GameLanguage { code: "ry_ua", name: "Руснацькый", region: "Пудкарпатя, Украина" },
    GameLanguage { code: "sah_sah", name: "Сахалыы", region: "Cаха Сирэ" },
    GameLanguage { code: "se_no", name: "Davvisámegiella", region: "Sápmi" },
    GameLanguage { code: "sk_sk", name: "Slovenčina", region: "Slovensko" },
    GameLanguage { code: "sl_si", name: "Slovenščina", region: "Slovenija" },
    GameLanguage { code: "so_so", name: "Soomaali", region: "Soomaaliya" },
    GameLanguage { code: "sq_al", name: "Shqip", region: "Shqipëri" },
    GameLanguage { code: "sr_cs", name: "Srpski", region: "Srbija" },
    GameLanguage { code: "sr_sp", name: "Српски", region: "Србија" },
    GameLanguage { code: "sv_se", name: "Svenska", region: "Sverige" },
    GameLanguage { code: "sxu", name: "Säggs’sch", region: "Saggsn" },
    GameLanguage { code: "szl", name: "Ślōnski", region: "Gōrny Ślōnsk" },
    GameLanguage { code: "ta_in", name: "தமிழ்", region: "இந்தியா" },
    GameLanguage { code: "th_th", name: "ไทย", region: "ประเทศไทย" },
    GameLanguage { code: "tl_ph", name: "Tagalog", region: "Pilipinas" },
    GameLanguage { code: "tlh_aa", name: "tlhIngan Hol", region: "tlhIngan wo'" },
    GameLanguage { code: "tok", name: "toki pona", region: "kulupu pona" },
    GameLanguage { code: "tr_tr", name: "Türkçe", region: "Türkiye" },
    GameLanguage { code: "tt_ru", name: "Татарча", region: "Татарстан, Рәсәй" },
    GameLanguage { code: "tzo_mx", name: "Bats'i k'op", region: "Jobel" },
    GameLanguage { code: "uk_ua", name: "Українська", region: "Україна" },
    GameLanguage { code: "uz_uz", name: "Özbekça", region: "Özbekiston" },
    GameLanguage { code: "val_es", name: "Català (Valencià)", region: "País Valencià" },
    GameLanguage { code: "vec_it", name: "Vèneto", region: "Veneto" },
    GameLanguage { code: "vi_vn", name: "Tiếng Việt", region: "Việt Nam" },
    GameLanguage { code: "vp_vl", name: "Viossa", region: "Vilant" },
    GameLanguage { code: "vro", name: "Võro", region: "Eesti" },
    GameLanguage { code: "yi_de", name: "ייִדיש", region: "אשכנזיש יידן" },
    GameLanguage { code: "yo_ng", name: "Yorùbá", region: "Nàìjíríà" },
    GameLanguage { code: "zh_cn", name: "简体中文", region: "中国大陆" },
    GameLanguage { code: "zh_hk", name: "繁體中文", region: "香港特別行政區" },
    GameLanguage { code: "zh_tw", name: "繁體中文", region: "台灣" },
    GameLanguage { code: "zlm_arab", name: "بهاس ملايو", region: "مليسيا" },
];

/// 某语种的「母国」变体优先级表（`en` → `en_us` 之类）。
fn preferred_variants(lang: &str) -> &'static [&'static str] {
    match lang {
        "zh" => &["zh_cn", "zh_tw", "zh_hk"][..],
        "pt" => &["pt_pt", "pt_br"][..],
        "en" => &["en_us", "en_gb"][..],
        "de" => &["de_de", "de_at", "de_ch"][..],
        "es" => &["es_es", "es_mx", "es_ar"][..],
        "fr" => &["fr_fr", "fr_ca"][..],
        "sr" => &["sr_cs", "sr_sp"][..],
        "nb" | "no" => &["no_no", "nn_no"][..],
        _ => &[][..],
    }
}

/// 按 locale 代码查找语言（大小写、连字符、缺省区域码都能容忍）。
pub fn find_language(code: &str) -> Option<&'static GameLanguage> {
    let needle = code.trim().to_ascii_lowercase().replace('-', "_");
    if needle.is_empty() {
        return None;
    }
    // 1) 完全一致
    if let Some(hit) = GAME_LANGUAGES.iter().find(|l| l.code == needle) {
        return Some(hit);
    }
    let lang = needle.split('_').next().unwrap_or("");
    // 2) 语种母国变体（`en` → `en_us`）
    for cand in preferred_variants(lang) {
        if let Some(found) = GAME_LANGUAGES.iter().find(|l| l.code == *cand) {
            return Some(found);
        }
    }
    // 3) 同语种任意变体，最后才退到「无区域后缀」的孤立条目（如 `bar`、`lzh`、`tok`）
    GAME_LANGUAGES
        .iter()
        .find(|l| l.code.starts_with(&format!("{lang}_")))
        .or_else(|| GAME_LANGUAGES.iter().find(|l| l.code == lang))
}

/// 把系统 locale（如 `zh-CN`、`pt_BR`、`en`）映射到最接近的 Minecraft 语言。
pub fn match_system_locale(system_locale: &str) -> Option<&'static GameLanguage> {
    find_language(system_locale)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_has_all_mojang_languages() {
        assert_eq!(
            GAME_LANGUAGES.len(),
            143,
            "142 locales from asset index + en_us (bundled in client jar)"
        );
    }

    #[test]
    fn table_is_sorted_and_unique() {
        let mut codes: Vec<&str> = GAME_LANGUAGES.iter().map(|l| l.code).collect();
        let sorted = {
            let mut c = codes.clone();
            c.sort_unstable();
            c
        };
        assert_eq!(codes, sorted, "locales must be sorted");
        codes.dedup();
        assert_eq!(codes.len(), GAME_LANGUAGES.len(), "duplicate locale codes");
    }

    #[test]
    fn contains_obscure_languages() {
        // 极小众/构造语言，是「支持小众语言」的验收点
        for code in ["tlh_aa", "qya_aa", "lzh", "tok", "jbo_en", "lol_us", "en_ud", "got_de", "isv", "nah"] {
            assert!(find_language(code).is_some(), "missing obscure language {code}");
        }
        assert_eq!(find_language("tlh_aa").unwrap().name, "tlhIngan Hol");
        assert_eq!(find_language("lzh").unwrap().name, "文言");
        assert_eq!(find_language("tok").unwrap().name, "toki pona");
    }

    #[test]
    fn lookup_tolerates_case_and_hyphen() {
        assert_eq!(find_language("zh-CN").unwrap().code, "zh_cn");
        assert_eq!(find_language("ZH_TW").unwrap().code, "zh_tw");
        assert_eq!(find_language("en").unwrap().code, "en_us");
    }

    #[test]
    fn system_locale_mapping() {
        assert_eq!(match_system_locale("zh-Hans-CN").unwrap().code, "zh_cn");
        assert_eq!(match_system_locale("zh-CN").unwrap().code, "zh_cn");
        assert_eq!(match_system_locale("ja-JP").unwrap().code, "ja_jp");
        assert_eq!(match_system_locale("pt-PT").unwrap().code, "pt_pt");
        assert_eq!(match_system_locale("eo").unwrap().code, "eo_uy");
        assert_eq!(match_system_locale("xx-YY"), None);
    }

    #[test]
    fn label_includes_region_when_present() {
        assert_eq!(find_language("zh_cn").unwrap().label(), "简体中文 (中国大陆)");
        assert_eq!(find_language("tok").unwrap().label(), "toki pona (kulupu pona)");
    }
}

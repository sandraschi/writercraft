//! WriterCraft Internationalization (i18n) Engine.
//!
//! Follows the fleet standard: typed locale enums, zero-panic string lookups,
//! multi-language translations (EN, DE, FR, ES, JA, KO, ZH),
//! and literary narrative / publishing terminology localization.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AppLocale {
    #[default]
    En, // English (Global Default)
    De, // German (Goethe / Kafka literary native)
    Fr, // French (Proust / Hugo literary native)
    Es, // Spanish (Cervantes / Borges literary native)
    Ja, // Japanese (Murasaki Shikibu / Mishima literary native)
    Ko, // Korean
    Zh, // Chinese
}

impl AppLocale {
    pub fn all() -> &'static [AppLocale] {
        &[
            AppLocale::En,
            AppLocale::De,
            AppLocale::Fr,
            AppLocale::Es,
            AppLocale::Ja,
            AppLocale::Ko,
            AppLocale::Zh,
        ]
    }

    pub fn code(&self) -> &'static str {
        match self {
            AppLocale::En => "en",
            AppLocale::De => "de",
            AppLocale::Fr => "fr",
            AppLocale::Es => "es",
            AppLocale::Ja => "ja",
            AppLocale::Ko => "ko",
            AppLocale::Zh => "zh",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            AppLocale::En => "English (US/UK)",
            AppLocale::De => "Deutsch (DE/AT/CH)",
            AppLocale::Fr => "Français (FR)",
            AppLocale::Es => "Español (ES/LATAM)",
            AppLocale::Ja => "日本語 (JA)",
            AppLocale::Ko => "한국어 (KO)",
            AppLocale::Zh => "中文 (ZH)",
        }
    }

    pub fn flag(&self) -> &'static str {
        match self {
            AppLocale::En => "🇬🇧",
            AppLocale::De => "🇩🇪",
            AppLocale::Fr => "🇫🇷",
            AppLocale::Es => "🇪🇸",
            AppLocale::Ja => "🇯🇵",
            AppLocale::Ko => "🇰🇷",
            AppLocale::Zh => "🇨🇳",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum I18nKey {
    // Navigation & Workspace
    NavManuscriptEditor,
    NavCharacterBible,
    NavWorldbuildingAtlas,
    NavPlotTimeline,
    NavPublishingExport,

    // Actions & Tools
    WordCount,
    ReadingTime,
    ActionCompileManuscript,
    ActionExportEpub,
    ActionExportPdf,
    StatusReady,
}

pub fn t(key: I18nKey, locale: AppLocale) -> &'static str {
    match (key, locale) {
        (I18nKey::NavManuscriptEditor, AppLocale::En) => "✍️ Manuscript Editor",
        (I18nKey::NavManuscriptEditor, AppLocale::De) => "✍️ Manuskript-Editor",
        (I18nKey::NavManuscriptEditor, AppLocale::Fr) => "✍️ Éditeur de manuscrit",
        (I18nKey::NavManuscriptEditor, AppLocale::Es) => "✍️ Editor de manuscrito",
        (I18nKey::NavManuscriptEditor, AppLocale::Ja) => "✍️ 原稿・本文エディタ",
        (I18nKey::NavManuscriptEditor, AppLocale::Ko) => "✍️ 원고 편집기",
        (I18nKey::NavManuscriptEditor, AppLocale::Zh) => "✍️ 手稿正文编辑器",

        (I18nKey::NavCharacterBible, AppLocale::En) => "👥 Character Bible",
        (I18nKey::NavCharacterBible, AppLocale::De) => "👥 Charakter-Bibel",
        (I18nKey::NavCharacterBible, AppLocale::Fr) => "👥 Bible des personnages",
        (I18nKey::NavCharacterBible, AppLocale::Es) => "👥 Guía de personajes",
        (I18nKey::NavCharacterBible, AppLocale::Ja) => "👥 登場人物設定資料 (バイブル)",
        (I18nKey::NavCharacterBible, AppLocale::Ko) => "👥 인물 관계도 및 설정",
        (I18nKey::NavCharacterBible, AppLocale::Zh) => "👥 角色人物设定档案",

        (I18nKey::NavWorldbuildingAtlas, AppLocale::En) => "🌍 Worldbuilding Codex",
        (I18nKey::NavWorldbuildingAtlas, AppLocale::De) => "🌍 Weltenbau-Kodex",
        (I18nKey::NavWorldbuildingAtlas, AppLocale::Fr) => "🌍 Codex de création d'univers",
        (I18nKey::NavWorldbuildingAtlas, AppLocale::Es) => "🌍 Códice de construcción de mundo",
        (I18nKey::NavWorldbuildingAtlas, AppLocale::Ja) => "🌍 世界観設定・地誌コデックス",
        (I18nKey::NavWorldbuildingAtlas, AppLocale::Ko) => "🌍 세계관 설정 도감",
        (I18nKey::NavWorldbuildingAtlas, AppLocale::Zh) => "🌍 世界观架空百科典籍",

        (I18nKey::NavPlotTimeline, AppLocale::En) => "⏳ Narrative Arc & Plot Line",
        (I18nKey::NavPlotTimeline, AppLocale::De) => "⏳ Spannungsbogen & Handlungsstrang",
        (I18nKey::NavPlotTimeline, AppLocale::Fr) => "⏳ Arc narratif & Intrigue",
        (I18nKey::NavPlotTimeline, AppLocale::Es) => "⏳ Arco narrativo y Trama",
        (I18nKey::NavPlotTimeline, AppLocale::Ja) => "⏳ 物語の伏線・プロット年表",
        (I18nKey::NavPlotTimeline, AppLocale::Ko) => "⏳ 플롯 및 서사 곡선",
        (I18nKey::NavPlotTimeline, AppLocale::Zh) => "⏳ 叙事曲线与剧情时间轴",

        (I18nKey::NavPublishingExport, AppLocale::En) => "📦 Publishing Typesetter",
        (I18nKey::NavPublishingExport, AppLocale::De) => "📦 Verlags-Schriftsatz",
        (I18nKey::NavPublishingExport, AppLocale::Fr) => "📦 Typographie & Publication",
        (I18nKey::NavPublishingExport, AppLocale::Es) => "📦 Composición tipográfica",
        (I18nKey::NavPublishingExport, AppLocale::Ja) => "📦 出版組版・装丁出力",
        (I18nKey::NavPublishingExport, AppLocale::Ko) => "📦 출판 조판 및 배포",
        (I18nKey::NavPublishingExport, AppLocale::Zh) => "📦 出版排版与成书分发",

        (I18nKey::WordCount, AppLocale::En) => "Words",
        (I18nKey::WordCount, AppLocale::De) => "Wörter",
        (I18nKey::WordCount, AppLocale::Fr) => "Mots",
        (I18nKey::WordCount, AppLocale::Es) => "Palabras",
        (I18nKey::WordCount, AppLocale::Ja) => "文字数",
        (I18nKey::WordCount, AppLocale::Ko) => "단어 수",
        (I18nKey::WordCount, AppLocale::Zh) => "字数",

        (I18nKey::ReadingTime, AppLocale::En) => "Reading Time (min)",
        (I18nKey::ReadingTime, AppLocale::De) => "Lesezeit (Min.)",
        (I18nKey::ReadingTime, AppLocale::Fr) => "Temps de lecture (min)",
        (I18nKey::ReadingTime, AppLocale::Es) => "Tiempo de lectura (min)",
        (I18nKey::ReadingTime, AppLocale::Ja) => "読了予測時間 (分)",
        (I18nKey::ReadingTime, AppLocale::Ko) => "예상 읽기 시간 (분)",
        (I18nKey::ReadingTime, AppLocale::Zh) => "预估阅读耗时 (分钟)",

        (I18nKey::ActionCompileManuscript, AppLocale::En) => "Compile Master Manuscript",
        (I18nKey::ActionCompileManuscript, AppLocale::De) => "Gesamtmanuskript kompilieren",
        (I18nKey::ActionCompileManuscript, AppLocale::Fr) => "Compiler le manuscrit",
        (I18nKey::ActionCompileManuscript, AppLocale::Es) => "Compilar manuscrito final",
        (I18nKey::ActionCompileManuscript, AppLocale::Ja) => "原稿全体を一括コンパイル",
        (I18nKey::ActionCompileManuscript, AppLocale::Ko) => "원고 전체 컴파일",
        (I18nKey::ActionCompileManuscript, AppLocale::Zh) => "一键整合生成全书手稿",

        (I18nKey::ActionExportEpub, AppLocale::En) => "Export EPUB 3.0",
        (I18nKey::ActionExportEpub, AppLocale::De) => "EPUB 3.0 exportieren",
        (I18nKey::ActionExportEpub, AppLocale::Fr) => "Exporter en EPUB 3.0",
        (I18nKey::ActionExportEpub, AppLocale::Es) => "Exportar a EPUB 3.0",
        (I18nKey::ActionExportEpub, AppLocale::Ja) => "EPUB 3.0 電子書籍出力",
        (I18nKey::ActionExportEpub, AppLocale::Ko) => "EPUB 3.0 전자책 내보내기",
        (I18nKey::ActionExportEpub, AppLocale::Zh) => "导出标准 EPUB 3.0 电子书",

        (I18nKey::ActionExportPdf, AppLocale::En) => "Export Print-Ready PDF",
        (I18nKey::ActionExportPdf, AppLocale::De) => "Druckfähiges PDF exportieren",
        (I18nKey::ActionExportPdf, AppLocale::Fr) => "Exporter en PDF prêt à l'impression",
        (I18nKey::ActionExportPdf, AppLocale::Es) => "Exportar PDF para imprenta",
        (I18nKey::ActionExportPdf, AppLocale::Ja) => "商業印刷用 PDF 出力",
        (I18nKey::ActionExportPdf, AppLocale::Ko) => "인쇄용 PDF 내보내기",
        (I18nKey::ActionExportPdf, AppLocale::Zh) => "导出高清印刷级 PDF 排版",

        (I18nKey::StatusReady, AppLocale::En) => "WriterCraft Narrative Engine Ready",
        (I18nKey::StatusReady, AppLocale::De) => "WriterCraft Erzähl-Engine bereit",
        (I18nKey::StatusReady, AppLocale::Fr) => "Moteur narratif WriterCraft prêt",
        (I18nKey::StatusReady, AppLocale::Es) => "Motor narrativo WriterCraft listo",
        (I18nKey::StatusReady, AppLocale::Ja) => "WriterCraft 創作支援エンジン準備完了",
        (I18nKey::StatusReady, AppLocale::Ko) => "WriterCraft 서사 엔진 준비완료",
        (I18nKey::StatusReady, AppLocale::Zh) => "WriterCraft 叙事创作引擎就绪",
    }
}

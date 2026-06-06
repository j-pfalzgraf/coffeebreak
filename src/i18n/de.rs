//! German (de) locale table.

/// Translations for this locale. Any omitted message falls back to English.
pub static ENTRIES: super::Table = &[
    (super::Msg::PhaseFocus, "FOKUS"),
    (super::Msg::PhaseShortBreak, "PAUSE"),
    (super::Msg::PhaseLongBreak, "LANGE PAUSE"),
    (super::Msg::AnnounceFocus, "Zeit, sich zu konzentrieren."),
    (
        super::Msg::AnnounceShortBreak,
        "Kurze Pause — weg von der Tastatur.",
    ),
    (
        super::Msg::AnnounceLongBreak,
        "Lange Pause — du hast sie dir verdient.",
    ),
    (super::Msg::PomodoroOne, "Pomodoro"),
    (super::Msg::PomodoroOther, "Pomodoros"),
    (super::Msg::CycleOne, "Zyklus"),
    (super::Msg::CycleOther, "Zyklen"),
    (super::Msg::DayOne, "Tag"),
    (super::Msg::DayOther, "Tage"),
    (
        super::Msg::ControlsHint,
        "space Pause · s überspringen · +/- anpassen · q beenden",
    ),
    (super::Msg::Paused, "PAUSIERT"),
    (super::Msg::Left, "übrig"),
    (super::Msg::CycleOf, "Zyklus {n} von {total}"),
    (super::Msg::DoneFooter, "Fertig! {count} abgeschlossen. ☕"),
    (
        super::Msg::StoppedFooter,
        "Gestoppt — {count} in dieser Sitzung abgeschlossen.",
    ),
    (
        super::Msg::CelebrateMsg,
        "Sitzung abgeschlossen — {count} geschafft!",
    ),
    (
        super::Msg::PlanSummary,
        "{count} · Fokus {work} / Pause {brk}",
    ),
    (super::Msg::StatsTitle, "☕ coffeebreak — Statistiken"),
    (
        super::Msg::StatsEmpty,
        "Noch keine Pomodoros abgeschlossen — starte mit `coffeebreak`! ☕",
    ),
    (super::Msg::StatsToday, "Heute:"),
    (super::Msg::StatsAllTime, "Insgesamt:"),
    (super::Msg::StatsStreak, "Aktuelle Serie:"),
    (super::Msg::StatsBestDay, "Bester Tag:"),
    (super::Msg::StatsLongestStreak, "Längste Serie:"),
    (super::Msg::StatsGoal, "Tagesziel:"),
    (super::Msg::StatsLast14, "Letzte 14 Tage"),
    (super::Msg::StatsHeatmap, "Letzte 12 Wochen"),
    (super::Msg::HeatLess, "weniger"),
    (super::Msg::HeatMore, "mehr"),
    (super::Msg::GoalShort, "Ziel"),
    (super::Msg::GoalReached, "Ziel erreicht!"),
    (super::Msg::MinFocus, "Min. Fokus"),
    (super::Msg::Over, "über"),
    (super::Msg::DoctorTitle, "☕ coffeebreak — Diagnose"),
    (super::Msg::DoctorTerminal, "Terminal"),
    (super::Msg::DoctorTtyYes, "interaktiv (animierte UI)"),
    (super::Msg::DoctorTtyNo, "kein TTY (einfache Ausgabe)"),
    (super::Msg::DoctorColor, "Truecolour"),
    (super::Msg::DoctorColorYes, "unterstützt"),
    (
        super::Msg::DoctorColorNo,
        "nicht erkannt (COLORTERM=truecolor setzen)",
    ),
    (super::Msg::DoctorLang, "Sprache"),
    (super::Msg::DoctorConfig, "Konfigdatei"),
    (super::Msg::DoctorConfigExists, "vorhanden"),
    (
        super::Msg::DoctorConfigMissing,
        "nicht erstellt (ausführen: coffeebreak config init)",
    ),
    (super::Msg::DoctorData, "Datenverzeichnis"),
    (super::Msg::DoctorDataOk, "beschreibbar"),
    (super::Msg::DoctorDataNo, "nicht beschreibbar"),
    (super::Msg::DoctorNotify, "Benachrichtigungen"),
    (super::Msg::DoctorNotifyYes, "verfügbar"),
    (
        super::Msg::DoctorNotifyNo,
        "kein Benachrichtigungsdienst erkannt",
    ),
    (super::Msg::DoctorSound, "Ton"),
    (super::Msg::DoctorSoundChime, "rodio-Klang (sound feature)"),
    (super::Msg::DoctorSoundBell, "Terminalglocke"),
    (super::Msg::ThemesTitle, "Verfügbare Themes:"),
    (
        super::Msg::ThemesHint,
        "Verwende eines mit: coffeebreak --theme <name>",
    ),
    (super::Msg::PresetsTitle, "Verfügbare Presets:"),
    (
        super::Msg::PresetsHint,
        "Verwende eines mit: coffeebreak --preset <name>",
    ),
    (
        super::Msg::PresetCadence,
        "{work} Min. Fokus / {brk} Min. Pause · {count}",
    ),
    (
        super::Msg::PresetLong,
        ", lange Pause {long} Min. alle {every}",
    ),
    (super::Msg::LanguagesTitle, "Verfügbare Sprachen:"),
    (
        super::Msg::LanguagesHint,
        "Verwende eine mit: coffeebreak --lang <code>",
    ),
    (super::Msg::ConfigCreated, "Konfiguration erstellt:"),
    (
        super::Msg::ConfigExists,
        "Konfiguration existiert bereits unter",
    ),
    (super::Msg::UpdateCurrent, "Aktuelle Version: {version}"),
    (super::Msg::UpdateSource, "Quelle: {url}"),
    (
        super::Msg::UpdateUpToDate,
        "coffeebreak {version} ist aktuell.",
    ),
    (
        super::Msg::UpdateNewer,
        "Eine neuere Version ist verfügbar: {current} -> {latest}",
    ),
    (
        super::Msg::UpdateRunHint,
        "Führe `coffeebreak self update` aus, um zu aktualisieren.",
    ),
    (super::Msg::UpdateDone, "✓ Aktualisiert auf {version}."),
    (super::Msg::UpdateAlready, "Bereits aktuell ({version})."),
    (
        super::Msg::UninstallIntro,
        "Dies entfernt coffeebreak und seine Daten:",
    ),
    (super::Msg::UninstallItemBinary, "Binärdatei"),
    (super::Msg::UninstallItemConfig, "Konfiguration"),
    (super::Msg::UninstallItemData, "Daten"),
    (
        super::Msg::UninstallConfirm,
        "Alles oben Genannte entfernen?",
    ),
    (
        super::Msg::UninstallAborted,
        "Abgebrochen. Es wurde nichts entfernt.",
    ),
    (super::Msg::UninstallRemoved, "✓ {path} entfernt"),
    (
        super::Msg::UninstallBinFail,
        "Binärdatei konnte nicht automatisch entfernt werden ({error}).",
    ),
    (
        super::Msg::UninstallDone,
        "coffeebreak deinstalliert. ☕ Danke für die Fokus-Sitzungen!",
    ),
    (super::Msg::ConfirmYesNo, "[j/N]"),
    (super::Msg::ConfirmAffirmative, "j"),
    (
        super::Msg::NotATerminal,
        "kein Terminal; mit --yes erneut ausführen, um nicht-interaktiv zu bestätigen",
    ),
    (super::Msg::WordError, "Fehler"),
    (
        super::Msg::WarnStatsSave,
        "Statistiken konnten nicht gespeichert werden ({error})",
    ),
    (
        super::Msg::WarnStatsRead,
        "unlesbare Statistiken werden ignoriert ({error})",
    ),
    (
        super::Msg::WarnCtrlc,
        "Ctrl+C-Handler konnte nicht installiert werden ({error}); Statistiken werden bei einem Abbruch der Sitzung möglicherweise nicht gespeichert",
    ),
    (
        super::Msg::HelpAbout,
        "Ein Pomodoro-Fokus-Timer für dein Terminal ☕",
    ),
    (
        super::Msg::HelpLongAbout,
        "coffeebreak durchläuft Pomodoro-Fokus-/Pausenzyklen mit einer lebendigen, animierten Kaffeetasse, deren Dampf und Füllstand die Zeit anzeigen, großen Countdown-Ziffern, einem Verlaufsbalken, Desktop-Benachrichtigungen und einem Entwicklerzitat bei jeder Pause.",
    ),
    (
        super::Msg::HelpAfter,
        "Beispiele:\n  \
                 coffeebreak                         Klassisch 25/5, ein Zyklus\n  \
                 coffeebreak --preset classic        Vier 25/5-Runden, endend mit einer langen Pause\n  \
                 coffeebreak -w 50 -b 10 --cycles 3  Tiefe Arbeit: drei 50/10-Runden\n  \
                 coffeebreak --theme ocean           Das Farbthema ocean verwenden\n  \
                 coffeebreak --lang de               Auf Deutsch ausführen\n  \
                 coffeebreak --stats                 Deine Fokus-Statistiken anzeigen\n\n\
                 Während einer Sitzung:\n  \
                 space / p   pausieren oder fortsetzen   s / n   aktuelle Phase überspringen\n  \
                 + / =       eine Minute hinzufügen      - / _   eine Minute entfernen\n  \
                 q / Esc     beenden (Statistiken werden gespeichert)",
    ),
    (
        super::Msg::HelpStats,
        "Fokus-Statistiken anzeigen (heute, insgesamt, Serie, bester Tag)",
    ),
    (
        super::Msg::HelpConfig,
        "Konfigurationsdatei anzeigen oder erstellen",
    ),
    (
        super::Msg::HelpThemes,
        "Verfügbare Farbthemes mit Vorschau auflisten",
    ),
    (
        super::Msg::HelpPresets,
        "Verfügbare Timer-Presets auflisten",
    ),
    (
        super::Msg::HelpLanguages,
        "Verfügbare Oberflächensprachen auflisten",
    ),
    (
        super::Msg::HelpDoctor,
        "Umgebungsdiagnose ausführen (Terminal, Sprache, Konfig, …)",
    ),
    (
        super::Msg::HelpCompletions,
        "Ein Shell-Vervollständigungsskript erzeugen (bash, zsh, fish, …)",
    ),
    (
        super::Msg::HelpMan,
        "Eine roff-Manpage nach stdout ausgeben",
    ),
    (
        super::Msg::HelpSelf,
        "Die installierte coffeebreak-Binärdatei verwalten (update / uninstall)",
    ),
    (
        super::Msg::HelpSelfUpdate,
        "coffeebreak auf das neueste GitHub-Release aktualisieren",
    ),
    (
        super::Msg::HelpSelfUninstall,
        "Die coffeebreak-Binärdatei und ihre Konfigurations-/Datenverzeichnisse entfernen",
    ),
    (
        super::Msg::HelpConfigInit,
        "Eine Standard-Konfigurationsdatei schreiben (tut nichts, wenn bereits eine existiert)",
    ),
    (
        super::Msg::HelpConfigPath,
        "Den Pfad zur Konfigurationsdatei ausgeben",
    ),
    (
        super::Msg::HelpConfigShow,
        "Die effektive Konfiguration ausgeben",
    ),
    (
        super::Msg::HelpUpdateCheck,
        "Nur prüfen, ob eine neuere Version existiert; nicht installieren",
    ),
    (
        super::Msg::HelpUninstallYes,
        "Die Bestätigungsabfrage überspringen",
    ),
    (
        super::Msg::HelpCompletionsShell,
        "Die Shell, für die Vervollständigungen erzeugt werden",
    ),
    (
        super::Msg::HelpWork,
        "Länge des Fokusblocks in Minuten (Standard 25)",
    ),
    (
        super::Msg::HelpBreak,
        "Länge der Pause in Minuten (Standard 5)",
    ),
    (
        super::Msg::HelpCycles,
        "Anzahl der Fokus→Pause-Zyklen (Standard 1)",
    ),
    (
        super::Msg::HelpGoal,
        "Tägliches Pomodoro-Ziel in den Statistiken (0 = aus)",
    ),
    (
        super::Msg::HelpPreset,
        "Von einem benannten Preset starten: classic, deep, short, sprint",
    ),
    (
        super::Msg::HelpLong,
        "Eine lange Pause nach jeweils N Fokusblöcken aktivieren",
    ),
    (
        super::Msg::HelpLongBreak,
        "Länge der langen Pause in Minuten (impliziert --long; Standard 15)",
    ),
    (
        super::Msg::HelpLongEvery,
        "Wie viele Fokusblöcke vor einer langen Pause (Standard 4)",
    ),
    (
        super::Msg::HelpLabel,
        "Optionale Bezeichnung für diese Sitzung (in der Statuszeile angezeigt)",
    ),
    (
        super::Msg::HelpGitLabel,
        "Den aktuellen git-Branch als Sitzungsbezeichnung verwenden",
    ),
    (
        super::Msg::HelpTheme,
        "Farbthema: coffee, ocean, forest, grape, mono",
    ),
    (
        super::Msg::HelpFps,
        "Animationsbilder pro Sekunde (2–60; Standard 15)",
    ),
    (
        super::Msg::HelpPlain,
        "Einfache, nicht animierte Zeilenausgabe (auch automatisch bei Pipe)",
    ),
    (super::Msg::HelpNoColor, "Farbige Ausgabe deaktivieren"),
    (
        super::Msg::HelpNoSound,
        "Das akustische Signal bei Phasenwechsel stummschalten",
    ),
    (
        super::Msg::HelpNoNotify,
        "Keine Desktop-Benachrichtigungen senden",
    ),
    (
        super::Msg::HelpStatsFlag,
        "Heutige und Gesamt-Statistiken anzeigen, dann beenden",
    ),
    (
        super::Msg::HelpLang,
        "Oberflächensprache: en, de, es, fr, it, pt",
    ),
    (
        super::Msg::WaitContinue,
        "Beliebige Taste drücken zum Fortfahren · q zum Beenden",
    ),
    (
        super::Msg::WaitContinuePlain,
        "Eingabetaste drücken, um fortzufahren…",
    ),
    (
        super::Msg::HelpWait,
        "Zwischen den Phasen auf einen Tastendruck warten statt automatisch fortzufahren",
    ),
];

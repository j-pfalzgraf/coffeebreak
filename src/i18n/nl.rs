//! Dutch (nl) locale table.

/// Translations for this locale. Any omitted message falls back to English.
pub static ENTRIES: super::Table = &[
    (super::Msg::PhaseFocus, "FOCUS"),
    (super::Msg::PhaseShortBreak, "PAUZE"),
    (super::Msg::PhaseLongBreak, "LANGE PAUZE"),
    (super::Msg::AnnounceFocus, "Tijd om te focussen."),
    (
        super::Msg::AnnounceShortBreak,
        "Korte pauze — weg van het toetsenbord.",
    ),
    (
        super::Msg::AnnounceLongBreak,
        "Lange pauze — die heb je verdiend.",
    ),
    (super::Msg::PomodoroOne, "pomodoro"),
    (super::Msg::PomodoroOther, "pomodoro's"),
    (super::Msg::CycleOne, "cyclus"),
    (super::Msg::CycleOther, "cycli"),
    (super::Msg::DayOne, "dag"),
    (super::Msg::DayOther, "dagen"),
    (
        super::Msg::ControlsHint,
        "space pauze · s overslaan · +/- aanpassen · q stoppen",
    ),
    (super::Msg::Paused, "GEPAUZEERD"),
    (super::Msg::Left, "resterend"),
    (super::Msg::CycleOf, "cyclus {n} van {total}"),
    (
        super::Msg::WaitContinue,
        "Druk op een toets om door te gaan · q om te stoppen",
    ),
    (
        super::Msg::WaitContinuePlain,
        "Druk op Enter om door te gaan…",
    ),
    (super::Msg::DoneFooter, "Klaar! {count} voltooid. ☕"),
    (
        super::Msg::StoppedFooter,
        "Gestopt — {count} voltooid in deze sessie.",
    ),
    (
        super::Msg::CelebrateMsg,
        "Sessie voltooid — {count} gedaan!",
    ),
    (
        super::Msg::PlanSummary,
        "{count} · focus {work} / pauze {brk}",
    ),
    (super::Msg::StatsTitle, "☕ coffeebreak — statistieken"),
    (
        super::Msg::StatsEmpty,
        "Nog geen pomodoro's voltooid — start met `coffeebreak`! ☕",
    ),
    (super::Msg::StatsToday, "Vandaag:"),
    (super::Msg::StatsAllTime, "Totaal:"),
    (super::Msg::StatsStreak, "Huidige reeks:"),
    (super::Msg::StatsBestDay, "Beste dag:"),
    (super::Msg::StatsLongestStreak, "Langste reeks:"),
    (super::Msg::StatsGoal, "Dagdoel:"),
    (super::Msg::StatsLast14, "Laatste 14 dagen"),
    (super::Msg::StatsHeatmap, "Laatste 12 weken"),
    (super::Msg::HeatLess, "minder"),
    (super::Msg::HeatMore, "meer"),
    (super::Msg::GoalShort, "doel"),
    (super::Msg::GoalReached, "doel bereikt!"),
    (super::Msg::MinFocus, "min. focus"),
    (super::Msg::Over, "over"),
    (super::Msg::DoctorTitle, "☕ coffeebreak — diagnose"),
    (super::Msg::DoctorTerminal, "Terminal"),
    (super::Msg::DoctorTtyYes, "interactief (geanimeerde UI)"),
    (super::Msg::DoctorTtyNo, "geen TTY (eenvoudige uitvoer)"),
    (super::Msg::DoctorColor, "Truecolour"),
    (super::Msg::DoctorColorYes, "ondersteund"),
    (
        super::Msg::DoctorColorNo,
        "niet gedetecteerd (stel COLORTERM=truecolor in)",
    ),
    (super::Msg::DoctorLang, "Taal"),
    (super::Msg::DoctorConfig, "Configuratiebestand"),
    (super::Msg::DoctorConfigExists, "aanwezig"),
    (
        super::Msg::DoctorConfigMissing,
        "niet aangemaakt (voer uit: coffeebreak config init)",
    ),
    (super::Msg::DoctorData, "Gegevensmap"),
    (super::Msg::DoctorDataOk, "schrijfbaar"),
    (super::Msg::DoctorDataNo, "niet schrijfbaar"),
    (super::Msg::DoctorNotify, "Meldingen"),
    (super::Msg::DoctorNotifyYes, "beschikbaar"),
    (
        super::Msg::DoctorNotifyNo,
        "geen meldingsdienst gedetecteerd",
    ),
    (super::Msg::DoctorSound, "Geluid"),
    (super::Msg::DoctorSoundChime, "rodio-klank (sound feature)"),
    (super::Msg::DoctorSoundBell, "terminalbel"),
    (super::Msg::ThemesTitle, "Beschikbare thema's:"),
    (
        super::Msg::ThemesHint,
        "Gebruik er een met: coffeebreak --theme <name>",
    ),
    (super::Msg::PresetsTitle, "Beschikbare presets:"),
    (
        super::Msg::PresetsHint,
        "Gebruik er een met: coffeebreak --preset <name>",
    ),
    (
        super::Msg::PresetCadence,
        "{work} min. focus / {brk} min. pauze · {count}",
    ),
    (
        super::Msg::PresetLong,
        ", lange pauze {long} min. elke {every}",
    ),
    (super::Msg::LanguagesTitle, "Beschikbare talen:"),
    (
        super::Msg::LanguagesHint,
        "Gebruik er een met: coffeebreak --lang <code>",
    ),
    (super::Msg::ConfigCreated, "Configuratie aangemaakt:"),
    (super::Msg::ConfigExists, "Configuratie bestaat al op"),
    (super::Msg::UpdateCurrent, "Huidige versie: {version}"),
    (super::Msg::UpdateSource, "Bron: {url}"),
    (
        super::Msg::UpdateUpToDate,
        "coffeebreak {version} is up-to-date.",
    ),
    (
        super::Msg::UpdateNewer,
        "Er is een nieuwere versie beschikbaar: {current} -> {latest}",
    ),
    (
        super::Msg::UpdateRunHint,
        "Voer `coffeebreak self update` uit om bij te werken.",
    ),
    (super::Msg::UpdateDone, "✓ Bijgewerkt naar {version}."),
    (super::Msg::UpdateAlready, "Al up-to-date ({version})."),
    (
        super::Msg::UninstallIntro,
        "Dit verwijdert coffeebreak en zijn gegevens:",
    ),
    (super::Msg::UninstallItemBinary, "binary"),
    (super::Msg::UninstallItemConfig, "configuratie"),
    (super::Msg::UninstallItemData, "gegevens"),
    (super::Msg::UninstallConfirm, "Alles hierboven verwijderen?"),
    (
        super::Msg::UninstallAborted,
        "Afgebroken. Er is niets verwijderd.",
    ),
    (super::Msg::UninstallRemoved, "✓ {path} verwijderd"),
    (
        super::Msg::UninstallBinFail,
        "De binary kon niet automatisch worden verwijderd ({error}).",
    ),
    (
        super::Msg::UninstallDone,
        "coffeebreak gedeïnstalleerd. ☕ Bedankt voor de focussessies!",
    ),
    (super::Msg::ConfirmYesNo, "[j/N]"),
    (super::Msg::ConfirmAffirmative, "j"),
    (
        super::Msg::NotATerminal,
        "geen terminal; voer opnieuw uit met --yes om niet-interactief te bevestigen",
    ),
    (super::Msg::WordError, "fout"),
    (
        super::Msg::WarnStatsSave,
        "statistieken konden niet worden opgeslagen ({error})",
    ),
    (
        super::Msg::WarnStatsRead,
        "onleesbare statistieken worden genegeerd ({error})",
    ),
    (
        super::Msg::WarnStatsQuarantined,
        "het onleesbare statistiekenbestand is verplaatst naar {path} zodat het niet wordt overschreven",
    ),
    (
        super::Msg::WarnCtrlc,
        "Ctrl+C-handler kon niet worden geïnstalleerd ({error}); statistieken worden mogelijk niet opgeslagen als je de sessie onderbreekt",
    ),
    (
        super::Msg::HelpAbout,
        "Een Pomodoro-focustimer voor je terminal ☕",
    ),
    (
        super::Msg::HelpLongAbout,
        "coffeebreak doorloopt Pomodoro-focus-/pauzecycli met een levendige, geanimeerde koffiekop waarvan de stoom en vulling de tijd volgen, grote countdown-cijfers, een verloopvoortgangsbalk, desktopmeldingen en een ontwikkelaarscitaat bij elke pauze.",
    ),
    (
        super::Msg::HelpAfter,
        "Voorbeelden:\n  \
                 coffeebreak                         Klassiek 25/5, één cyclus\n  \
                 coffeebreak --preset classic        Vier 25/5-rondes, eindigend met een lange pauze\n  \
                 coffeebreak -w 50 -b 10 --cycles 3  Diep werk: drie 50/10-rondes\n  \
                 coffeebreak --theme ocean           Het kleurthema ocean gebruiken\n  \
                 coffeebreak --lang de               In het Duits uitvoeren\n  \
                 coffeebreak --stats                 Je focusstatistieken tonen\n\n\
                 Tijdens een sessie:\n  \
                 space / p   pauzeren of hervatten     s / n   huidige fase overslaan\n  \
                 + / =       een minuut toevoegen      - / _   een minuut verwijderen\n  \
                 q / Esc     stoppen (statistieken worden opgeslagen)",
    ),
    (
        super::Msg::HelpStats,
        "Focusstatistieken tonen (vandaag, totaal, reeks, beste dag)",
    ),
    (
        super::Msg::HelpConfig,
        "Het configuratiebestand bekijken of aanmaken",
    ),
    (
        super::Msg::HelpThemes,
        "De beschikbare kleurthema's met een voorbeeld tonen",
    ),
    (
        super::Msg::HelpPresets,
        "De beschikbare timer-presets tonen",
    ),
    (
        super::Msg::HelpLanguages,
        "De beschikbare interfacetalen tonen",
    ),
    (
        super::Msg::HelpDoctor,
        "Omgevingsdiagnose uitvoeren (terminal, taal, configuratie, …)",
    ),
    (
        super::Msg::HelpCompletions,
        "Een shell-completiescript genereren (bash, zsh, fish, …)",
    ),
    (
        super::Msg::HelpMan,
        "Een roff-manpagina naar stdout schrijven",
    ),
    (
        super::Msg::HelpSelf,
        "De geïnstalleerde coffeebreak-binary beheren (update / uninstall)",
    ),
    (
        super::Msg::HelpSelfUpdate,
        "coffeebreak bijwerken naar de nieuwste GitHub-release",
    ),
    (
        super::Msg::HelpSelfUninstall,
        "De coffeebreak-binary en zijn configuratie-/gegevensmappen verwijderen",
    ),
    (
        super::Msg::HelpConfigInit,
        "Een standaard configuratiebestand schrijven (doet niets als er al een bestaat)",
    ),
    (
        super::Msg::HelpConfigPath,
        "Het pad naar het configuratiebestand tonen",
    ),
    (
        super::Msg::HelpConfigShow,
        "De effectieve configuratie tonen",
    ),
    (
        super::Msg::HelpUpdateCheck,
        "Alleen controleren of er een nieuwere versie bestaat; niet installeren",
    ),
    (
        super::Msg::HelpUninstallYes,
        "De bevestigingsvraag overslaan",
    ),
    (
        super::Msg::HelpCompletionsShell,
        "De shell waarvoor completies worden gegenereerd",
    ),
    (
        super::Msg::HelpWork,
        "Lengte van het focusblok in minuten (standaard 25)",
    ),
    (
        super::Msg::HelpBreak,
        "Lengte van de pauze in minuten (standaard 5)",
    ),
    (
        super::Msg::HelpCycles,
        "Aantal focus→pauze-cycli (standaard 1)",
    ),
    (
        super::Msg::HelpGoal,
        "Dagelijks pomodoro-doel getoond in statistieken (0 = uit)",
    ),
    (
        super::Msg::HelpWait,
        "Tussen fases op een toetsaanslag wachten in plaats van automatisch door te gaan",
    ),
    (
        super::Msg::HelpPreset,
        "Starten vanaf een benoemde preset: classic, deep, short, sprint",
    ),
    (
        super::Msg::HelpLong,
        "Een lange pauze inschakelen na elke N focusblokken",
    ),
    (
        super::Msg::HelpLongBreak,
        "Lengte van de lange pauze in minuten (impliceert --long; standaard 15)",
    ),
    (
        super::Msg::HelpLongEvery,
        "Hoeveel focusblokken vóór een lange pauze (standaard 4)",
    ),
    (
        super::Msg::HelpLabel,
        "Optioneel label voor deze sessie (getoond in de statusbalk)",
    ),
    (
        super::Msg::HelpGitLabel,
        "De huidige git-branch als sessielabel gebruiken",
    ),
    (
        super::Msg::HelpTheme,
        "Kleurthema — zie `coffeebreak themes`: coffee, ocean, forest, grape, mono, dracula, nord, gruvbox, solarized, rose-pine, custom",
    ),
    (
        super::Msg::HelpFps,
        "Animatiebeelden per seconde (2–60; standaard 15)",
    ),
    (
        super::Msg::HelpPlain,
        "Eenvoudige, niet-geanimeerde regeluitvoer (ook automatisch bij doorsluizen)",
    ),
    (super::Msg::HelpNoColor, "Gekleurde uitvoer uitschakelen"),
    (
        super::Msg::HelpNoSound,
        "Het hoorbare signaal bij faseovergang dempen",
    ),
    (super::Msg::HelpNoNotify, "Geen desktopmeldingen versturen"),
    (
        super::Msg::HelpStatsFlag,
        "De statistieken van vandaag en totaal tonen, dan stoppen",
    ),
    (
        super::Msg::HelpLang,
        "Interfacetaal: en, de, es, fr, it, pt, nl",
    ),
    (
        super::Msg::HelpFormat,
        "Uitvoerformaat: text (dashboard), json of csv",
    ),
    (
        super::Msg::HelpAchievements,
        "Je verdiende badges en voortgang naar de volgende tonen",
    ),
    (
        super::Msg::HelpDemo,
        "Elke widget en animatie showcasen, dan afsluiten",
    ),
    (
        super::Msg::HelpIndicator,
        "Grote countdown-stijl: cijfers (standaard) of ring",
    ),
    (
        super::Msg::HelpBrew,
        "De zet-introanimatie afspelen vóór het eerste focusblok",
    ),
    (
        super::Msg::HelpHistory,
        "Toon het sessielogboek (schakel in met `history = true` in de configuratie)",
    ),
    (
        super::Msg::HelpHistoryLimit,
        "Toon hoogstens de laatste N sessies (0 = alle)",
    ),
    (
        super::Msg::HistoryTitle,
        "☕ coffeebreak — sessiegeschiedenis",
    ),
    (
        super::Msg::HistoryEmpty,
        "Nog geen sessies gelogd. Zet `history = true` in de configuratie en voltooi een focusblok. ☕",
    ),
    (super::Msg::HistoryColWhen, "Wanneer"),
    (super::Msg::HistoryColMinutes, "Min"),
    (super::Msg::HistoryColLabel, "Label"),
    (super::Msg::AchTitle, "🏅 coffeebreak — prestaties"),
    (
        super::Msg::AchEmpty,
        "Nog geen badges — voer `coffeebreak` uit om je eerste te verdienen! ☕",
    ),
    (super::Msg::AchUnlocked, "Ontgrendeld:"),
    (super::Msg::AchNext, "Volgende:"),
    (
        super::Msg::AchAllUnlocked,
        "Alle badges ontgrendeld — meesterlijk! ☕",
    ),
    (super::Msg::AchTierFirst, "Eerste stappen"),
    (super::Msg::AchTierVolume, "Volumemijlpalen"),
    (super::Msg::AchTierStreak, "Reeksmijlpalen"),
    (super::Msg::AchTierSingleDay, "Eendagsprestaties"),
    (super::Msg::AchTierConsistency, "Consistentie"),
    (super::Msg::AchFirstSipT, "Eerste Slok"),
    (super::Msg::AchFirstSipD, "Voltooi je allereerste pomodoro."),
    (super::Msg::AchGettingStartedT, "Goed Begonnen"),
    (
        super::Msg::AchGettingStartedD,
        "Bereik 10 pomodoro's in totaal.",
    ),
    (super::Msg::AchHalfCenturyT, "Halve Eeuw"),
    (super::Msg::AchHalfCenturyD, "50 pomodoro's voltooid."),
    (super::Msg::AchCenturionT, "Centurion"),
    (super::Msg::AchCenturionD, "100 pomodoro's voltooid."),
    (super::Msg::AchDeepDiverT, "Diepzeeduiker"),
    (super::Msg::AchDeepDiverD, "250 pomodoro's voltooid."),
    (super::Msg::AchMountaineerT, "Bergbeklimmer"),
    (super::Msg::AchMountaineerD, "500 pomodoro's voltooid."),
    (super::Msg::AchMillenniumT, "Millennium"),
    (super::Msg::AchMillenniumD, "1000 pomodoro's voltooid."),
    (super::Msg::AchHourMasterT, "Urenmeester"),
    (super::Msg::AchHourMasterD, "600 focusminuten in totaal."),
    (super::Msg::AchOnARollT, "Lekker Bezig"),
    (super::Msg::AchOnARollD, "Bereik een reeks van 3 dagen."),
    (super::Msg::AchWeekWarriorT, "Weekstrijder"),
    (super::Msg::AchWeekWarriorD, "Bereik een reeks van 7 dagen."),
    (super::Msg::AchFortnightT, "Twee Weken Focus"),
    (super::Msg::AchFortnightD, "Bereik een reeks van 14 dagen."),
    (super::Msg::AchUnbrokenT, "Ononderbroken"),
    (super::Msg::AchUnbrokenD, "Bereik een reeks van 30 dagen."),
    (super::Msg::AchProductiveDayT, "Productieve Dag"),
    (super::Msg::AchProductiveDayD, "4 pomodoro's op één dag."),
    (super::Msg::AchInTheZoneT, "In De Flow"),
    (super::Msg::AchInTheZoneD, "8 pomodoro's op één dag."),
    (super::Msg::AchMarathonT, "Marathonloper"),
    (super::Msg::AchMarathonD, "12 pomodoro's op één dag."),
    (super::Msg::AchWeekendFocusT, "Weekendfocus"),
    (
        super::Msg::AchWeekendFocusD,
        "Voltooi een pomodoro op een zaterdag of zondag.",
    ),
    (super::Msg::AchRegularT, "Vaste Klant"),
    (
        super::Msg::AchRegularD,
        "Wees actief op 5 van de laatste 7 dagen.",
    ),
    (super::Msg::AchGoalGetterT, "Doelpunter"),
    (super::Msg::AchGoalGetterD, "Haal vandaag je dagdoel."),
    (super::Msg::Brewing, "Zetten…"),
    (
        super::Msg::BrewSkipHint,
        "druk op een toets om over te slaan",
    ),
    (super::Msg::Checking, "controleren op updates…"),
    (
        super::Msg::DemoFooter,
        "een toets om af te sluiten · de live-UI animeert elk beeld",
    ),
    (
        super::Msg::DemoNotTty,
        "demo vereist een interactieve terminal (een TTY).",
    ),
    (super::Msg::SceneBrewing, "Zetten"),
    (super::Msg::SceneCup, "Koffiekop"),
    (super::Msg::SceneClock, "Aftellen"),
    (super::Msg::SceneRing, "Ringmeter"),
    (super::Msg::SceneSpinner, "Spinner"),
    (super::Msg::SceneCharts, "Grafieken"),
    (super::Msg::SceneFinale, "Feest"),
];

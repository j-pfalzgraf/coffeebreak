//! Italian (it) locale table.

/// Translations for this locale. Any omitted message falls back to English.
pub static ENTRIES: super::Table = &[
    (super::Msg::PhaseFocus, "CONCENTRAZIONE"),
    (super::Msg::PhaseShortBreak, "PAUSA"),
    (super::Msg::PhaseLongBreak, "PAUSA LUNGA"),
    (super::Msg::AnnounceFocus, "È ora di concentrarsi."),
    (super::Msg::AnnounceShortBreak, "Pausa breve — allontanati dalla tastiera."),
    (super::Msg::AnnounceLongBreak, "Pausa lunga — te la sei meritata."),

    (super::Msg::PomodoroOne, "pomodoro"),
    (super::Msg::PomodoroOther, "pomodori"),
    (super::Msg::CycleOne, "ciclo"),
    (super::Msg::CycleOther, "cicli"),
    (super::Msg::DayOne, "giorno"),
    (super::Msg::DayOther, "giorni"),

    (super::Msg::ControlsHint, "space pausa · s salta · +/- regola · q esci"),
    (super::Msg::Paused, "IN PAUSA"),
    (super::Msg::Left, "rimasti"),
    (super::Msg::CycleOf, "ciclo {n} di {total}"),

    (super::Msg::DoneFooter, "Fatto! {count} completati. ☕"),
    (super::Msg::StoppedFooter, "Interrotto — {count} completati in questa sessione."),
    (super::Msg::CelebrateMsg, "Sessione completata — {count} fatti!"),
    (super::Msg::PlanSummary, "{count} · concentrazione {work} / pausa {brk}"),

    (super::Msg::StatsTitle, "☕ coffeebreak — statistiche"),
    (super::Msg::StatsEmpty, "Nessun pomodoro completato ancora — esegui `coffeebreak` per iniziare! ☕"),
    (super::Msg::StatsToday, "Oggi:"),
    (super::Msg::StatsAllTime, "Da sempre:"),
    (super::Msg::StatsStreak, "Serie attuale:"),
    (super::Msg::StatsBestDay, "Giorno migliore:"),
    (super::Msg::MinFocus, "min concentrazione"),
    (super::Msg::Over, "in"),

    (super::Msg::ThemesTitle, "Temi disponibili:"),
    (super::Msg::ThemesHint, "Usane uno con: coffeebreak --theme <nome>"),
    (super::Msg::PresetsTitle, "Preset disponibili:"),
    (super::Msg::PresetsHint, "Usane uno con: coffeebreak --preset <nome>"),
    (super::Msg::PresetCadence, "{work} min concentrazione / {brk} min pausa · {count}"),
    (super::Msg::PresetLong, ", pausa lunga {long} min ogni {every}"),
    (super::Msg::LanguagesTitle, "Lingue disponibili:"),
    (super::Msg::LanguagesHint, "Usane una con: coffeebreak --lang <codice>"),

    (super::Msg::ConfigCreated, "Configurazione creata:"),
    (super::Msg::ConfigExists, "La configurazione esiste già in"),

    (super::Msg::UpdateCurrent, "Versione attuale: {version}"),
    (super::Msg::UpdateSource, "Sorgente: {url}"),
    (super::Msg::UpdateUpToDate, "coffeebreak {version} è aggiornato."),
    (super::Msg::UpdateNewer, "È disponibile una versione più recente: {current} -> {latest}"),
    (super::Msg::UpdateRunHint, "Esegui `coffeebreak self update` per aggiornare."),
    (super::Msg::UpdateDone, "✓ Aggiornato a {version}."),
    (super::Msg::UpdateAlready, "Già aggiornato ({version})."),
    (super::Msg::UninstallIntro, "Questo rimuoverà coffeebreak e i suoi dati:"),
    (super::Msg::UninstallItemBinary, "binario"),
    (super::Msg::UninstallItemConfig, "configurazione"),
    (super::Msg::UninstallItemData, "dati"),
    (super::Msg::UninstallConfirm, "Rimuovere tutto quanto sopra?"),
    (super::Msg::UninstallAborted, "Annullato. Niente è stato rimosso."),
    (super::Msg::UninstallRemoved, "✓ Rimosso {path}"),
    (super::Msg::UninstallBinFail, "Impossibile rimuovere il binario automaticamente ({error})."),
    (super::Msg::UninstallDone, "coffeebreak disinstallato. ☕ Grazie per le sessioni di concentrazione!"),
    (super::Msg::ConfirmYesNo, "[s/N]"),
    (super::Msg::ConfirmAffirmative, "s"),
    (super::Msg::NotATerminal, "non è un terminale; riesegui con --yes per confermare in modo non interattivo"),

    (super::Msg::WordError, "errore"),
    (super::Msg::WarnStatsSave, "impossibile salvare le statistiche ({error})"),
    (super::Msg::WarnStatsRead, "statistiche illeggibili ignorate ({error})"),
    (super::Msg::WarnCtrlc, "impossibile installare il gestore di Ctrl+C ({error}); le statistiche potrebbero non essere salvate se interrompi la sessione"),

    (super::Msg::HelpAbout, "Un timer Pomodoro per la concentrazione nel tuo terminale ☕"),
    (super::Msg::HelpLongAbout, "coffeebreak esegue cicli di concentrazione/pausa Pomodoro con una tazza di caffè animata e dal vivo il cui \
                                 vapore e riempimento seguono il tempo, grandi cifre per il conto alla rovescia, una barra di avanzamento sfumata, \
                                 notifiche desktop e una citazione da sviluppatore a ogni pausa."),
    (super::Msg::HelpAfter, "Esempi:\n  \
                             coffeebreak                         Classico 25/5, un ciclo\n  \
                             coffeebreak --preset classic        Quattro round 25/5, terminando con una pausa lunga\n  \
                             coffeebreak -w 50 -b 10 --cycles 3  Lavoro intenso: tre round 50/10\n  \
                             coffeebreak --theme ocean           Usa il tema di colori ocean\n  \
                             coffeebreak --lang de               Esegui in tedesco\n  \
                             coffeebreak --stats                 Mostra le tue statistiche di concentrazione\n\n\
                             Durante una sessione:\n  \
                             space / p   pausa o riprendi        s / n   salta la fase corrente\n  \
                             + / =       aggiungi un minuto      - / _   togli un minuto\n  \
                             q / Esc     esci (le statistiche vengono salvate)"),
    (super::Msg::HelpStats, "Mostra le statistiche di concentrazione (oggi, da sempre, serie, giorno migliore)"),
    (super::Msg::HelpConfig, "Ispeziona o crea il file di configurazione"),
    (super::Msg::HelpThemes, "Elenca i temi di colori disponibili con un'anteprima"),
    (super::Msg::HelpPresets, "Elenca i preset di timer disponibili"),
    (super::Msg::HelpLanguages, "Elenca le lingue dell'interfaccia disponibili"),
    (super::Msg::HelpCompletions, "Genera uno script di completamento per la shell (bash, zsh, fish, …)"),
    (super::Msg::HelpMan, "Stampa una pagina man roff su stdout"),
    (super::Msg::HelpSelf, "Gestisci il binario coffeebreak installato (update / uninstall)"),
    (super::Msg::HelpSelfUpdate, "Aggiorna coffeebreak all'ultima release di GitHub"),
    (super::Msg::HelpSelfUninstall, "Rimuovi il binario coffeebreak e le sue cartelle di configurazione/dati"),
    (super::Msg::HelpConfigInit, "Scrivi un file di configurazione predefinito (non fa nulla se ne esiste già uno)"),
    (super::Msg::HelpConfigPath, "Stampa il percorso del file di configurazione"),
    (super::Msg::HelpConfigShow, "Stampa la configurazione effettiva"),
    (super::Msg::HelpUpdateCheck, "Controlla solo se esiste una versione più recente; non installare"),
    (super::Msg::HelpUninstallYes, "Salta la richiesta di conferma"),
    (super::Msg::HelpCompletionsShell, "La shell per cui generare i completamenti"),
    (super::Msg::HelpWork, "Durata del blocco di concentrazione in minuti (predefinito 25)"),
    (super::Msg::HelpBreak, "Durata della pausa in minuti (predefinito 5)"),
    (super::Msg::HelpCycles, "Numero di cicli concentrazione→pausa da eseguire (predefinito 1)"),
    (super::Msg::HelpPreset, "Parti da un preset con nome: classic, deep, short, sprint"),
    (super::Msg::HelpLong, "Abilita una pausa lunga dopo ogni N blocchi di concentrazione"),
    (super::Msg::HelpLongBreak, "Durata della pausa lunga in minuti (implica --long; predefinito 15)"),
    (super::Msg::HelpLongEvery, "Quanti blocchi di concentrazione prima di una pausa lunga (predefinito 4)"),
    (super::Msg::HelpLabel, "Etichetta opzionale per questa sessione (mostrata nella riga di stato)"),
    (super::Msg::HelpGitLabel, "Usa il branch git corrente come etichetta della sessione"),
    (super::Msg::HelpTheme, "Tema di colori: coffee, ocean, forest, grape, mono"),
    (super::Msg::HelpFps, "Fotogrammi di animazione al secondo (2–60; predefinito 15)"),
    (super::Msg::HelpPlain, "Output di testo semplice e non animato (usato anche automaticamente con il piping)"),
    (super::Msg::HelpNoColor, "Disabilita l'output colorato"),
    (super::Msg::HelpNoSound, "Silenzia il segnale acustico al cambio di fase"),
    (super::Msg::HelpNoNotify, "Non inviare notifiche desktop"),
    (super::Msg::HelpStatsFlag, "Mostra le statistiche di oggi e da sempre, poi esci"),
    (super::Msg::HelpLang, "Lingua dell'interfaccia: en, de, es, fr, it, pt"),
];

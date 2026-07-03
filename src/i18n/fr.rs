//! French (fr) locale table.

/// Translations for this locale. Any omitted message falls back to English.
pub static ENTRIES: super::Table = &[
    (super::Msg::PhaseFocus, "CONCENTRATION"),
    (super::Msg::PhaseShortBreak, "PAUSE"),
    (super::Msg::PhaseLongBreak, "GRANDE PAUSE"),
    (super::Msg::AnnounceFocus, "C'est l'heure de se concentrer."),
    (
        super::Msg::AnnounceShortBreak,
        "Petite pause — éloignez-vous du clavier.",
    ),
    (
        super::Msg::AnnounceLongBreak,
        "Grande pause — vous l'avez méritée.",
    ),
    (super::Msg::PomodoroOne, "pomodoro"),
    (super::Msg::PomodoroOther, "pomodoros"),
    (super::Msg::CycleOne, "cycle"),
    (super::Msg::CycleOther, "cycles"),
    (super::Msg::DayOne, "jour"),
    (super::Msg::DayOther, "jours"),
    (
        super::Msg::ControlsHint,
        "space pause · s passer · +/- ajuster · q quitter",
    ),
    (super::Msg::Paused, "EN PAUSE"),
    (super::Msg::Left, "restant"),
    (super::Msg::CycleOf, "cycle {n} sur {total}"),
    (super::Msg::DoneFooter, "Terminé ! {count} accomplis. ☕"),
    (
        super::Msg::StoppedFooter,
        "Arrêté — {count} accomplis durant cette session.",
    ),
    (
        super::Msg::CelebrateMsg,
        "Session terminée — {count} accomplis !",
    ),
    (
        super::Msg::PlanSummary,
        "{count} · concentration {work} / pause {brk}",
    ),
    (super::Msg::StatsTitle, "☕ coffeebreak — statistiques"),
    (
        super::Msg::StatsEmpty,
        "Aucun pomodoro terminé pour l'instant — lancez `coffeebreak` pour commencer ! ☕",
    ),
    (super::Msg::StatsToday, "Aujourd'hui :"),
    (super::Msg::StatsAllTime, "Total :"),
    (super::Msg::StatsStreak, "Série en cours :"),
    (super::Msg::StatsBestDay, "Meilleur jour :"),
    (super::Msg::StatsLongestStreak, "Plus longue série :"),
    (super::Msg::StatsGoal, "Objectif quotidien :"),
    (super::Msg::StatsLast14, "14 derniers jours"),
    (super::Msg::StatsHeatmap, "12 dernières semaines"),
    (super::Msg::HeatLess, "moins"),
    (super::Msg::HeatMore, "plus"),
    (super::Msg::GoalShort, "objectif"),
    (super::Msg::GoalReached, "objectif atteint !"),
    (super::Msg::MinFocus, "min de concentration"),
    (super::Msg::Over, "sur"),
    (super::Msg::DoctorTitle, "☕ coffeebreak — diagnostic"),
    (super::Msg::DoctorTerminal, "Terminal"),
    (super::Msg::DoctorTtyYes, "interactif (UI animée)"),
    (super::Msg::DoctorTtyNo, "pas un TTY (sortie simple)"),
    (super::Msg::DoctorColor, "Truecolor"),
    (super::Msg::DoctorColorYes, "pris en charge"),
    (
        super::Msg::DoctorColorNo,
        "non détecté (définir COLORTERM=truecolor)",
    ),
    (super::Msg::DoctorLang, "Langue"),
    (super::Msg::DoctorConfig, "Fichier de config"),
    (super::Msg::DoctorConfigExists, "présent"),
    (
        super::Msg::DoctorConfigMissing,
        "non créé (lancer : coffeebreak config init)",
    ),
    (super::Msg::DoctorData, "Dossier de données"),
    (super::Msg::DoctorDataOk, "accessible en écriture"),
    (super::Msg::DoctorDataNo, "non accessible en écriture"),
    (super::Msg::DoctorNotify, "Notifications"),
    (super::Msg::DoctorNotifyYes, "disponibles"),
    (
        super::Msg::DoctorNotifyNo,
        "aucun service de notification détecté",
    ),
    (super::Msg::DoctorSound, "Son"),
    (
        super::Msg::DoctorSoundChime,
        "carillon rodio (sound feature)",
    ),
    (super::Msg::DoctorSoundBell, "cloche du terminal"),
    (super::Msg::ThemesTitle, "Thèmes disponibles :"),
    (
        super::Msg::ThemesHint,
        "Utilisez-en un avec : coffeebreak --theme <nom>",
    ),
    (super::Msg::PresetsTitle, "Préréglages disponibles :"),
    (
        super::Msg::PresetsHint,
        "Utilisez-en un avec : coffeebreak --preset <nom>",
    ),
    (
        super::Msg::PresetCadence,
        "{work} min de concentration / {brk} min de pause · {count}",
    ),
    (
        super::Msg::PresetLong,
        ", grande pause {long} min toutes les {every}",
    ),
    (super::Msg::LanguagesTitle, "Langues disponibles :"),
    (
        super::Msg::LanguagesHint,
        "Utilisez-en une avec : coffeebreak --lang <code>",
    ),
    (super::Msg::ConfigCreated, "Configuration créée :"),
    (super::Msg::ConfigExists, "La configuration existe déjà à"),
    (super::Msg::UpdateCurrent, "Version actuelle : {version}"),
    (super::Msg::UpdateSource, "Source : {url}"),
    (
        super::Msg::UpdateUpToDate,
        "coffeebreak {version} est à jour.",
    ),
    (
        super::Msg::UpdateNewer,
        "Une version plus récente est disponible : {current} -> {latest}",
    ),
    (
        super::Msg::UpdateRunHint,
        "Lancez `coffeebreak self update` pour mettre à jour.",
    ),
    (super::Msg::UpdateDone, "✓ Mis à jour vers {version}."),
    (super::Msg::UpdateAlready, "Déjà à jour ({version})."),
    (
        super::Msg::UninstallIntro,
        "Ceci supprimera coffeebreak et ses données :",
    ),
    (super::Msg::UninstallItemBinary, "binaire"),
    (super::Msg::UninstallItemConfig, "configuration"),
    (super::Msg::UninstallItemData, "données"),
    (super::Msg::UninstallConfirm, "Tout supprimer ?"),
    (
        super::Msg::UninstallAborted,
        "Annulé. Rien n'a été supprimé.",
    ),
    (super::Msg::UninstallRemoved, "✓ {path} supprimé"),
    (
        super::Msg::UninstallBinFail,
        "Impossible de supprimer le binaire automatiquement ({error}).",
    ),
    (
        super::Msg::UninstallDone,
        "coffeebreak désinstallé. ☕ Merci pour les sessions de concentration !",
    ),
    (super::Msg::ConfirmYesNo, "[o/N]"),
    (super::Msg::ConfirmAffirmative, "o"),
    (
        super::Msg::NotATerminal,
        "pas un terminal ; relancez avec --yes pour confirmer de façon non interactive",
    ),
    (super::Msg::WordError, "erreur"),
    (
        super::Msg::WarnStatsSave,
        "impossible d'enregistrer les statistiques ({error})",
    ),
    (
        super::Msg::WarnStatsRead,
        "statistiques illisibles ignorées ({error})",
    ),
    (
        super::Msg::WarnStatsQuarantined,
        "le fichier de statistiques illisible a été déplacé vers {path} pour ne pas être écrasé",
    ),
    (
        super::Msg::WarnCtrlc,
        "impossible d'installer le gestionnaire Ctrl+C ({error}) ; les statistiques peuvent ne pas être enregistrées si vous interrompez la session",
    ),
    (
        super::Msg::HelpAbout,
        "Un minuteur Pomodoro pour votre terminal ☕",
    ),
    (
        super::Msg::HelpLongAbout,
        "coffeebreak enchaîne des cycles de concentration et de pause Pomodoro avec une tasse de café animée en direct dont la vapeur et le remplissage suivent le temps, de grands chiffres de compte à rebours, une barre de progression en dégradé, des notifications de bureau et une citation de développeur à chaque pause.",
    ),
    (
        super::Msg::HelpAfter,
        "Exemples :\n  \
                 coffeebreak                         25/5 classique, un cycle\n  \
                 coffeebreak --preset classic        Quatre rounds 25/5, finissant par une grande pause\n  \
                 coffeebreak -w 50 -b 10 --cycles 3  Travail intense : trois rounds 50/10\n  \
                 coffeebreak --theme ocean           Utiliser le thème de couleurs ocean\n  \
                 coffeebreak --lang de               Lancer en allemand\n  \
                 coffeebreak --stats                 Afficher vos statistiques de concentration\n\n\
                 Pendant une session :\n  \
                 space / p   mettre en pause ou reprendre  s / n   passer la phase en cours\n  \
                 + / =       ajouter une minute            - / _   retirer une minute\n  \
                 q / Esc     quitter (les statistiques sont enregistrées)",
    ),
    (
        super::Msg::HelpStats,
        "Afficher les statistiques de concentration (aujourd'hui, total, série, meilleur jour)",
    ),
    (
        super::Msg::HelpConfig,
        "Inspecter ou créer le fichier de configuration",
    ),
    (
        super::Msg::HelpThemes,
        "Lister les thèmes de couleurs disponibles avec un aperçu",
    ),
    (
        super::Msg::HelpPresets,
        "Lister les préréglages de minuteur disponibles",
    ),
    (
        super::Msg::HelpLanguages,
        "Lister les langues d'interface disponibles",
    ),
    (
        super::Msg::HelpDoctor,
        "Lancer des diagnostics d'environnement (terminal, langue, config, …)",
    ),
    (
        super::Msg::HelpCompletions,
        "Générer un script de complétion shell (bash, zsh, fish, …)",
    ),
    (
        super::Msg::HelpMan,
        "Afficher une page de manuel roff sur la sortie standard",
    ),
    (
        super::Msg::HelpSelf,
        "Gérer le binaire coffeebreak installé (mettre à jour / désinstaller)",
    ),
    (
        super::Msg::HelpSelfUpdate,
        "Mettre à jour coffeebreak vers la dernière version GitHub",
    ),
    (
        super::Msg::HelpSelfUninstall,
        "Supprimer le binaire coffeebreak et ses dossiers de configuration/données",
    ),
    (
        super::Msg::HelpConfigInit,
        "Écrire un fichier de configuration par défaut (sans effet s'il en existe déjà un)",
    ),
    (
        super::Msg::HelpConfigPath,
        "Afficher le chemin du fichier de configuration",
    ),
    (
        super::Msg::HelpConfigShow,
        "Afficher la configuration effective",
    ),
    (
        super::Msg::HelpConfigGet,
        "Afficher la valeur d'une clé de configuration",
    ),
    (
        super::Msg::HelpConfigSet,
        "Définir une clé de configuration et enregistrer le fichier",
    ),
    (
        super::Msg::HelpConfigKey,
        "La clé à lire ou à modifier (toutes les clés : `coffeebreak config show`)",
    ),
    (
        super::Msg::HelpConfigValue,
        "La nouvelle valeur (validée avant l'enregistrement)",
    ),
    (super::Msg::ConfigSet, "Enregistré {key} = {value}"),
    (
        super::Msg::HelpUpdateCheck,
        "Vérifier seulement si une version plus récente existe ; ne pas installer",
    ),
    (
        super::Msg::HelpUninstallYes,
        "Ignorer la demande de confirmation",
    ),
    (
        super::Msg::HelpCompletionsShell,
        "Le shell pour lequel générer les complétions",
    ),
    (
        super::Msg::HelpWork,
        "Durée du bloc de concentration en minutes (par défaut 25)",
    ),
    (
        super::Msg::HelpBreak,
        "Durée de la pause en minutes (par défaut 5)",
    ),
    (
        super::Msg::HelpCycles,
        "Nombre de cycles concentration→pause à exécuter (par défaut 1)",
    ),
    (
        super::Msg::HelpGoal,
        "Objectif quotidien de pomodoros affiché dans les stats (0 = désactivé)",
    ),
    (
        super::Msg::HelpPreset,
        "Démarrer à partir d'un préréglage nommé : classic, deep, short, sprint",
    ),
    (
        super::Msg::HelpLong,
        "Activer une grande pause après chaque N blocs de concentration",
    ),
    (
        super::Msg::HelpLongBreak,
        "Durée de la grande pause en minutes (implique --long ; par défaut 15)",
    ),
    (
        super::Msg::HelpLongEvery,
        "Combien de blocs de concentration avant une grande pause (par défaut 4)",
    ),
    (
        super::Msg::HelpLabel,
        "Étiquette facultative pour cette session (affichée dans la ligne d'état)",
    ),
    (
        super::Msg::HelpGitLabel,
        "Utiliser la branche git actuelle comme étiquette de session",
    ),
    (
        super::Msg::HelpTheme,
        "Thème de couleurs : coffee, ocean, forest, grape, mono, dracula, nord, gruvbox, solarized, rose-pine, custom",
    ),
    (
        super::Msg::HelpFps,
        "Images d'animation par seconde (2–60 ; par défaut 15)",
    ),
    (
        super::Msg::HelpPlain,
        "Sortie en ligne simple, sans animation (utilisée aussi automatiquement en pipe)",
    ),
    (super::Msg::HelpNoColor, "Désactiver la sortie en couleurs"),
    (
        super::Msg::HelpNoSound,
        "Couper le signal sonore au changement de phase",
    ),
    (
        super::Msg::HelpNoNotify,
        "Ne pas envoyer de notifications de bureau",
    ),
    (
        super::Msg::HelpStatsFlag,
        "Afficher les statistiques du jour et globales, puis quitter",
    ),
    (
        super::Msg::HelpLang,
        "Langue d'interface : en, de, es, fr, it, pt, nl",
    ),
    (
        super::Msg::WaitContinue,
        "Appuie sur une touche pour continuer · q pour quitter",
    ),
    (
        super::Msg::WaitContinuePlain,
        "Appuie sur Entrée pour continuer…",
    ),
    (
        super::Msg::HelpWait,
        "Attendre une touche entre les phases au lieu d'enchaîner automatiquement",
    ),
    (
        super::Msg::HelpFormat,
        "Format de sortie : text (tableau de bord), json ou csv",
    ),
    (
        super::Msg::HelpAchievements,
        "Afficher vos badges obtenus et la progression vers le suivant",
    ),
    (
        super::Msg::HelpDemo,
        "Présenter tous les widgets et animations, puis quitter",
    ),
    (
        super::Msg::HelpIndicator,
        "Style du grand compte à rebours : digits (par défaut) ou ring",
    ),
    (
        super::Msg::HelpBrew,
        "Jouer l'animation d'intro d'infusion avant le premier bloc de concentration",
    ),
    (
        super::Msg::HelpHistory,
        "Afficher le journal des sessions (activez-le avec `history = true` dans la configuration)",
    ),
    (
        super::Msg::HelpHistoryLimit,
        "Afficher au plus les N dernières sessions (0 = toutes)",
    ),
    (
        super::Msg::HistoryTitle,
        "☕ coffeebreak — historique des sessions",
    ),
    (
        super::Msg::HistoryEmpty,
        "Aucune session enregistrée pour l'instant. Mettez `history = true` dans la configuration, puis terminez un bloc de concentration. ☕",
    ),
    (super::Msg::HistoryColWhen, "Quand"),
    (super::Msg::HistoryColMinutes, "Min"),
    (super::Msg::HistoryColLabel, "Libellé"),
    (super::Msg::AchTitle, "🏅 coffeebreak — accomplissements"),
    (
        super::Msg::AchEmpty,
        "Aucun badge pour l'instant — lancez `coffeebreak` pour obtenir le premier ! ☕",
    ),
    (super::Msg::AchUnlocked, "Débloqué :"),
    (super::Msg::AchNext, "Suivant :"),
    (
        super::Msg::AchAllUnlocked,
        "Tous les badges débloqués — magistral ! ☕",
    ),
    (super::Msg::AchTierFirst, "Premiers pas"),
    (super::Msg::AchTierVolume, "Jalons de volume"),
    (super::Msg::AchTierStreak, "Jalons de série"),
    (super::Msg::AchTierSingleDay, "Exploits d'une journée"),
    (super::Msg::AchTierConsistency, "Régularité"),
    (super::Msg::AchFirstSipT, "Première gorgée"),
    (
        super::Msg::AchFirstSipD,
        "Terminer votre tout premier pomodoro.",
    ),
    (super::Msg::AchGettingStartedT, "Sur les rails"),
    (
        super::Msg::AchGettingStartedD,
        "Atteindre 10 pomodoros au total.",
    ),
    (super::Msg::AchHalfCenturyT, "Demi-siècle"),
    (super::Msg::AchHalfCenturyD, "50 pomodoros terminés."),
    (super::Msg::AchCenturionT, "Centurion"),
    (super::Msg::AchCenturionD, "100 pomodoros terminés."),
    (super::Msg::AchDeepDiverT, "Plongeur"),
    (super::Msg::AchDeepDiverD, "250 pomodoros terminés."),
    (super::Msg::AchMountaineerT, "Alpiniste"),
    (super::Msg::AchMountaineerD, "500 pomodoros terminés."),
    (super::Msg::AchMillenniumT, "Millénaire"),
    (super::Msg::AchMillenniumD, "1000 pomodoros terminés."),
    (super::Msg::AchHourMasterT, "Maître des heures"),
    (
        super::Msg::AchHourMasterD,
        "600 minutes de concentration au total.",
    ),
    (super::Msg::AchOnARollT, "Sur sa lancée"),
    (super::Msg::AchOnARollD, "Atteindre une série de 3 jours."),
    (super::Msg::AchWeekWarriorT, "Guerrier de la semaine"),
    (
        super::Msg::AchWeekWarriorD,
        "Atteindre une série de 7 jours.",
    ),
    (super::Msg::AchFortnightT, "Quinzaine concentrée"),
    (
        super::Msg::AchFortnightD,
        "Atteindre une série de 14 jours.",
    ),
    (super::Msg::AchUnbrokenT, "Sans faille"),
    (super::Msg::AchUnbrokenD, "Atteindre une série de 30 jours."),
    (super::Msg::AchProductiveDayT, "Journée productive"),
    (
        super::Msg::AchProductiveDayD,
        "4 pomodoros en une seule journée.",
    ),
    (super::Msg::AchInTheZoneT, "Dans le flow"),
    (
        super::Msg::AchInTheZoneD,
        "8 pomodoros en une seule journée.",
    ),
    (super::Msg::AchMarathonT, "Marathonien"),
    (
        super::Msg::AchMarathonD,
        "12 pomodoros en une seule journée.",
    ),
    (super::Msg::AchWeekendFocusT, "Concentration du week-end"),
    (
        super::Msg::AchWeekendFocusD,
        "Terminer un pomodoro un samedi ou un dimanche.",
    ),
    (super::Msg::AchRegularT, "Habitué"),
    (
        super::Msg::AchRegularD,
        "Être actif 5 des 7 derniers jours.",
    ),
    (super::Msg::AchGoalGetterT, "Objectif atteint"),
    (
        super::Msg::AchGoalGetterD,
        "Atteindre votre objectif quotidien aujourd'hui.",
    ),
    (super::Msg::Brewing, "Infusion…"),
    (
        super::Msg::BrewSkipHint,
        "appuyez sur une touche pour passer",
    ),
    (super::Msg::Checking, "recherche de mises à jour…"),
    (
        super::Msg::DemoFooter,
        "une touche pour quitter · l'UI en direct s'anime à chaque image",
    ),
    (
        super::Msg::DemoNotTty,
        "la démo nécessite un terminal interactif (un TTY).",
    ),
    (super::Msg::SceneBrewing, "Infusion"),
    (super::Msg::SceneCup, "Tasse de café"),
    (super::Msg::SceneClock, "Compte à rebours"),
    (super::Msg::SceneRing, "Jauge en anneau"),
    (super::Msg::SceneSpinner, "Indicateur d'activité"),
    (super::Msg::SceneCharts, "Graphiques"),
    (super::Msg::SceneFinale, "Célébration"),
];

//! Portuguese (pt) locale table.

/// Translations for this locale. Any omitted message falls back to English.
pub static ENTRIES: super::Table = &[
    (super::Msg::PhaseFocus, "FOCO"),
    (super::Msg::PhaseShortBreak, "PAUSA"),
    (super::Msg::PhaseLongBreak, "PAUSA LONGA"),
    (super::Msg::AnnounceFocus, "Hora de focar."),
    (
        super::Msg::AnnounceShortBreak,
        "Pausa curta — afaste-se do teclado.",
    ),
    (super::Msg::AnnounceLongBreak, "Pausa longa — bem merecida."),
    (super::Msg::PomodoroOne, "pomodoro"),
    (super::Msg::PomodoroOther, "pomodoros"),
    (super::Msg::CycleOne, "ciclo"),
    (super::Msg::CycleOther, "ciclos"),
    (super::Msg::DayOne, "dia"),
    (super::Msg::DayOther, "dias"),
    (
        super::Msg::ControlsHint,
        "space pausar · s saltar · +/- ajustar · q sair",
    ),
    (super::Msg::Paused, "EM PAUSA"),
    (super::Msg::Left, "restante"),
    (super::Msg::CycleOf, "ciclo {n} de {total}"),
    (super::Msg::DoneFooter, "Concluído! {count} concluídos. ☕"),
    (
        super::Msg::StoppedFooter,
        "Parado — {count} concluídos nesta sessão.",
    ),
    (
        super::Msg::CelebrateMsg,
        "Sessão concluída — {count} feitos!",
    ),
    (
        super::Msg::PlanSummary,
        "{count} · foco {work} / pausa {brk}",
    ),
    (super::Msg::StatsTitle, "☕ coffeebreak — estatísticas"),
    (
        super::Msg::StatsEmpty,
        "Ainda não concluiu pomodoros — execute `coffeebreak` para começar! ☕",
    ),
    (super::Msg::StatsToday, "Hoje:"),
    (super::Msg::StatsAllTime, "Total:"),
    (super::Msg::StatsStreak, "Sequência atual:"),
    (super::Msg::StatsBestDay, "Melhor dia:"),
    (super::Msg::StatsLongestStreak, "Maior sequência:"),
    (super::Msg::StatsGoal, "Meta diária:"),
    (super::Msg::StatsLast14, "Últimos 14 dias"),
    (super::Msg::StatsHeatmap, "Últimas 12 semanas"),
    (super::Msg::HeatLess, "menos"),
    (super::Msg::HeatMore, "mais"),
    (super::Msg::GoalShort, "meta"),
    (super::Msg::GoalReached, "meta atingida!"),
    (super::Msg::MinFocus, "min de foco"),
    (super::Msg::Over, "ao longo de"),
    (super::Msg::DoctorTitle, "☕ coffeebreak — diagnóstico"),
    (super::Msg::DoctorTerminal, "Terminal"),
    (super::Msg::DoctorTtyYes, "interativo (UI animada)"),
    (super::Msg::DoctorTtyNo, "não é TTY (saída simples)"),
    (super::Msg::DoctorColor, "Truecolor"),
    (super::Msg::DoctorColorYes, "suportado"),
    (
        super::Msg::DoctorColorNo,
        "não detetado (defina COLORTERM=truecolor)",
    ),
    (super::Msg::DoctorLang, "Idioma"),
    (super::Msg::DoctorConfig, "Ficheiro de config"),
    (super::Msg::DoctorConfigExists, "presente"),
    (
        super::Msg::DoctorConfigMissing,
        "não criado (execute: coffeebreak config init)",
    ),
    (super::Msg::DoctorData, "Diretório de dados"),
    (super::Msg::DoctorDataOk, "gravável"),
    (super::Msg::DoctorDataNo, "não gravável"),
    (super::Msg::DoctorNotify, "Notificações"),
    (super::Msg::DoctorNotifyYes, "disponível"),
    (
        super::Msg::DoctorNotifyNo,
        "nenhum serviço de notificação detetado",
    ),
    (super::Msg::DoctorSound, "Som"),
    (super::Msg::DoctorSoundChime, "chime rodio (sound feature)"),
    (super::Msg::DoctorSoundBell, "campainha do terminal"),
    (super::Msg::ThemesTitle, "Temas disponíveis:"),
    (
        super::Msg::ThemesHint,
        "Use um com: coffeebreak --theme <nome>",
    ),
    (super::Msg::PresetsTitle, "Predefinições disponíveis:"),
    (
        super::Msg::PresetsHint,
        "Use uma com: coffeebreak --preset <nome>",
    ),
    (
        super::Msg::PresetCadence,
        "{work} min de foco / {brk} min de pausa · {count}",
    ),
    (
        super::Msg::PresetLong,
        ", pausa longa {long} min a cada {every}",
    ),
    (super::Msg::LanguagesTitle, "Idiomas disponíveis:"),
    (
        super::Msg::LanguagesHint,
        "Use um com: coffeebreak --lang <código>",
    ),
    (super::Msg::ConfigCreated, "Configuração criada:"),
    (super::Msg::ConfigExists, "A configuração já existe em"),
    (super::Msg::UpdateCurrent, "Versão atual: {version}"),
    (super::Msg::UpdateSource, "Origem: {url}"),
    (
        super::Msg::UpdateUpToDate,
        "O coffeebreak {version} está atualizado.",
    ),
    (
        super::Msg::UpdateNewer,
        "Está disponível uma versão mais recente: {current} -> {latest}",
    ),
    (
        super::Msg::UpdateRunHint,
        "Execute `coffeebreak self update` para atualizar.",
    ),
    (super::Msg::UpdateDone, "✓ Atualizado para {version}."),
    (super::Msg::UpdateAlready, "Já está atualizado ({version})."),
    (
        super::Msg::UninstallIntro,
        "Isto irá remover o coffeebreak e os seus dados:",
    ),
    (super::Msg::UninstallItemBinary, "binário"),
    (super::Msg::UninstallItemConfig, "configuração"),
    (super::Msg::UninstallItemData, "dados"),
    (
        super::Msg::UninstallConfirm,
        "Remover tudo o que está acima?",
    ),
    (
        super::Msg::UninstallAborted,
        "Cancelado. Nada foi removido.",
    ),
    (super::Msg::UninstallRemoved, "✓ Removido {path}"),
    (
        super::Msg::UninstallBinFail,
        "Não foi possível remover o binário automaticamente ({error}).",
    ),
    (
        super::Msg::UninstallDone,
        "coffeebreak desinstalado. ☕ Obrigado pelas sessões de foco!",
    ),
    (super::Msg::ConfirmYesNo, "[s/N]"),
    (super::Msg::ConfirmAffirmative, "s"),
    (
        super::Msg::NotATerminal,
        "não é um terminal; execute novamente com --yes para confirmar de forma não interativa",
    ),
    (super::Msg::WordError, "erro"),
    (
        super::Msg::WarnStatsSave,
        "não foi possível guardar as estatísticas ({error})",
    ),
    (
        super::Msg::WarnStatsRead,
        "a ignorar estatísticas ilegíveis ({error})",
    ),
    (
        super::Msg::WarnStatsQuarantined,
        "o ficheiro de estatísticas ilegível foi movido para {path} para não ser substituído",
    ),
    (
        super::Msg::WarnCtrlc,
        "não foi possível instalar o handler de Ctrl+C ({error}); as estatísticas podem não ser guardadas se interromper a sessão",
    ),
    (
        super::Msg::HelpAbout,
        "Um temporizador Pomodoro de foco para o seu terminal ☕",
    ),
    (
        super::Msg::HelpLongAbout,
        "O coffeebreak executa ciclos Pomodoro de foco/pausa com uma chávena de café animada ao vivo, cujo vapor e enchimento acompanham o tempo, dígitos grandes de contagem decrescente, uma barra de progresso em gradiente, notificações no ambiente de trabalho e uma citação para programadores em cada pausa.",
    ),
    (
        super::Msg::HelpAfter,
        "Exemplos:\n  \
         coffeebreak                         Clássico 25/5, um ciclo\n  \
         coffeebreak --preset classic        Quatro rondas 25/5, terminando numa pausa longa\n  \
         coffeebreak -w 50 -b 10 --cycles 3  Trabalho profundo: três rondas 50/10\n  \
         coffeebreak --theme ocean           Usa o tema de cor ocean\n  \
         coffeebreak --lang de               Executa em alemão\n  \
         coffeebreak --stats                 Mostra as suas estatísticas de foco\n\n\
         Durante uma sessão:\n  \
         space / p   pausar ou retomar       s / n   saltar a fase atual\n  \
         + / =       adicionar um minuto     - / _   remover um minuto\n  \
         q / Esc     sair (estatísticas guardadas)",
    ),
    (
        super::Msg::HelpStats,
        "Mostra estatísticas de foco (hoje, total, sequência, melhor dia)",
    ),
    (
        super::Msg::HelpConfig,
        "Inspeciona ou cria o ficheiro de configuração",
    ),
    (
        super::Msg::HelpThemes,
        "Lista os temas de cor disponíveis com uma pré-visualização",
    ),
    (
        super::Msg::HelpPresets,
        "Lista as predefinições de temporizador disponíveis",
    ),
    (
        super::Msg::HelpLanguages,
        "Lista os idiomas de interface disponíveis",
    ),
    (
        super::Msg::HelpDoctor,
        "Executa diagnósticos do ambiente (terminal, idioma, config, …)",
    ),
    (
        super::Msg::HelpCompletions,
        "Gera um script de conclusão da shell (bash, zsh, fish, …)",
    ),
    (
        super::Msg::HelpMan,
        "Imprime uma página man roff para stdout",
    ),
    (
        super::Msg::HelpSelf,
        "Gere o binário coffeebreak instalado (update / uninstall)",
    ),
    (
        super::Msg::HelpSelfUpdate,
        "Atualiza o coffeebreak para a versão mais recente do GitHub",
    ),
    (
        super::Msg::HelpSelfUninstall,
        "Remove o binário coffeebreak e os seus diretórios de configuração/dados",
    ),
    (
        super::Msg::HelpConfigInit,
        "Escreve um ficheiro de configuração predefinido (não faz nada se já existir um)",
    ),
    (
        super::Msg::HelpConfigPath,
        "Imprime o caminho para o ficheiro de configuração",
    ),
    (super::Msg::HelpConfigShow, "Imprime a configuração efetiva"),
    (
        super::Msg::HelpUpdateCheck,
        "Apenas verifica se existe uma versão mais recente; não instala",
    ),
    (super::Msg::HelpUninstallYes, "Ignora a confirmação"),
    (
        super::Msg::HelpCompletionsShell,
        "A shell para a qual gerar as conclusões",
    ),
    (
        super::Msg::HelpWork,
        "Duração do bloco de foco em minutos (predefinição 25)",
    ),
    (
        super::Msg::HelpBreak,
        "Duração da pausa em minutos (predefinição 5)",
    ),
    (
        super::Msg::HelpCycles,
        "Número de ciclos foco→pausa a executar (predefinição 1)",
    ),
    (
        super::Msg::HelpGoal,
        "Meta diária de pomodoros mostrada nas estatísticas (0 = desativada)",
    ),
    (
        super::Msg::HelpPreset,
        "Começa a partir de uma predefinição: classic, deep, short, sprint",
    ),
    (
        super::Msg::HelpLong,
        "Ativa uma pausa longa a cada N blocos de foco",
    ),
    (
        super::Msg::HelpLongBreak,
        "Duração da pausa longa em minutos (implica --long; predefinição 15)",
    ),
    (
        super::Msg::HelpLongEvery,
        "Quantos blocos de foco antes de uma pausa longa (predefinição 4)",
    ),
    (
        super::Msg::HelpLabel,
        "Rótulo opcional para esta sessão (mostrado na linha de estado)",
    ),
    (
        super::Msg::HelpGitLabel,
        "Usa o ramo git atual como rótulo da sessão",
    ),
    (
        super::Msg::HelpTheme,
        "Tema de cor: coffee, ocean, forest, grape, mono, dracula, nord, gruvbox, solarized, rose-pine, custom",
    ),
    (
        super::Msg::HelpFps,
        "Frames de animação por segundo (2–60; predefinição 15)",
    ),
    (
        super::Msg::HelpPlain,
        "Saída simples em linha, sem animação (usada automaticamente em pipe)",
    ),
    (super::Msg::HelpNoColor, "Desativa a saída colorida"),
    (
        super::Msg::HelpNoSound,
        "Silencia o aviso sonoro na mudança de fase",
    ),
    (
        super::Msg::HelpNoNotify,
        "Não envia notificações no ambiente de trabalho",
    ),
    (
        super::Msg::HelpStatsFlag,
        "Mostra as estatísticas de hoje e totais, depois sai",
    ),
    (
        super::Msg::HelpLang,
        "Idioma da interface: en, de, es, fr, it, pt, nl",
    ),
    (
        super::Msg::WaitContinue,
        "Pressione qualquer tecla para continuar · q para sair",
    ),
    (
        super::Msg::WaitContinuePlain,
        "Pressione Enter para continuar…",
    ),
    (
        super::Msg::HelpWait,
        "Aguardar uma tecla entre as fases em vez de avançar automaticamente",
    ),
    (
        super::Msg::HelpFormat,
        "Formato de saída: text (painel), json ou csv",
    ),
    (
        super::Msg::HelpAchievements,
        "Mostra os emblemas conquistados e o progresso até ao próximo",
    ),
    (
        super::Msg::HelpDemo,
        "Apresenta todos os widgets e animações, depois sai",
    ),
    (
        super::Msg::HelpIndicator,
        "Estilo da contagem decrescente grande: dígitos (predefinição) ou anel",
    ),
    (
        super::Msg::HelpBrew,
        "Reproduz a animação de preparação antes do primeiro bloco de foco",
    ),
    (super::Msg::AchTitle, "🏅 coffeebreak — conquistas"),
    (
        super::Msg::AchEmpty,
        "Ainda sem emblemas — execute `coffeebreak` para ganhar o primeiro! ☕",
    ),
    (super::Msg::AchUnlocked, "Desbloqueado:"),
    (super::Msg::AchNext, "Próximo:"),
    (
        super::Msg::AchAllUnlocked,
        "Todos os emblemas desbloqueados — magistral! ☕",
    ),
    (super::Msg::AchTierFirst, "Primeiros passos"),
    (super::Msg::AchTierVolume, "Marcos de volume"),
    (super::Msg::AchTierStreak, "Marcos de sequência"),
    (super::Msg::AchTierSingleDay, "Façanhas num só dia"),
    (super::Msg::AchTierConsistency, "Consistência"),
    (super::Msg::AchFirstSipT, "Primeiro Gole"),
    (super::Msg::AchFirstSipD, "Conclua o seu primeiro pomodoro."),
    (super::Msg::AchGettingStartedT, "A Começar"),
    (
        super::Msg::AchGettingStartedD,
        "Alcance 10 pomodoros no total.",
    ),
    (super::Msg::AchHalfCenturyT, "Meio Século"),
    (super::Msg::AchHalfCenturyD, "50 pomodoros concluídos."),
    (super::Msg::AchCenturionT, "Centurião"),
    (super::Msg::AchCenturionD, "100 pomodoros concluídos."),
    (super::Msg::AchDeepDiverT, "Mergulhador Profundo"),
    (super::Msg::AchDeepDiverD, "250 pomodoros concluídos."),
    (super::Msg::AchMountaineerT, "Alpinista"),
    (super::Msg::AchMountaineerD, "500 pomodoros concluídos."),
    (super::Msg::AchMillenniumT, "Milénio"),
    (super::Msg::AchMillenniumD, "1000 pomodoros concluídos."),
    (super::Msg::AchHourMasterT, "Mestre das Horas"),
    (super::Msg::AchHourMasterD, "600 minutos de foco no total."),
    (super::Msg::AchOnARollT, "Embalado"),
    (super::Msg::AchOnARollD, "Alcance uma sequência de 3 dias."),
    (super::Msg::AchWeekWarriorT, "Guerreiro da Semana"),
    (
        super::Msg::AchWeekWarriorD,
        "Alcance uma sequência de 7 dias.",
    ),
    (super::Msg::AchFortnightT, "Foco Quinzenal"),
    (
        super::Msg::AchFortnightD,
        "Alcance uma sequência de 14 dias.",
    ),
    (super::Msg::AchUnbrokenT, "Inquebrável"),
    (
        super::Msg::AchUnbrokenD,
        "Alcance uma sequência de 30 dias.",
    ),
    (super::Msg::AchProductiveDayT, "Dia Produtivo"),
    (super::Msg::AchProductiveDayD, "4 pomodoros num único dia."),
    (super::Msg::AchInTheZoneT, "Na Zona"),
    (super::Msg::AchInTheZoneD, "8 pomodoros num único dia."),
    (super::Msg::AchMarathonT, "Maratonista"),
    (super::Msg::AchMarathonD, "12 pomodoros num único dia."),
    (super::Msg::AchWeekendFocusT, "Foco de Fim de Semana"),
    (
        super::Msg::AchWeekendFocusD,
        "Conclua um pomodoro a um sábado ou domingo.",
    ),
    (super::Msg::AchRegularT, "Habitual"),
    (
        super::Msg::AchRegularD,
        "Esteja ativo em 5 dos últimos 7 dias.",
    ),
    (super::Msg::AchGoalGetterT, "Cumpridor de Metas"),
    (super::Msg::AchGoalGetterD, "Atinja a sua meta diária hoje."),
    (super::Msg::Brewing, "A preparar…"),
    (super::Msg::BrewSkipHint, "prima qualquer tecla para saltar"),
    (super::Msg::Checking, "a verificar atualizações…"),
    (
        super::Msg::DemoFooter,
        "qualquer tecla para sair · a UI ao vivo anima cada frame",
    ),
    (
        super::Msg::DemoNotTty,
        "a demonstração precisa de um terminal interativo (um TTY).",
    ),
    (super::Msg::SceneBrewing, "A preparar"),
    (super::Msg::SceneCup, "Chávena de café"),
    (super::Msg::SceneClock, "Contagem decrescente"),
    (super::Msg::SceneRing, "Medidor em anel"),
    (super::Msg::SceneSpinner, "Indicador rotativo"),
    (super::Msg::SceneCharts, "Gráficos"),
    (super::Msg::SceneFinale, "Celebração"),
];

//! Spanish (es) locale table.

/// Translations for this locale. Any omitted message falls back to English.
pub static ENTRIES: super::Table = &[
    (super::Msg::PhaseFocus, "CONCENTRACIÓN"),
    (super::Msg::PhaseShortBreak, "DESCANSO"),
    (super::Msg::PhaseLongBreak, "DESCANSO LARGO"),
    (super::Msg::AnnounceFocus, "Hora de concentrarse."),
    (
        super::Msg::AnnounceShortBreak,
        "Descanso corto — aléjate del teclado.",
    ),
    (
        super::Msg::AnnounceLongBreak,
        "Descanso largo — te lo has ganado.",
    ),
    (super::Msg::PomodoroOne, "pomodoro"),
    (super::Msg::PomodoroOther, "pomodoros"),
    (super::Msg::CycleOne, "ciclo"),
    (super::Msg::CycleOther, "ciclos"),
    (super::Msg::DayOne, "día"),
    (super::Msg::DayOther, "días"),
    (
        super::Msg::ControlsHint,
        "space pausar · s saltar · +/- ajustar · q salir",
    ),
    (super::Msg::Paused, "EN PAUSA"),
    (super::Msg::Left, "restante"),
    (super::Msg::CycleOf, "ciclo {n} de {total}"),
    (super::Msg::DoneFooter, "¡Hecho! {count} completados. ☕"),
    (
        super::Msg::StoppedFooter,
        "Detenido — {count} completados en esta sesión.",
    ),
    (
        super::Msg::CelebrateMsg,
        "Sesión completada — ¡{count} hechos!",
    ),
    (
        super::Msg::PlanSummary,
        "{count} · concentración {work} / descanso {brk}",
    ),
    (super::Msg::StatsTitle, "☕ coffeebreak — estadísticas"),
    (
        super::Msg::StatsEmpty,
        "Aún no has completado pomodoros — ejecuta `coffeebreak` para empezar. ☕",
    ),
    (super::Msg::StatsToday, "Hoy:"),
    (super::Msg::StatsAllTime, "Histórico:"),
    (super::Msg::StatsStreak, "Racha actual:"),
    (super::Msg::StatsBestDay, "Mejor día:"),
    (super::Msg::StatsLongestStreak, "Racha más larga:"),
    (super::Msg::StatsGoal, "Meta diaria:"),
    (super::Msg::StatsLast14, "Últimos 14 días"),
    (super::Msg::StatsHeatmap, "Últimas 12 semanas"),
    (super::Msg::HeatLess, "menos"),
    (super::Msg::HeatMore, "más"),
    (super::Msg::GoalShort, "meta"),
    (super::Msg::GoalReached, "¡meta alcanzada!"),
    (super::Msg::MinFocus, "min de concentración"),
    (super::Msg::Over, "en"),
    (super::Msg::DoctorTitle, "☕ coffeebreak — diagnóstico"),
    (super::Msg::DoctorTerminal, "Terminal"),
    (super::Msg::DoctorTtyYes, "interactivo (UI animada)"),
    (super::Msg::DoctorTtyNo, "no es TTY (salida simple)"),
    (super::Msg::DoctorColor, "Color verdadero"),
    (super::Msg::DoctorColorYes, "compatible"),
    (
        super::Msg::DoctorColorNo,
        "no detectado (define COLORTERM=truecolor)",
    ),
    (super::Msg::DoctorLang, "Idioma"),
    (super::Msg::DoctorConfig, "Archivo config"),
    (super::Msg::DoctorConfigExists, "presente"),
    (
        super::Msg::DoctorConfigMissing,
        "no creado (ejecuta: coffeebreak config init)",
    ),
    (super::Msg::DoctorData, "Directorio datos"),
    (super::Msg::DoctorDataOk, "escribible"),
    (super::Msg::DoctorDataNo, "no escribible"),
    (super::Msg::DoctorNotify, "Notificaciones"),
    (super::Msg::DoctorNotifyYes, "disponibles"),
    (
        super::Msg::DoctorNotifyNo,
        "no se detectó servicio de notificaciones",
    ),
    (super::Msg::DoctorSound, "Sonido"),
    (
        super::Msg::DoctorSoundChime,
        "campanilla rodio (sound feature)",
    ),
    (super::Msg::DoctorSoundBell, "campana del terminal"),
    (super::Msg::ThemesTitle, "Temas disponibles:"),
    (
        super::Msg::ThemesHint,
        "Usa uno con: coffeebreak --theme <nombre>",
    ),
    (
        super::Msg::PresetsTitle,
        "Ajustes predefinidos disponibles:",
    ),
    (
        super::Msg::PresetsHint,
        "Usa uno con: coffeebreak --preset <nombre>",
    ),
    (
        super::Msg::PresetCadence,
        "{work} min concentración / {brk} min descanso · {count}",
    ),
    (
        super::Msg::PresetLong,
        ", descanso largo {long} min cada {every}",
    ),
    (super::Msg::LanguagesTitle, "Idiomas disponibles:"),
    (
        super::Msg::LanguagesHint,
        "Usa uno con: coffeebreak --lang <código>",
    ),
    (super::Msg::ConfigCreated, "Configuración creada:"),
    (super::Msg::ConfigExists, "La configuración ya existe en"),
    (super::Msg::UpdateCurrent, "Versión actual: {version}"),
    (super::Msg::UpdateSource, "Origen: {url}"),
    (
        super::Msg::UpdateUpToDate,
        "coffeebreak {version} está actualizado.",
    ),
    (
        super::Msg::UpdateNewer,
        "Hay una versión más reciente disponible: {current} -> {latest}",
    ),
    (
        super::Msg::UpdateRunHint,
        "Ejecuta `coffeebreak self update` para actualizar.",
    ),
    (super::Msg::UpdateDone, "✓ Actualizado a {version}."),
    (
        super::Msg::UpdateAlready,
        "Ya está actualizado ({version}).",
    ),
    (
        super::Msg::UninstallIntro,
        "Esto eliminará coffeebreak y sus datos:",
    ),
    (super::Msg::UninstallItemBinary, "binario"),
    (super::Msg::UninstallItemConfig, "configuración"),
    (super::Msg::UninstallItemData, "datos"),
    (super::Msg::UninstallConfirm, "¿Eliminar todo lo anterior?"),
    (
        super::Msg::UninstallAborted,
        "Cancelado. No se eliminó nada.",
    ),
    (super::Msg::UninstallRemoved, "✓ Eliminado {path}"),
    (
        super::Msg::UninstallBinFail,
        "No se pudo eliminar el binario automáticamente ({error}).",
    ),
    (
        super::Msg::UninstallDone,
        "coffeebreak desinstalado. ☕ ¡Gracias por las sesiones de concentración!",
    ),
    (super::Msg::ConfirmYesNo, "[s/N]"),
    (super::Msg::ConfirmAffirmative, "s"),
    (
        super::Msg::NotATerminal,
        "no es un terminal; vuelve a ejecutar con --yes para confirmar de forma no interactiva",
    ),
    (super::Msg::WordError, "error"),
    (
        super::Msg::WarnStatsSave,
        "no se pudieron guardar las estadísticas ({error})",
    ),
    (
        super::Msg::WarnStatsRead,
        "se ignoran estadísticas ilegibles ({error})",
    ),
    (
        super::Msg::WarnCtrlc,
        "no se pudo instalar el manejador de Ctrl+C ({error}); puede que las estadísticas no se guarden si interrumpes la sesión",
    ),
    (
        super::Msg::HelpAbout,
        "Un temporizador Pomodoro de concentración para tu terminal ☕",
    ),
    (
        super::Msg::HelpLongAbout,
        "coffeebreak ejecuta ciclos Pomodoro de concentración/descanso con una taza de café animada en vivo \
         cuyo vapor y llenado siguen el tiempo, dígitos grandes de cuenta atrás, una barra de progreso degradada, \
         notificaciones de escritorio y una cita para desarrolladores en cada descanso.",
    ),
    (
        super::Msg::HelpAfter,
        "Ejemplos:\n  \
         coffeebreak                         Clásico 25/5, un ciclo\n  \
         coffeebreak --preset classic        Cuatro rondas 25/5, terminando en un descanso largo\n  \
         coffeebreak -w 50 -b 10 --cycles 3  Trabajo profundo: tres rondas 50/10\n  \
         coffeebreak --theme ocean           Usa el tema de color ocean\n  \
         coffeebreak --lang de               Ejecuta en alemán\n  \
         coffeebreak --stats                 Muestra tus estadísticas de concentración\n\n\
         Durante una sesión:\n  \
         space / p   pausar o reanudar       s / n   saltar la fase actual\n  \
         + / =       añadir un minuto        - / _   quitar un minuto\n  \
         q / Esc     salir (se guardan las estadísticas)",
    ),
    (
        super::Msg::HelpStats,
        "Muestra estadísticas de concentración (hoy, histórico, racha, mejor día)",
    ),
    (
        super::Msg::HelpConfig,
        "Inspecciona o crea el archivo de configuración",
    ),
    (
        super::Msg::HelpThemes,
        "Lista los temas de color disponibles con una vista previa",
    ),
    (
        super::Msg::HelpPresets,
        "Lista los ajustes predefinidos del temporizador disponibles",
    ),
    (
        super::Msg::HelpLanguages,
        "Lista los idiomas de la interfaz disponibles",
    ),
    (
        super::Msg::HelpDoctor,
        "Ejecuta diagnósticos del entorno (terminal, idioma, configuración, …)",
    ),
    (
        super::Msg::HelpCompletions,
        "Genera un script de autocompletado para la shell (bash, zsh, fish, …)",
    ),
    (
        super::Msg::HelpMan,
        "Imprime una página de manual roff en stdout",
    ),
    (
        super::Msg::HelpSelf,
        "Gestiona el binario instalado de coffeebreak (actualizar / desinstalar)",
    ),
    (
        super::Msg::HelpSelfUpdate,
        "Actualiza coffeebreak a la última versión de GitHub",
    ),
    (
        super::Msg::HelpSelfUninstall,
        "Elimina el binario de coffeebreak y sus directorios de configuración/datos",
    ),
    (
        super::Msg::HelpConfigInit,
        "Escribe un archivo de configuración por defecto (no hace nada si ya existe uno)",
    ),
    (
        super::Msg::HelpConfigPath,
        "Imprime la ruta del archivo de configuración",
    ),
    (
        super::Msg::HelpConfigShow,
        "Imprime la configuración efectiva",
    ),
    (
        super::Msg::HelpUpdateCheck,
        "Solo comprueba si existe una versión más reciente; no instala",
    ),
    (
        super::Msg::HelpUninstallYes,
        "Omite la solicitud de confirmación",
    ),
    (
        super::Msg::HelpCompletionsShell,
        "La shell para la que generar el autocompletado",
    ),
    (
        super::Msg::HelpWork,
        "Duración del bloque de concentración en minutos (por defecto 25)",
    ),
    (
        super::Msg::HelpBreak,
        "Duración del descanso en minutos (por defecto 5)",
    ),
    (
        super::Msg::HelpCycles,
        "Número de ciclos concentración→descanso a ejecutar (por defecto 1)",
    ),
    (
        super::Msg::HelpGoal,
        "Meta diaria de pomodoros mostrada en estadísticas (0 = desactivada)",
    ),
    (
        super::Msg::HelpPreset,
        "Empieza desde un ajuste predefinido: classic, deep, short, sprint",
    ),
    (
        super::Msg::HelpLong,
        "Activa un descanso largo cada N bloques de concentración",
    ),
    (
        super::Msg::HelpLongBreak,
        "Duración del descanso largo en minutos (implica --long; por defecto 15)",
    ),
    (
        super::Msg::HelpLongEvery,
        "Cuántos bloques de concentración antes de un descanso largo (por defecto 4)",
    ),
    (
        super::Msg::HelpLabel,
        "Etiqueta opcional para esta sesión (se muestra en la línea de estado)",
    ),
    (
        super::Msg::HelpGitLabel,
        "Usa la rama de git actual como etiqueta de la sesión",
    ),
    (
        super::Msg::HelpTheme,
        "Tema de color: coffee, ocean, forest, grape, mono, custom",
    ),
    (
        super::Msg::HelpFps,
        "Fotogramas de animación por segundo (2–60; por defecto 15)",
    ),
    (
        super::Msg::HelpPlain,
        "Salida en líneas simples, sin animación (también se usa automáticamente al canalizar)",
    ),
    (super::Msg::HelpNoColor, "Desactiva la salida en color"),
    (
        super::Msg::HelpNoSound,
        "Silencia el aviso sonoro al cambiar de fase",
    ),
    (
        super::Msg::HelpNoNotify,
        "No envía notificaciones de escritorio",
    ),
    (
        super::Msg::HelpStatsFlag,
        "Muestra las estadísticas de hoy y el histórico, y luego sale",
    ),
    (
        super::Msg::HelpLang,
        "Idioma de la interfaz: en, de, es, fr, it, pt",
    ),
    (
        super::Msg::WaitContinue,
        "Pulsa cualquier tecla para continuar · q para salir",
    ),
    (super::Msg::WaitContinuePlain, "Pulsa Intro para continuar…"),
    (
        super::Msg::HelpWait,
        "Esperar una tecla entre fases en lugar de avanzar automáticamente",
    ),
    (
        super::Msg::HelpFormat,
        "Formato de salida: text (panel), json o csv",
    ),
];

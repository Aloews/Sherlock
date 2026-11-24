#[cfg(target_arch = "wasm32")]
use gloo_timers::callback::Interval;
use leptos::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
use sherlock_game_core::{Game, GamePhase, GameSettings, bundled_cards};
mod dynasty;
use dynasty::DynastyApp;

#[cfg(target_arch = "wasm32")]
type TimerHandle = Interval;

#[cfg(not(target_arch = "wasm32"))]
type TimerHandle = ();

fn start_timer(
    timer: StoredValue<Option<TimerHandle>, LocalStorage>,
    set_game: WriteSignal<Option<Game>>,
) {
    #[cfg(target_arch = "wasm32")]
    {
        let interval = Interval::new(1_000, move || {
            set_game.update(|current| {
                if let Some(game) = current {
                    game.tick();
                }
            });
        });
        timer.set_value(Some(interval));
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (timer, set_game);
    }
}

#[cfg(target_arch = "wasm32")]
fn random_seed() -> u64 {
    (js_sys::Date::now() as u64) ^ ((js_sys::Math::random() * u64::MAX as f64) as u64)
}

#[cfg(not(target_arch = "wasm32"))]
fn random_seed() -> u64 {
    20_261_123
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PrimaryRoute {
    Home,
    Alias,
    Players,
    Clubs,
    Matches,
    Statistics,
    Forecasts,
    Fantasy,
    News,
    AmateurFootball,
    Dynasty,
}

impl PrimaryRoute {
    #[cfg(any(target_arch = "wasm32", test))]
    fn from_hash(hash: &str) -> Self {
        let path = hash
            .strip_prefix("#/")
            .unwrap_or(hash)
            .trim_matches('/')
            .split('?')
            .next()
            .unwrap_or("");
        match path {
            "alias" => Self::Alias,
            "players" => Self::Players,
            "clubs" => Self::Clubs,
            "matches" => Self::Matches,
            "statistics" => Self::Statistics,
            "forecasts" => Self::Forecasts,
            "fantasy" => Self::Fantasy,
            "news" => Self::News,
            "amateur" => Self::AmateurFootball,
            "dynasty" => Self::Dynasty,
            _ => Self::Home,
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn current_route() -> PrimaryRoute {
    web_sys::window()
        .and_then(|window| window.location().hash().ok())
        .map(|hash| PrimaryRoute::from_hash(&hash))
        .unwrap_or(PrimaryRoute::Home)
}

#[cfg(not(target_arch = "wasm32"))]
fn current_route() -> PrimaryRoute {
    PrimaryRoute::Home
}

#[component]
fn PrimaryNavigation(active_route: ReadSignal<PrimaryRoute>) -> impl IntoView {
    view! {
        <nav class="primary-navigation" aria-label="Основная навигация">
            <a href="#/home" class:active=move || active_route.get() == PrimaryRoute::Home
                aria-current=move || if active_route.get() == PrimaryRoute::Home { "page" } else { "false" }>"Главная"</a>
            <a href="#/alias" class:active=move || active_route.get() == PrimaryRoute::Alias
                aria-current=move || if active_route.get() == PrimaryRoute::Alias { "page" } else { "false" }>"Элиас"</a>
            <a href="#/players" class:active=move || active_route.get() == PrimaryRoute::Players
                aria-current=move || if active_route.get() == PrimaryRoute::Players { "page" } else { "false" }>"Игроки"</a>
            <a href="#/clubs" class:active=move || active_route.get() == PrimaryRoute::Clubs
                aria-current=move || if active_route.get() == PrimaryRoute::Clubs { "page" } else { "false" }>"Клубы"</a>
            <a href="#/matches" class:active=move || active_route.get() == PrimaryRoute::Matches
                aria-current=move || if active_route.get() == PrimaryRoute::Matches { "page" } else { "false" }>"Матчи"</a>
            <a href="#/statistics" class:active=move || active_route.get() == PrimaryRoute::Statistics
                aria-current=move || if active_route.get() == PrimaryRoute::Statistics { "page" } else { "false" }>"Статистика"</a>
            <a href="#/forecasts" class:active=move || active_route.get() == PrimaryRoute::Forecasts
                aria-current=move || if active_route.get() == PrimaryRoute::Forecasts { "page" } else { "false" }>"Прогнозы"</a>
            <a href="#/fantasy" class:active=move || active_route.get() == PrimaryRoute::Fantasy
                aria-current=move || if active_route.get() == PrimaryRoute::Fantasy { "page" } else { "false" }>"Фэнтези"</a>
            <a href="#/news" class:active=move || active_route.get() == PrimaryRoute::News
                aria-current=move || if active_route.get() == PrimaryRoute::News { "page" } else { "false" }>"Новости"</a>
            <a href="#/amateur" class:active=move || active_route.get() == PrimaryRoute::AmateurFootball
                aria-current=move || if active_route.get() == PrimaryRoute::AmateurFootball { "page" } else { "false" }>"Любительский"</a>
            <a href="#/dynasty" class:active=move || active_route.get() == PrimaryRoute::Dynasty
                aria-current=move || if active_route.get() == PrimaryRoute::Dynasty { "page" } else { "false" }>"Династия"</a>
        </nav>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <section class="product-home">
            <div class="eyebrow"><span class="eyebrow-dot"></span>"SHERLOCK FOOTBALL"</div>
            <h1>"Футбол. "<span>"Без границ."</span></h1>
            <p class="product-home-lead">"Игровые режимы уже доступны локально. Остальные разделы подключаются к проверенным источникам данных по мере миграции."</p>
            <div class="home-mode-grid">
                <a class="home-mode-card" href="#/alias">
                    <span class="home-mode-kicker">"ИГРОВОЙ РЕЖИМ"</span>
                    <strong>"Футбольный Элиас"</strong>
                    <span>"Карточки, раунды, таймер и счёт на одном устройстве."</span>
                    <span class="home-mode-action">"Открыть игру →"</span>
                </a>
                <a class="home-mode-card" href="#/dynasty">
                    <span class="home-mode-kicker">"КАРЬЕРНЫЙ РЕЖИМ"</span>
                    <strong>"Футбольная династия"</strong>
                    <span>"Клуб, состав, тренировки, трансферы и сезонная лига."</span>
                    <span class="home-mode-action">"Открыть карьеру →"</span>
                </a>
            </div>
            <div class="home-data-note">
                <span class="home-data-dot"></span>
                <div><strong>"Данные не подменяются макетами"</strong><p>"Профили, составы, результаты и прогнозы не показывают выдуманные значения. Разделы без проверенного источника отмечены отдельно."</p></div>
            </div>
        </section>
    }
}

#[component]
fn FeatureUnavailablePage(
    title: &'static str,
    status: &'static str,
    detail: &'static str,
) -> impl IntoView {
    view! {
        <section class="feature-page">
            <div class="eyebrow"><span class="eyebrow-dot"></span>"SHERLOCK / FOOTBALL"</div>
            <h1>{title}</h1>
            <div class="feature-status">
                <span class="feature-status-mark"></span>
                <strong>{status}</strong>
            </div>
            <p class="feature-detail">{detail}</p>
            <div class="feature-data-contract">
                <strong>"Правило данных"</strong>
                <p>"Нет подтверждённого источника — нет выдуманных записей. Пустые, неизвестные и недоступные значения не превращаются в нули."</p>
            </div>
            <a class="feature-back-link" href="#/home">"← На главную"</a>
        </section>
    }
}

#[component]
fn PlayersPage() -> impl IntoView {
    view! { <FeatureUnavailablePage title="Игроки" status="Профили и статистика не подключены" detail="Каталог карточек Элиаса не является источником подтверждённых биографий, карьерных итогов или сезонной статистики игроков."/> }
}

#[component]
fn ClubsPage() -> impl IntoView {
    view! { <FeatureUnavailablePage title="Клубы" status="Реальные составы не подключены" detail="Нет подтверждённого серверного источника актуальных составов, контрактов, тренеров и рыночной стоимости клубов."/> }
}

#[component]
fn MatchesPage() -> impl IntoView {
    view! { <FeatureUnavailablePage title="Матчи" status="Результаты и календарь не подключены" detail="Вымышленные матчи карьеры «Династия» отделены от реальных матчей и не отображаются как футбольные результаты."/> }
}

#[component]
fn StatisticsPage() -> impl IntoView {
    view! { <FeatureUnavailablePage title="Статистика" status="Проверенная статистическая модель не подключена" detail="Карьерные, сезонные и матчевые показатели должны храниться раздельно. Неполные данные не выдаются за полную статистику."/> }
}

#[component]
fn ForecastsPage() -> impl IntoView {
    view! { <FeatureUnavailablePage title="Прогнозы" status="Калиброванные прогнозы не подключены" detail="Вероятности, история прогнозов и качество моделей не публикуются без проверенных входных данных и измеренных результатов."/> }
}

#[component]
fn FantasyPage() -> impl IntoView {
    view! { <FeatureUnavailablePage title="Фэнтези" status="Составы и подсчёт очков не подключены" detail="Для фэнтези необходимы подтверждённые заявки, события матчей, правила начисления очков и история пересчётов."/> }
}

#[component]
fn NewsPage() -> impl IntoView {
    view! { <FeatureUnavailablePage title="Новости" status="Редакционные источники не подключены" detail="Новостная лента, дайджест и видео появятся после подключения источников, дат публикации и атрибуции материалов."/> }
}

#[component]
fn AmateurFootballPage() -> impl IntoView {
    view! { <FeatureUnavailablePage title="Любительский футбол" status="Пространства и права доступа не подключены" detail="Команды, турниры и управление владельцев требуют авторизации, изоляции данных и серверных проверок прав."/> }
}


#[component]
pub fn App() -> impl IntoView {
    let (game, set_game) = signal::<Option<Game>>(None);
    let (active_route, set_active_route) = signal(current_route());

    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window().expect("browser window is available");
        let listener_window = window.clone();
        let listener = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
            set_active_route.set(current_route());
        });
        let callback = listener
            .as_ref()
            .unchecked_ref::<js_sys::Function>()
            .clone();
        let _ = window.add_event_listener_with_callback("hashchange", &callback);
        on_cleanup(move || {
            let _ = listener_window.remove_event_listener_with_callback("hashchange", &callback);
            drop(listener);
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    let _ = set_active_route;

    let (team_one, set_team_one) = signal("Команда 1".to_string());
    let (team_two, set_team_two) = signal("Команда 2".to_string());
    let (round_length, set_round_length) = signal("60".to_string());
    let (category, set_category) = signal("all".to_string());
    let (setup_error, set_setup_error) = signal::<Option<String>>(None);

    let timer = StoredValue::new_local(None::<TimerHandle>);
    let timer_on_cleanup = timer;
    on_cleanup(move || timer_on_cleanup.set_value(None));

    let timer_on_start = timer;
    let on_start = move || {
        let mut cards = bundled_cards();
        let selected_category = category.get();
        if selected_category != "all" {
            cards.retain(|card| card.category == selected_category);
        }

        let settings = GameSettings {
            team_names: [team_one.get(), team_two.get()],
            seconds_per_round: round_length.get().parse().unwrap_or(60),
            rounds: 4,
        };

        match Game::new(cards, settings, random_seed()) {
            Ok(mut new_game) => {
                if let Err(error) = new_game.start() {
                    set_setup_error.set(Some(error.to_string()));
                    return;
                }
                set_setup_error.set(None);
                set_game.set(Some(new_game));
                timer_on_start.set_value(None);
                start_timer(timer_on_start, set_game);
            }
            Err(error) => set_setup_error.set(Some(error.to_string())),
        }
    };

    let timer_on_next = timer;
    let on_next_round = move || {
        set_game.update(|current| {
            if let Some(game) = current {
                let _ = game.next_round();
            }
        });
        timer_on_next.set_value(None);
        if game
            .get()
            .as_ref()
            .is_some_and(|current| current.phase() == GamePhase::Active)
        {
            start_timer(timer_on_next, set_game);
        }
    };

    let timer_on_reset = timer;
    let on_new_game = move || {
        timer_on_reset.set_value(None);
        set_game.set(None);
        set_setup_error.set(None);
    };

    view! {
        <main class="app-shell">
            <header class="topbar">
                <a class="brand" href="/" aria-label="Sherlock">
                    <span class="brand-mark">"S"</span>
                    <span class="brand-name">"SHERLOCK"</span>
                </a>
                <span class="topbar-tag">"FOOTBALL PLATFORM"</span>
            </header>

            <PrimaryNavigation active_route=active_route/>
            <section class="route-home" class:route-hidden=move || active_route.get() != PrimaryRoute::Home>
                <HomePage/>
            </section>
            <section class="main-content" class:mode-hidden=move || active_route.get() != PrimaryRoute::Alias>
                {move || {
                    match game.get() {
                        None => view! {
                            <section class="setup-screen">
                                <div class="eyebrow">
                                    <span class="eyebrow-dot"></span>
                                    "ФУТБОЛЬНАЯ ИГРА НА ОБЪЯСНЕНИЕ"
                                </div>
                                <h1>"Футбольный " <span>"Элиас"</span></h1>
                                <p class="hero-subtitle">
                                    "Две команды. Карточки с футбольными именами. Раунд на время."
                                </p>

                                <div class="setup-panel">
                                    <div class="panel-heading">
                                        <span>"01"</span>
                                        <h2>"Команды"</h2>
                                    </div>
                                    <div class="team-fields">
                                        <label class="field">
                                            <span>"КОМАНДА A"</span>
                                            <input
                                                type="text"
                                                maxlength="24"
                                                prop:value=move || team_one.get()
                                                on:input=move |event| set_team_one.set(event_target_value(&event))
                                            />
                                        </label>
                                        <label class="field">
                                            <span>"КОМАНДА B"</span>
                                            <input
                                                type="text"
                                                maxlength="24"
                                                prop:value=move || team_two.get()
                                                on:input=move |event| set_team_two.set(event_target_value(&event))
                                            />
                                        </label>
                                    </div>

                                    <div class="panel-heading panel-heading-spaced">
                                        <span>"02"</span>
                                        <h2>"Настройки матча"</h2>
                                    </div>
                                    <div class="settings-fields">
                                        <label class="field">
                                            <span>"ВРЕМЯ РАУНДА"</span>
                                            <select
                                                prop:value=move || round_length.get()
                                                on:change=move |event| set_round_length.set(event_target_value(&event))
                                            >
                                                <option value="30">"30 секунд"</option>
                                                <option value="45">"45 секунд"</option>
                                                <option value="60">"60 секунд"</option>
                                                <option value="90">"90 секунд"</option>
                                            </select>
                                        </label>
                                        <label class="field">
                                            <span>"КОЛОДА"</span>
                                            <select
                                                prop:value=move || category.get()
                                                on:change=move |event| set_category.set(event_target_value(&event))
                                            >
                                                <option value="all">"Все категории"</option>
                                                <option value="player">"Футболисты"</option>
                                                <option value="club">"Клубы"</option>
                                                <option value="coach">"Тренеры"</option>
                                                <option value="stadium">"Стадионы"</option>
                                                <option value="term">"Футбольные термины"</option>
                                                <option value="commentator">"Комментаторы"</option>
                                                <option value="referee">"Судьи"</option>
                                                <option value="woman">"Женский футбол"</option>
                                                <option value="position">"Игровые позиции"</option>
                                                <option value="club_nickname">"Прозвища клубов"</option>
                                            </select>
                                        </label>
                                    </div>
                                    <div class="match-meta">
                                        <div>
                                            <span class="meta-number">"4"</span>
                                            <span class="meta-label">"РАУНДА"</span>
                                        </div>
                                        <div>
                                            <span class="meta-number">"2"</span>
                                            <span class="meta-label">"КОМАНДЫ"</span>
                                        </div>
                                        <div>
                                            <span class="meta-number">"2 145"</span>
                                            <span class="meta-label">"КАРТОЧЕК В КОЛОДЕ"</span>
                                        </div>
                                    </div>

                                    {move || setup_error.get().map(|message| view! {
                                        <p class="error-message" role="alert">{message}</p>
                                    })}

                                    <button class="primary-button start-button" on:click=move |_| on_start()>
                                        <span>"Начать игру"</span>
                                        <span class="button-arrow">"→"</span>
                                    </button>
                                </div>
                                <p class="setup-footnote">
                                    "Локальный матч на одном устройстве. Игровой цикл, таймер и счёт работают без подключения к серверу."
                                </p>
                            </section>
                        }.into_any(),

                        Some(snapshot) => match snapshot.phase() {
                            GamePhase::Active => view! {
                                <section class="game-screen">
                                    <div class="game-topline">
                                        <div>
                                            <span class="eyebrow-dot"></span>
                                            <span class="eyebrow-inline">"МАТЧ В ИГРЕ"</span>
                                        </div>
                                        <span class="round-indicator">
                                            "РАУНД " {move || game.get().map(|g| g.round_number()).unwrap_or(1)}
                                            " / " {move || game.get().map(|g| g.total_rounds()).unwrap_or(4)}
                                        </span>
                                    </div>

                                    <div class="scoreboard">
                                        <div class="score-team" class:active-team=move || game.get().as_ref().is_some_and(|g| g.active_team() == 0)>
                                            <span class="score-team-name">{move || game.get().map(|g| g.team_name(0).to_string()).unwrap_or_default()}</span>
                                            <strong>{move || game.get().map(|g| g.scores()[0]).unwrap_or(0)}</strong>
                                        </div>
                                        <div class="score-divider">"—"</div>
                                        <div class="score-team score-team-right" class:active-team=move || game.get().as_ref().is_some_and(|g| g.active_team() == 1)>
                                            <strong>{move || game.get().map(|g| g.scores()[1]).unwrap_or(0)}</strong>
                                            <span class="score-team-name">{move || game.get().map(|g| g.team_name(1).to_string()).unwrap_or_default()}</span>
                                        </div>
                                    </div>

                                    <div class="turn-banner">
                                        <span class="turn-label">"СЕЙЧАС ОБЪЯСНЯЕТ"</span>
                                        <strong>{move || game.get().map(|g| g.active_team_name().to_string()).unwrap_or_default()}</strong>
                                    </div>

                                    <div class="timer-card">
                                        <div class="timer-caption">"ОСТАЛОСЬ ВРЕМЕНИ"</div>
                                        <div class="timer-value" class:urgent=move || game.get().is_some_and(|g| g.seconds_left() <= 10)>
                                            {move || {
                                                let seconds = game.get().map(|g| g.seconds_left()).unwrap_or(0);
                                                format!("{:02}:{:02}", seconds / 60, seconds % 60)
                                            }}
                                        </div>
                                    </div>

                                    <div class="player-card">
                                        <div class="card-category">
                                            {move || game.get().and_then(|g| g.current_card().map(|c| c.category_ru.clone())).unwrap_or_default()}
                                        </div>
                                        <div class="card-name">
                                            {move || game.get().and_then(|g| g.current_card().map(|c| c.name.clone())).unwrap_or_else(|| "Колода закончилась".into())}
                                        </div>
                                        <div class="card-bottom">
                                            <span>"КАРТОЧКА " {move || {
                                                game.get().map(|g| format!("{} / {}", g.card_number(), g.total_cards())).unwrap_or_default()
                                            }}</span>
                                            <span>"SHERLOCK / ALIAS"</span>
                                        </div>
                                    </div>

                                    <div class="action-grid">
                                        <button class="answer-button correct-button" on:click=move |_| {
                                            set_game.update(|current| {
                                                if let Some(g) = current { let _ = g.correct(); }
                                            });
                                        }>
                                            <span class="action-icon">"✓"</span>
                                            <span>"Угадали"</span>
                                        </button>
                                        <button class="answer-button skip-button" on:click=move |_| {
                                            set_game.update(|current| {
                                                if let Some(g) = current { let _ = g.skip(); }
                                            });
                                        }>
                                            <span class="action-icon">"↷"</span>
                                            <span>"Пропустить"</span>
                                        </button>
                                    </div>

                                    <button class="finish-round-button" on:click=move |_| {
                                        set_game.update(|current| {
                                            if let Some(g) = current { let _ = g.finish_round(); }
                                        });
                                    }>"Завершить раунд"</button>
                                </section>
                            }.into_any(),

                            GamePhase::RoundSummary => view! {
                                <section class="result-screen">
                                    <div class="result-kicker">"РАУНД ЗАВЕРШЁН"</div>
                                    <div class="result-orb">"✓"</div>
                                    <h1>"Итоги раунда"</h1>
                                    <p class="result-team">{move || game.get().map(|g| g.active_team_name().to_string()).unwrap_or_default()}</p>
                                    <div class="round-result-stats">
                                        <div>
                                            <strong>{move || game.get().map(|g| g.round_points()).unwrap_or(0)}</strong>
                                            <span>"ОЧКОВ"</span>
                                        </div>
                                        <div>
                                            <strong>{move || game.get().map(|g| g.round_correct()).unwrap_or(0)}</strong>
                                            <span>"УГАДАНО"</span>
                                        </div>
                                        <div>
                                            <strong>{move || game.get().map(|g| g.round_skipped()).unwrap_or(0)}</strong>
                                            <span>"ПРОПУСКОВ"</span>
                                        </div>
                                    </div>
                                    <div class="final-score">
                                        <div>
                                            <span>{move || game.get().map(|g| g.team_name(0).to_string()).unwrap_or_default()}</span>
                                            <strong>{move || game.get().map(|g| g.scores()[0]).unwrap_or(0)}</strong>
                                        </div>
                                        <div>
                                            <span>{move || game.get().map(|g| g.team_name(1).to_string()).unwrap_or_default()}</span>
                                            <strong>{move || game.get().map(|g| g.scores()[1]).unwrap_or(0)}</strong>
                                        </div>
                                    </div>
                                    <button class="primary-button" on:click=move |_| on_next_round()>
                                        <span>"Продолжить"</span>
                                        <span class="button-arrow">"→"</span>
                                    </button>
                                </section>
                            }.into_any(),

                            GamePhase::Finished => view! {
                                <section class="result-screen final-screen">
                                    <div class="result-kicker">"МАТЧ ЗАВЕРШЁН"</div>
                                    <div class="result-orb trophy">"★"</div>
                                    <h1>"Финальный счёт"</h1>
                                    <p class="result-team winner-name">
                                        {move || game.get().map(|g| match g.winner() {
                                            Some(winner) => format!("Победила команда {}", g.team_name(winner)),
                                            None => "Ничья".to_string(),
                                        }).unwrap_or_default()}
                                    </p>
                                    <div class="final-score final-score-large">
                                        <div>
                                            <span>{move || game.get().map(|g| g.team_name(0).to_string()).unwrap_or_default()}</span>
                                            <strong>{move || game.get().map(|g| g.scores()[0]).unwrap_or(0)}</strong>
                                        </div>
                                        <div>
                                            <span>{move || game.get().map(|g| g.team_name(1).to_string()).unwrap_or_default()}</span>
                                            <strong>{move || game.get().map(|g| g.scores()[1]).unwrap_or(0)}</strong>
                                        </div>
                                    </div>
                                    <button class="primary-button" on:click=move |_| on_new_game()>
                                        <span>"Новая игра"</span>
                                        <span class="button-arrow">"↻"</span>
                                    </button>
                                </section>
                            }.into_any(),

                            GamePhase::Setup => view! { <div class="loading-state">"Подготовка матча…"</div> }.into_any(),
                        },
                    }
                }}
            </section>
            <section class="dynasty-content" class:mode-visible=move || active_route.get() == PrimaryRoute::Dynasty>
                <DynastyApp/>
            </section>
            <section class="route-feature" class:route-hidden=move || !matches!(
                active_route.get(),
                PrimaryRoute::Players | PrimaryRoute::Clubs | PrimaryRoute::Matches |
                PrimaryRoute::Statistics | PrimaryRoute::Forecasts | PrimaryRoute::Fantasy |
                PrimaryRoute::News | PrimaryRoute::AmateurFootball
            )>
                {move || match active_route.get() {
                    PrimaryRoute::Players => view! { <PlayersPage/> }.into_any(),
                    PrimaryRoute::Clubs => view! { <ClubsPage/> }.into_any(),
                    PrimaryRoute::Matches => view! { <MatchesPage/> }.into_any(),
                    PrimaryRoute::Statistics => view! { <StatisticsPage/> }.into_any(),
                    PrimaryRoute::Forecasts => view! { <ForecastsPage/> }.into_any(),
                    PrimaryRoute::Fantasy => view! { <FantasyPage/> }.into_any(),
                    PrimaryRoute::News => view! { <NewsPage/> }.into_any(),
                    PrimaryRoute::AmateurFootball => view! { <AmateurFootballPage/> }.into_any(),
                    _ => {
                        let _: () = view! { <></> };
                        ().into_any()
                    },
                }}
            </section>
            <footer class="footer">
                <span>"SHERLOCK"</span>
                <span>"SHERLOCK FOOTBALL"</span>
            </footer>
        </main>
    }
}


#[cfg(test)]
mod route_tests {
    use super::PrimaryRoute;

    #[test]
    fn every_primary_route_resolves_from_its_hash() {
        let cases = [
            ("#/home", PrimaryRoute::Home),
            ("#/alias", PrimaryRoute::Alias),
            ("#/players", PrimaryRoute::Players),
            ("#/clubs", PrimaryRoute::Clubs),
            ("#/matches", PrimaryRoute::Matches),
            ("#/statistics", PrimaryRoute::Statistics),
            ("#/forecasts", PrimaryRoute::Forecasts),
            ("#/fantasy", PrimaryRoute::Fantasy),
            ("#/news", PrimaryRoute::News),
            ("#/amateur", PrimaryRoute::AmateurFootball),
            ("#/dynasty", PrimaryRoute::Dynasty),
        ];
        for (hash, expected) in cases {
            assert_eq!(PrimaryRoute::from_hash(hash), expected, "{hash}");
        }
    }

    #[test]
    fn unknown_or_empty_hash_falls_back_to_home() {
        assert_eq!(PrimaryRoute::from_hash(""), PrimaryRoute::Home);
        assert_eq!(PrimaryRoute::from_hash("#/unknown"), PrimaryRoute::Home);
        assert_eq!(PrimaryRoute::from_hash("#/players/unknown"), PrimaryRoute::Home);
    }

    #[test]
    fn route_query_parameters_do_not_change_route_identity() {
        assert_eq!(PrimaryRoute::from_hash("#/players?sort=name"), PrimaryRoute::Players);
        assert_eq!(PrimaryRoute::from_hash("#/dynasty?tab=club"), PrimaryRoute::Dynasty);
    }
}


#[cfg(target_arch = "wasm32")]
pub fn mount_app() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

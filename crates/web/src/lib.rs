#[cfg(target_arch = "wasm32")]
use gloo_timers::callback::Interval;
use leptos::prelude::*;
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

#[component]
pub fn App() -> impl IntoView {
    let (game, set_game) = signal::<Option<Game>>(None);
    let (dynasty_mode, set_dynasty_mode) = signal(false);
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
        <main class="app-shell" class:dynasty-active=move || dynasty_mode.get()>
            <header class="topbar">
                <a class="brand" href="/" aria-label="Sherlock">
                    <span class="brand-mark">"S"</span>
                    <span class="brand-name">"SHERLOCK"</span>
                </a>
                <button type="button" class="mode-switch" on:click=move |_| set_dynasty_mode.update(|active| *active = !*active)>{move || if dynasty_mode.get() { "Футбольный Элиас" } else { "Футбольная династия" }}</button>
            </header>

            <section class="main-content" class:mode-hidden=move || dynasty_mode.get()>
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
            <section class="dynasty-content" class:mode-visible=move || dynasty_mode.get()>
                <DynastyApp/>
            </section>
            <footer class="footer">
                <span>"SHERLOCK"</span>
                <span>"SHERLOCK FOOTBALL"</span>
            </footer>
        </main>
    }
}

#[cfg(target_arch = "wasm32")]
pub fn mount_app() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

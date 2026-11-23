use leptos::prelude::*;
use sherlock_dynasty_core::{Campaign, Formation, Position, TrainingFocus};

const SAVE_KEY: &str = "sherlock.football-dynasty.v1";

#[cfg(target_arch = "wasm32")]
fn load_campaign() -> Option<Campaign> {
    let value = web_sys::window()?
        .local_storage()
        .ok()??
        .get_item(SAVE_KEY)
        .ok()??;
    serde_json::from_str::<Campaign>(&value)
        .ok()
        .filter(|campaign| campaign.validate().is_ok())
}

#[cfg(not(target_arch = "wasm32"))]
fn load_campaign() -> Option<Campaign> {
    None
}

#[cfg(target_arch = "wasm32")]
fn save_campaign(campaign: &Campaign) -> Result<(), String> {
    let value = serde_json::to_string(campaign).map_err(|_| "Не удалось подготовить сохранение".to_string())?;
    web_sys::window()
        .and_then(|window| window.local_storage().ok().flatten())
        .ok_or_else(|| "Хранилище браузера недоступно".to_string())?
        .set_item(SAVE_KEY, &value)
        .map_err(|_| "Браузер не сохранил карьеру".to_string())
}

#[cfg(not(target_arch = "wasm32"))]
fn save_campaign(_campaign: &Campaign) -> Result<(), String> {
    Ok(())
}

fn save_and_set(
    campaign: Campaign,
    set_campaign: WriteSignal<Option<Campaign>>,
    set_notice: WriteSignal<Option<String>>,
) {
    let persistence_error = save_campaign(&campaign).err();
    set_campaign.set(Some(campaign));
    set_notice.set(persistence_error);
}

#[cfg(target_arch = "wasm32")]
fn random_seed() -> u64 {
    (js_sys::Date::now() as u64) ^ ((js_sys::Math::random() * u64::MAX as f64) as u64)
}

#[cfg(not(target_arch = "wasm32"))]
fn random_seed() -> u64 {
    20261123
}

fn money(value: i64) -> String {
    let sign = if value < 0 { "−" } else { "" };
    format!("{sign}{} €", value.unsigned_abs())
}

fn position_order(position: Position) -> u8 {
    match position {
        Position::Goalkeeper => 0,
        Position::Defender => 1,
        Position::Midfielder => 2,
        Position::Forward => 3,
    }
}

fn next_training_focus(value: &str) -> TrainingFocus {
    match value {
        "attack" => TrainingFocus::Attack,
        "defence" => TrainingFocus::Defence,
        "fitness" => TrainingFocus::Fitness,
        _ => TrainingFocus::Balanced,
    }
}

#[component]
pub fn DynastyApp() -> impl IntoView {
    let (campaign, set_campaign) = signal(load_campaign());
    let (manager_name, set_manager_name) = signal("Александр".to_string());
    let (club_name, set_club_name) = signal("ФК Север".to_string());
    let (heir_name, set_heir_name) = signal("Михаил".to_string());
    let (heir_age, set_heir_age) = signal("18".to_string());
    let (focus, set_focus) = signal("balanced".to_string());
    let (notice, set_notice) = signal::<Option<String>>(None);
    let (tab, set_tab) = signal("club".to_string());

    let create_campaign = move |_| {
        match Campaign::new(&manager_name.get(), &club_name.get(), random_seed()) {
            Ok(mut created) => {
                let heir = heir_name.get();
                if !heir.trim().is_empty() {
                    let age = heir_age.get().parse::<u8>().unwrap_or(18).min(100);
                    if let Err(error) = created.add_heir(&heir, age) {
                        set_notice.set(Some(error.to_string()));
                    }
                }
                save_and_set(created, set_campaign, set_notice);
            }
            Err(error) => set_notice.set(Some(error.to_string())),
        }
    };

    let next_matchday = move |_| {
        if let Some(mut updated) = campaign.get() {
            match updated.advance_matchday() {
                Ok(results) => {
                    let text = results.iter().map(|(fixture, score)| {
                        format!("{fixture}: {}:{}", score.home, score.away)
                    }).collect::<Vec<_>>().join(" · ");
                    set_notice.set(Some(text));
                    save_and_set(updated, set_campaign, set_notice);
                }
                Err(error) => set_notice.set(Some(error.to_string())),
            }
        }
    };

    let train_team = move |_| {
        if let Some(mut updated) = campaign.get() {
            match updated.train(next_training_focus(&focus.get())) {
                Ok(improved) => {
                    set_notice.set(Some(format!("Тренировка завершена. Игроков с ростом общего рейтинга: {improved}.")));
                    save_and_set(updated, set_campaign, set_notice);
                }
                Err(error) => set_notice.set(Some(error.to_string())),
            }
        }
    };

    let upgrade_facility = move |_| {
        if let Some(mut updated) = campaign.get() {
            match updated.upgrade_training_facility() {
                Ok(()) => save_and_set(updated, set_campaign, set_notice),
                Err(error) => set_notice.set(Some(error.to_string())),
            }
        }
    };

    let add_heir = move |_| {
        if let Some(mut updated) = campaign.get() {
            let age = heir_age.get().parse::<u8>().unwrap_or(0);
            match updated.add_heir(&heir_name.get(), age) {
                Ok(()) => save_and_set(updated, set_campaign, set_notice),
                Err(error) => set_notice.set(Some(error.to_string())),
            }
        }
    };

    let reset_campaign = move |_| {
        #[cfg(target_arch = "wasm32")]
        if let Some(storage) = web_sys::window().and_then(|window| window.local_storage().ok().flatten()) {
            let _ = storage.remove_item(SAVE_KEY);
        }
        set_campaign.set(None);
        set_notice.set(Some("Локальное сохранение удалено.".into()));
    };

    view! {
        <div class="dynasty-shell">
            <div class="dynasty-heading">
                <div>
                    <div class="dynasty-kicker">"SHERLOCK / CAREER MODE"</div>
                    <h1>"Футбольная " <span>"династия"</span></h1>
                    <p>"Создайте клуб, управляйте составом и проведите свою семью через поколения."</p>
                </div>
                {move || campaign.get().map(|save| view! {
                    <div class="dynasty-season-badge">
                        <span>"СЕЗОН"</span>
                        <strong>{save.season}</strong>
                    </div>
                })}
            </div>

            {move || notice.get().map(|message| view! {
                <div class="dynasty-notice" role="status">
                    <span>{message}</span>
                    <button type="button" on:click=move |_| set_notice.set(None) aria-label="Закрыть">"×"</button>
                </div>
            })}

            {move || match campaign.get() {
                None => view! {
                    <section class="dynasty-setup">
                        <div class="dynasty-panel-title">
                            <span>"01"</span>
                            <div>
                                <h2>"Основание клуба"</h2>
                                <p>"Новая вымышленная футбольная вселенная"</p>
                            </div>
                        </div>
                        <div class="dynasty-form-grid">
                            <label class="dynasty-field">
                                <span>"ИМЯ РУКОВОДИТЕЛЯ"</span>
                                <input type="text" maxlength="32" prop:value=move || manager_name.get()
                                    on:input=move |event| set_manager_name.set(event_target_value(&event))/>
                            </label>
                            <label class="dynasty-field">
                                <span>"НАЗВАНИЕ КЛУБА"</span>
                                <input type="text" maxlength="32" prop:value=move || club_name.get()
                                    on:input=move |event| set_club_name.set(event_target_value(&event))/>
                            </label>
                            <label class="dynasty-field">
                                <span>"ИМЯ НАСЛЕДНИКА"</span>
                                <input type="text" maxlength="32" prop:value=move || heir_name.get()
                                    on:input=move |event| set_heir_name.set(event_target_value(&event))/>
                            </label>
                            <label class="dynasty-field">
                                <span>"ВОЗРАСТ НАСЛЕДНИКА"</span>
                                <input type="number" min="0" max="100" prop:value=move || heir_age.get()
                                    on:input=move |event| set_heir_age.set(event_target_value(&event))/>
                            </label>
                        </div>
                        <div class="dynasty-start-summary">
                            <div><strong>"22"</strong><span>"ИГРОКА В СОСТАВЕ"</span></div>
                            <div><strong>"4"</strong><span>"КЛУБА В ЛИГЕ"</span></div>
                            <div><strong>"6"</strong><span>"ТУРОВ В СЕЗОНЕ"</span></div>
                            <div><strong>"2,5 млн €"</strong><span>"СТАРТОВЫЙ БЮДЖЕТ"</span></div>
                        </div>
                        <button type="button" class="dynasty-primary" on:click=create_campaign>
                            "Создать династию" <span>"→"</span>
                        </button>
                        <p class="dynasty-footnote">"Составы, соперники и матчи в этом режиме вымышлены. Карьера сохраняется в локальном хранилище этого браузера."</p>
                    </section>
                }.into_any(),
                Some(active) => {
                    let standings = active.standings();
                    let squad_count = active.club.players.len();
                    let starter_count = active.club.starting_xi.len();
                    let next_fixtures = active.fixtures.iter().filter(|fixture| fixture.matchday == active.matchday).cloned().collect::<Vec<_>>();
                    let recent_events = active.chronicle.iter().rev().take(6).cloned().collect::<Vec<_>>();
                    let recent_ledger = active.ledger.iter().rev().take(5).cloned().collect::<Vec<_>>();
                    view! {
                        <div class="dynasty-dashboard">
                            <div class="dynasty-overview-grid">
                                <div class="dynasty-stat-card">
                                    <span>"БЮДЖЕТ КЛУБА"</span>
                                    <strong>{money(active.club.cash)}</strong>
                                    <small>{format!("{} игроков · {} в основе", squad_count, starter_count)}</small>
                                </div>
                                <div class="dynasty-stat-card">
                                    <span>"РЕПУТАЦИЯ"</span>
                                    <strong>{format!("{}/100", active.club.reputation)}</strong>
                                    <small>"Результаты и решения клуба"</small>
                                </div>
                                <div class="dynasty-stat-card">
                                    <span>"ИГРОВОЕ ВРЕМЯ"</span>
                                    <strong>{format!("День {}", active.game_day)}</strong>
                                    <small>{format!("Сезон {} · тур {}", active.season, active.matchday)}</small>
                                </div>
                                <div class="dynasty-stat-card">
                                    <span>"РУКОВОДИТЕЛЬ"</span>
                                    <strong>{active.manager_name.clone()}</strong>
                                    <small>{format!("{} лет · база уровня {}", active.manager_age, active.club.training_facility)}</small>
                                </div>
                            </div>

                            <nav class="dynasty-tabs" aria-label="Разделы карьеры">
                                <button type="button" class:active=move || tab.get() == "club" on:click=move |_| set_tab.set("club".into())>"Клуб"</button>
                                <button type="button" class:active=move || tab.get() == "squad" on:click=move |_| set_tab.set("squad".into())>"Состав"</button>
                                <button type="button" class:active=move || tab.get() == "league" on:click=move |_| set_tab.set("league".into())>"Чемпионат"</button>
                                <button type="button" class:active=move || tab.get() == "transfers" on:click=move |_| set_tab.set("transfers".into())>"Трансферы"</button>
                                <button type="button" class:active=move || tab.get() == "family" on:click=move |_| set_tab.set("family".into())>"Династия"</button>
                            </nav>

                            {move || match tab.get().as_str() {
                                "squad" => view! {
                                    <section class="dynasty-panel">
                                        <div class="dynasty-section-heading">
                                            <div><span>"КОМАНДА"</span><h2>"Состав и тактика"</h2></div>
                                            <label class="dynasty-inline-field">
                                                <span>"СХЕМА"</span>
                                                <select prop:value=move || campaign.get().map(|c| match c.club.tactics.formation {
                                                    Formation::FourThreeThree => "433",
                                                    Formation::FourFourTwo => "442",
                                                    Formation::ThreeFiveTwo => "352",
                                                }.to_string()).unwrap_or_else(|| "433".into())
                                                    on:change=move |event| {
                                                        if let Some(mut updated) = campaign.get() {
                                                            let formation = match event_target_value(&event).as_str() {
                                                                "442" => Formation::FourFourTwo,
                                                                "352" => Formation::ThreeFiveTwo,
                                                                _ => Formation::FourThreeThree,
                                                            };
                                                            updated.set_formation(formation);
                                                            save_and_set(updated, set_campaign, set_notice);
                                                        }
                                                    }>
                                                    <option value="433">"4–3–3"</option>
                                                    <option value="442">"4–4–2"</option>
                                                    <option value="352">"3–5–2"</option>
                                                </select>
                                            </label>
                                        </div>
                                        <p class="dynasty-hint">"В стартовом составе 11 игроков. Выберите игрока из резерва, чтобы заменить футболиста на той же позиции."</p>
                                        <div class="dynasty-roster">
                                            {move || campaign.get().map(|save| {
                                                let mut players = save.club.players.clone();
                                                players.sort_by_key(|player| (position_order(player.position), !save.club.starting_xi.contains(&player.id), player.name.clone()));
                                                players.into_iter().map(|player| {
                                                    let player_id = player.id;
                                                    let starter = save.club.starting_xi.contains(&player_id);
                                                    let unavailable = player.injury_days > 0 || player.suspension_matches > 0;
                                                    view! {
                                                        <article class="dynasty-player-row">
                                                            <div class="dynasty-player-rating">{player.overall()}</div>
                                                            <div class="dynasty-player-main">
                                                                <strong>{player.name.clone()}</strong>
                                                                <span>{format!("{} · {} лет · потенциал {}", player.position.label(), player.age, player.potential)}</span>
                                                            </div>
                                                            <div class="dynasty-player-career">
                                                                <strong>{format!("{} гол.", player.goals)}</strong>
                                                                <span>{format!("{} матчей", player.appearances)}</span>
                                                            </div>
                                                            <button type="button" class="dynasty-mini-button"
                                                                disabled=starter || unavailable
                                                                on:click=move |_| {
                                                                    if let Some(mut updated) = campaign.get() {
                                                                        match updated.select_player(player_id) {
                                                                            Ok(()) => save_and_set(updated, set_campaign, set_notice),
                                                                            Err(error) => set_notice.set(Some(error.to_string())),
                                                                        }
                                                                    }
                                                                }>
                                                                {if unavailable { "Недоступен" } else if starter { "В основе" } else { "В состав" }}
                                                            </button>
                                                        </article>
                                                    }
                                                }).collect_view()
                                            })}
                                        </div>
                                    </section>
                                }.into_any(),
                                "league" => view! {
                                    <div class="dynasty-two-col">
                                        <section class="dynasty-panel">
                                            <div class="dynasty-section-heading"><div><span>"ТУРНИР"</span><h2>"Таблица лиги"</h2></div></div>
                                            <div class="dynasty-table-wrap">
                                                <table class="dynasty-table">
                                                    <thead><tr><th>"#"</th><th>"Клуб"</th><th>"И"</th><th>"В"</th><th>"Н"</th><th>"П"</th><th>"М"</th><th>"О"</th></tr></thead>
                                                    <tbody>
                                                        {standings.iter().enumerate().map(|(index, row)| view! {
                                                            <tr class:dynasty-own-row=move || row.team == active.club.name>
                                                                <td>{index + 1}</td><td>{row.team.clone()}</td><td>{row.played}</td><td>{row.wins}</td><td>{row.draws}</td><td>{row.losses}</td><td>{format!("{}:{}", row.goals_for, row.goals_against)}</td><td><strong>{row.points}</strong></td>
                                                            </tr>
                                                        }).collect_view()}
                                                    </tbody>
                                                </table>
                                            </div>
                                        </section>
                                        <section class="dynasty-panel">
                                            <div class="dynasty-section-heading"><div><span>{format!("СЕЗОН {}", active.season)}</span><h2>{format!("Тур {}", active.matchday)}</h2></div></div>
                                            <div class="dynasty-fixtures">
                                                {next_fixtures.iter().map(|fixture| {
                                                    let home = if fixture.home_team == 0 { active.club.name.clone() } else { active.opponents[fixture.home_team - 1].clone() };
                                                    let away = if fixture.away_team == 0 { active.club.name.clone() } else { active.opponents[fixture.away_team - 1].clone() };
                                                    view! {
                                                        <div class="dynasty-fixture">
                                                            <span>{home}</span>
                                                            <strong>{fixture.score.map(|score| format!("{} : {}", score.home, score.away)).unwrap_or_else(|| "— : —".into())}</strong>
                                                            <span>{away}</span>
                                                        </div>
                                                    }
                                                }).collect_view()}
                                            </div>
                                            <button type="button" class="dynasty-primary" on:click=next_matchday>"Симулировать тур" <span>"→"</span></button>
                                        </section>
                                    </div>
                                }.into_any(),
                                "transfers" => view! {
                                    <div class="dynasty-two-col">
                                        <section class="dynasty-panel">
                                            <div class="dynasty-section-heading"><div><span>"СКАУТИНГ"</span><h2>"Рынок игроков"</h2></div></div>
                                            <p class="dynasty-hint">"Предложения сгенерированы для этой вымышленной карьеры. Цена списывается из бюджета клуба."</p>
                                            <div class="dynasty-roster">
                                                {move || campaign.get().map(|save| save.transfer_market.into_iter().map(|offer| {
                                                    let id = offer.player.id;
                                                    view! {
                                                        <article class="dynasty-player-row">
                                                            <div class="dynasty-player-rating">{offer.player.overall()}</div>
                                                            <div class="dynasty-player-main"><strong>{offer.player.name.clone()}</strong><span>{format!("{} · {} лет", offer.player.position.label(), offer.player.age)}</span></div>
                                                            <div class="dynasty-player-career"><strong>{money(i64::from(offer.asking_price))}</strong><span>{format!("Зарплата {} €", offer.player.wage)}</span></div>
                                                            <button type="button" class="dynasty-mini-button" on:click=move |_| {
                                                                if let Some(mut updated) = campaign.get() {
                                                                    match updated.sign_player(id) {
                                                                        Ok(()) => save_and_set(updated, set_campaign, set_notice),
                                                                        Err(error) => set_notice.set(Some(error.to_string())),
                                                                    }
                                                                }
                                                            }>"Подписать"</button>
                                                        </article>
                                                    }
                                                }).collect_view())}
                                            </div>
                                        </section>
                                        <section class="dynasty-panel">
                                            <div class="dynasty-section-heading"><div><span>"ФИНАНСЫ"</span><h2>"Движение средств"</h2></div></div>
                                            <div class="dynasty-ledger">
                                                {recent_ledger.iter().map(|entry| view! {
                                                    <div class="dynasty-ledger-row">
                                                        <span>{entry.description.clone()}</span>
                                                        <strong class:negative=entry.delta < 0>{money(entry.delta)}</strong>
                                                    </div>
                                                }).collect_view()}
                                            </div>
                                        </section>
                                    </div>
                                }.into_any(),
                                "family" => view! {
                                    <div class="dynasty-two-col">
                                        <section class="dynasty-panel">
                                            <div class="dynasty-section-heading"><div><span>"СЕМЬЯ"</span><h2>"Наследование клуба"</h2></div></div>
                                            <div class="dynasty-family-card">
                                                <div><span>"ТЕКУЩЕЕ ПОКОЛЕНИЕ"</span><strong>{active.manager_name.clone()}</strong><small>{format!("{} лет", active.manager_age)}</small></div>
                                                <div class="dynasty-family-mark">"I"</div>
                                            </div>
                                            <div class="dynasty-heirs">
                                                {active.heirs.iter().map(|heir| {
                                                    let heir_id = heir.id;
                                                    view! {
                                                        <div class="dynasty-heir-row">
                                                            <div><strong>{heir.name.clone()}</strong><span>{format!("{} лет", heir.age)}</span></div>
                                                            <button type="button" class="dynasty-mini-button" disabled=heir.age < 18
                                                                on:click=move |_| {
                                                                    if let Some(mut updated) = campaign.get() {
                                                                        match updated.retire_to_heir(heir_id) {
                                                                            Ok(()) => save_and_set(updated, set_campaign, set_notice),
                                                                            Err(error) => set_notice.set(Some(error.to_string())),
                                                                        }
                                                                    }
                                                                }>"Передать управление"</button>
                                                        </div>
                                                    }
                                                }).collect_view()}
                                            </div>
                                            <div class="dynasty-form-grid dynasty-heir-form">
                                                <label class="dynasty-field"><span>"ИМЯ НАСЛЕДНИКА"</span><input type="text" maxlength="32" prop:value=move || heir_name.get() on:input=move |event| set_heir_name.set(event_target_value(&event))/></label>
                                                <label class="dynasty-field"><span>"ВОЗРАСТ"</span><input type="number" min="0" max="100" prop:value=move || heir_age.get() on:input=move |event| set_heir_age.set(event_target_value(&event))/></label>
                                            </div>
                                            <button type="button" class="dynasty-secondary" on:click=add_heir>"Добавить наследника"</button>
                                            <p class="dynasty-hint">"Передача управления сохраняет клуб, состав, финансы и историю. Для перехода нужен наследник старше 18 лет."</p>
                                        </section>
                                        <section class="dynasty-panel">
                                            <div class="dynasty-section-heading"><div><span>"ХРОНИКА"</span><h2>"История семьи"</h2></div></div>
                                            <div class="dynasty-events">
                                                {recent_events.iter().map(|entry| view! {
                                                    <article><span>{format!("ДЕНЬ {}", entry.game_day)}</span><strong>{entry.title.clone()}</strong><p>{entry.detail.clone()}</p></article>
                                                }).collect_view()}
                                            </div>
                                        </section>
                                    </div>
                                }.into_any(),
                                _ => view! {
                                    <div class="dynasty-two-col">
                                        <div class="dynasty-column">
                                            <section class="dynasty-panel">
                                                <div class="dynasty-section-heading"><div><span>"СЛЕДУЮЩИЙ МАТЧДЕНЬ"</span><h2>{format!("Тур {} · сезон {}", active.matchday, active.season)}</h2></div></div>
                                                <div class="dynasty-fixtures">
                                                    {next_fixtures.iter().map(|fixture| {
                                                        let home = if fixture.home_team == 0 { active.club.name.clone() } else { active.opponents[fixture.home_team - 1].clone() };
                                                        let away = if fixture.away_team == 0 { active.club.name.clone() } else { active.opponents[fixture.away_team - 1].clone() };
                                                        view! {
                                                            <div class="dynasty-fixture"><span>{home}</span><strong>"— : —"</strong><span>{away}</span></div>
                                                        }
                                                    }).collect_view()}
                                                </div>
                                                <button type="button" class="dynasty-primary" on:click=next_matchday>"Провести тур" <span>"→"</span></button>
                                            </section>
                                            <section class="dynasty-panel">
                                                <div class="dynasty-section-heading"><div><span>"РАЗВИТИЕ"</span><h2>"Тренировочный процесс"</h2></div></div>
                                                <label class="dynasty-field">
                                                    <span>"ПРИОРИТЕТ ТРЕНИРОВОК"</span>
                                                    <select prop:value=move || focus.get() on:change=move |event| set_focus.set(event_target_value(&event))>
                                                        <option value="balanced">"Баланс"</option>
                                                        <option value="attack">"Атака"</option>
                                                        <option value="defence">"Оборона"</option>
                                                        <option value="fitness">"Физическая подготовка"</option>
                                                    </select>
                                                </label>
                                                <button type="button" class="dynasty-secondary" on:click=train_team>"Провести тренировку · 8 000 €"</button>
                                                <div class="dynasty-upgrade-row">
                                                    <span>{format!("Тренировочная база · уровень {}", active.club.training_facility)}</span>
                                                    <button type="button" class="dynasty-mini-button" disabled=active.club.training_facility >= 5 on:click=upgrade_facility>"Улучшить"</button>
                                                </div>
                                            </section>
                                        </div>
                                        <div class="dynasty-column">
                                            <section class="dynasty-panel">
                                                <div class="dynasty-section-heading"><div><span>"ТУРНИР"</span><h2>"Турнирная таблица"</h2></div></div>
                                                <div class="dynasty-table-wrap">
                                                    <table class="dynasty-table">
                                                        <thead><tr><th>"#"</th><th>"Клуб"</th><th>"И"</th><th>"М"</th><th>"О"</th></tr></thead>
                                                        <tbody>{standings.iter().enumerate().map(|(index, row)| view! {
                                                            <tr class:dynasty-own-row=move || row.team == active.club.name>
                                                                <td>{index + 1}</td><td>{row.team.clone()}</td><td>{row.played}</td><td>{format!("{}:{}", row.goals_for, row.goals_against)}</td><td><strong>{row.points}</strong></td>
                                                            </tr>
                                                        }).collect_view()}</tbody>
                                                    </table>
                                                </div>
                                            </section>
                                            <section class="dynasty-panel">
                                                <div class="dynasty-section-heading"><div><span>"ФИНАНСЫ"</span><h2>"Последние операции"</h2></div></div>
                                                <div class="dynasty-ledger">
                                                    {recent_ledger.iter().map(|entry| view! {
                                                        <div class="dynasty-ledger-row"><span>{entry.description.clone()}</span><strong class:negative=entry.delta < 0>{money(entry.delta)}</strong></div>
                                                    }).collect_view()}
                                                </div>
                                            </section>
                                        </div>
                                    </div>
                                }.into_any(),
                            }}
                        </div>
                    }.into_any()
                }
            }}
            <div class="dynasty-bottom-actions">
                <span>"Сохранение автоматически обновляется в этом браузере."</span>
                <button type="button" on:click=reset_campaign>"Удалить карьеру"</button>
            </div>
        </div>
    }
}

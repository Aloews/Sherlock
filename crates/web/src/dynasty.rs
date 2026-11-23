use chrono::{Datelike, NaiveDate};
use leptos::prelude::*;
use sherlock_dynasty_core::{
    CLUB_CASH_ACCOUNT, Campaign, CampaignMode, CampaignSetup, CampaignStatus, CharacterStatus,
    EventStatus, Origin,
};

#[cfg(target_arch = "wasm32")]
const STORAGE_KEY: &str = "sherlock.football-dynasty.v1";

fn load_campaign() -> Result<Option<Campaign>, String> {
    #[cfg(target_arch = "wasm32")]
    {
        let window =
            web_sys::window().ok_or_else(|| "Браузерное хранилище недоступно".to_owned())?;
        let storage = window
            .local_storage()
            .map_err(|_| "Не удалось открыть локальное хранилище".to_owned())?
            .ok_or_else(|| "Локальное хранилище отключено".to_owned())?;
        let Some(raw) = storage
            .get_item(STORAGE_KEY)
            .map_err(|_| "Не удалось прочитать сохранённую кампанию".to_owned())?
        else {
            return Ok(None);
        };
        let campaign = serde_json::from_str(&raw)
            .map_err(|_| "Сохранение повреждено или несовместимо с этой версией игры".to_owned())?;
        Ok(Some(campaign))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Ok(None)
    }
}

fn save_campaign(campaign: &Campaign) -> Result<(), String> {
    #[cfg(target_arch = "wasm32")]
    {
        let window =
            web_sys::window().ok_or_else(|| "Браузерное хранилище недоступно".to_owned())?;
        let storage = window
            .local_storage()
            .map_err(|_| "Не удалось открыть локальное хранилище".to_owned())?
            .ok_or_else(|| "Локальное хранилище отключено".to_owned())?;
        let encoded = serde_json::to_string(campaign)
            .map_err(|_| "Не удалось сериализовать кампанию".to_owned())?;
        storage
            .set_item(STORAGE_KEY, &encoded)
            .map_err(|_| "Не удалось сохранить кампанию в браузере".to_owned())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = campaign;
        Ok(())
    }
}

fn clear_saved_campaign() {
    #[cfg(target_arch = "wasm32")]
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        let _ = storage.remove_item(STORAGE_KEY);
    }
}

fn campaign_seed() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        return (js_sys::Date::now() as u64) ^ ((js_sys::Math::random() * u64::MAX as f64) as u64);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        20_261_123
    }
}

fn apply_campaign(
    set_campaign: WriteSignal<Option<Campaign>>,
    set_feedback: WriteSignal<String>,
    action: impl FnOnce(&mut Campaign) -> Result<(), sherlock_dynasty_core::DynastyError> + 'static,
) {
    let mut outcome = None;
    set_campaign.update(|current| {
        if let Some(campaign) = current {
            outcome = Some(match action(campaign) {
                Ok(()) => save_campaign(campaign),
                Err(error) => Err(error.to_string()),
            });
        }
    });
    match outcome {
        Some(Ok(())) => set_feedback.set("Кампания сохранена в этом браузере".to_owned()),
        Some(Err(error)) => set_feedback.set(error),
        None => set_feedback.set("Сначала создайте кампанию".to_owned()),
    }
}

fn date_label(date: NaiveDate) -> String {
    date.format("%d.%m.%Y").to_string()
}

fn euros_millions(amount_minor: i64) -> String {
    let euros = amount_minor / 100;
    if euros >= 1_000_000 {
        format!("€{} млн", euros / 1_000_000)
    } else if euros >= 1_000 {
        format!("€{} тыс.", euros / 1_000)
    } else {
        format!("€{euros}")
    }
}

fn status_label(status: CampaignStatus) -> &'static str {
    match status {
        CampaignStatus::Active => "Активна",
        CampaignStatus::SuccessionPending => "Ожидает наследования",
        CampaignStatus::EndedNoHeir => "Нет подходящего наследника",
    }
}

#[component]
pub fn DynastyScreen() -> impl IntoView {
    let initial = load_campaign();
    let initial_feedback = match &initial {
        Ok(Some(_)) => "Загружена сохранённая кампания".to_owned(),
        Ok(None) => "Новая кампания".to_owned(),
        Err(error) => error.clone(),
    };
    let (campaign, set_campaign) = signal(initial.ok().flatten());
    let (feedback, set_feedback) = signal(initial_feedback);
    let (campaign_name, set_campaign_name) = signal("Новая футбольная династия".to_owned());
    let (founder_name, set_founder_name) = signal("Александр Орлов".to_owned());
    let (dynasty_name, set_dynasty_name) = signal("Дом Орловых".to_owned());
    let (club_name, set_club_name) = signal("Northbridge FC".to_owned());
    let (origin, set_origin) = signal("entrepreneur".to_owned());
    let (active_tab, set_active_tab) = signal("overview".to_owned());
    let (heir_name, set_heir_name) = signal(String::new());
    let (heir_age, set_heir_age) = signal("18".to_owned());

    let on_create = move |_| {
        let selected_origin = match origin.get().as_str() {
            "football_family" => Origin::FootballFamily,
            "former_player" => Origin::FormerPlayer,
            "agent" => Origin::Agent,
            "executive" => Origin::Executive,
            "newcomer" => Origin::Newcomer,
            _ => Origin::Entrepreneur,
        };
        let setup = CampaignSetup {
            name: campaign_name.get(),
            mode: CampaignMode::Sandbox,
            founder_name: founder_name.get(),
            dynasty_name: dynasty_name.get(),
            club_name: club_name.get(),
            origin: selected_origin,
        };
        match Campaign::new(setup, campaign_seed()) {
            Ok(new_campaign) => {
                let saved = save_campaign(&new_campaign);
                set_campaign.set(Some(new_campaign));
                set_active_tab.set("overview".to_owned());
                set_feedback.set(match saved {
                    Ok(()) => "Кампания создана и сохранена в этом браузере".to_owned(),
                    Err(error) => format!("Кампания создана, но не сохранена: {error}"),
                });
            }
            Err(error) => set_feedback.set(error.to_string()),
        }
    };

    let on_add_heir = move |_| {
        let name = heir_name.get();
        let age = heir_age.get().parse::<u8>().unwrap_or(255);
        apply_campaign(set_campaign, set_feedback, move |active| {
            active.add_heir(&name, age).map(|_| ())
        });
        set_heir_name.set(String::new());
    };

    let tabs = [
        ("overview", "Обзор"),
        ("club", "Клуб"),
        ("season", "Сезон"),
        ("family", "Семья"),
        ("events", "События"),
        ("chronicle", "Хроника"),
    ];

    view! {
        <div class="dynasty-page">
            {move || match campaign.get() {
                None => view! {
                    <section class="dynasty-create">
                        <div class="dynasty-kicker">"SHERLOCK / FOOTBALL DYNASTY"</div>
                        <h1>"Футбольная " <span>"династия"</span></h1>
                        <p class="dynasty-lead">"Создайте семью, возьмите под контроль клуб и постройте наследие, которое переживёт основателя."</p>
                        <div class="dynasty-create-grid">
                            <label class="dynasty-field">
                                <span>"НАЗВАНИЕ КАМПАНИИ"</span>
                                <input prop:value=move || campaign_name.get() on:input=move |event| set_campaign_name.set(event_target_value(&event)) />
                            </label>
                            <label class="dynasty-field">
                                <span>"ОСНОВАТЕЛЬ"</span>
                                <input prop:value=move || founder_name.get() on:input=move |event| set_founder_name.set(event_target_value(&event)) />
                            </label>
                            <label class="dynasty-field">
                                <span>"ФАМИЛЬНЫЙ ДОМ"</span>
                                <input prop:value=move || dynasty_name.get() on:input=move |event| set_dynasty_name.set(event_target_value(&event)) />
                            </label>
                            <label class="dynasty-field">
                                <span>"ФУТБОЛЬНЫЙ КЛУБ"</span>
                                <input prop:value=move || club_name.get() on:input=move |event| set_club_name.set(event_target_value(&event)) />
                            </label>
                            <label class="dynasty-field">
                                <span>"ПРОИСХОЖДЕНИЕ"</span>
                                <select prop:value=move || origin.get() on:change=move |event| set_origin.set(event_target_value(&event))>
                                    <option value="football_family">"Футбольная семья"</option>
                                    <option value="entrepreneur">"Предприниматель"</option>
                                    <option value="former_player">"Бывший игрок"</option>
                                    <option value="agent">"Футбольный агент"</option>
                                    <option value="executive">"Управленец"</option>
                                    <option value="newcomer">"Новый человек в футболе"</option>
                                </select>
                            </label>
                        </div>
                        <div class="dynasty-create-footer">
                            <div class="dynasty-world-stats">
                                <span><strong>"4"</strong>" клуба"</span>
                                <span><strong>"12"</strong>" матчей сезона"</span>
                                <span><strong>"90"</strong>" лет — возрастной рубеж"</span>
                            </div>
                            <button class="dynasty-primary" on:click=on_create>"Создать кампанию" <span>"→"</span></button>
                        </div>
                        <p class="dynasty-feedback" role="status">{move || feedback.get()}</p>
                    </section>
                }.into_any(),
                Some(snapshot) => view! {
                    <section class="dynasty-dashboard">
                        <div class="dynasty-dashboard-heading">
                            <div>
                                <div class="dynasty-kicker">"FOOTBALL DYNASTY · СЕЗОН " {snapshot.game_date.year()}</div>
                                <h1>{snapshot.dynasty.name.clone()}</h1>
                                <p>{snapshot.name} " · " {snapshot.controlled_club().name.clone()}</p>
                            </div>
                            <div class="dynasty-heading-side">
                                <span class="dynasty-status">{status_label(snapshot.status)}</span>
                                <button class="dynasty-quiet-button" on:click=move |_| {
                                    clear_saved_campaign();
                                    set_campaign.set(None);
                                    set_feedback.set("Кампания удалена из этого браузера".to_owned());
                                }>"Новая кампания"</button>
                            </div>
                        </div>

                        <div class="dynasty-tabs">
                            {tabs.into_iter().map(|(key, label)| {
                                let tab_key = key.to_owned();
                                view! {
                                    <button
                                        class="dynasty-tab"
                                        class:active=move || active_tab.get() == tab_key
                                        on:click=move |_| set_active_tab.set(key.to_owned())
                                    >{label}</button>
                                }
                            }).collect_view()}
                        </div>

                        {move || if active_tab.get() == "overview" {
                            view! {
                                <div class="dynasty-content">
                                    <div class="dynasty-kpi-grid">
                                        <article class="dynasty-kpi">
                                            <span>"ИГРОВАЯ ДАТА"</span>
                                            <strong>{move || campaign.get().map(|c| date_label(c.game_date)).unwrap_or_default()}</strong>
                                            <small>"Календарь кампании"</small>
                                        </article>
                                        <article class="dynasty-kpi">
                                            <span>"ПРЕСТИЖ ДИНАСТИИ"</span>
                                            <strong>{move || campaign.get().map(|c| c.dynasty.prestige.to_string()).unwrap_or_default()}"/100"</strong>
                                            <small>"Репутация фамильного дома"</small>
                                        </article>
                                        <article class="dynasty-kpi">
                                            <span>"РЕПУТАЦИЯ КЛУБА"</span>
                                            <strong>{move || campaign.get().map(|c| c.controlled_club().reputation.to_string()).unwrap_or_default()}"/100"</strong>
                                            <small>"Положение в футбольном мире"</small>
                                        </article>
                                        <article class="dynasty-kpi">
                                            <span>"КАЗНА КЛУБА"</span>
                                            <strong>{move || campaign.get().map(|c| euros_millions(c.cash_balance(CLUB_CASH_ACCOUNT))).unwrap_or_default()}</strong>
                                            <small>"Баланс по финансовому журналу"</small>
                                        </article>
                                    </div>
                                    <div class="dynasty-panel">
                                        <div class="dynasty-panel-head">
                                            <div><span>"КАЛЕНДАРЬ"</span><h2>"Продвижение времени"</h2></div>
                                            <strong>{move || campaign.get().map(|c| date_label(c.game_date)).unwrap_or_default()}</strong>
                                        </div>
                                        <div class="dynasty-action-row">
                                            <button class="dynasty-action" on:click=move |_| apply_campaign(set_campaign, set_feedback, |c| c.advance_days(1))>"+1 день"</button>
                                            <button class="dynasty-action" on:click=move |_| apply_campaign(set_campaign, set_feedback, |c| c.advance_days(7))>"+1 неделя"</button>
                                            <button class="dynasty-action" on:click=move |_| apply_campaign(set_campaign, set_feedback, |c| c.advance_days(30))>"+30 дней"</button>
                                        </div>
                                        <p class="dynasty-muted">"Матчи, события, сроки решений и возраст персонажей обновляются по игровому календарю."</p>
                                    </div>
                                    <div class="dynasty-panel">
                                        <div class="dynasty-panel-head"><div><span>"СЕМЬЯ"</span><h2>"Глава династии"</h2></div></div>
                                        <div class="dynasty-current-head">
                                            <div class="dynasty-avatar">"D"</div>
                                            <div>
                                                <strong>{move || campaign.get().and_then(|c| c.current_character().map(|p| p.full_name.clone())).unwrap_or_default()}</strong>
                                                <small>{move || campaign.get().map(|c| c.current_character().map(|p| format!("{} лет", p.age_on(c.game_date))).unwrap_or_default()).unwrap_or_default()}</small>
                                            </div>
                                            <span>"ГЛАВА ДИНАСТИИ"</span>
                                        </div>
                                    </div>
                                    <div class="dynasty-feedback" role="status">{move || feedback.get()}</div>
                                </div>
                            }.into_any()
                        } else if active_tab.get() == "club" {
                            view! {
                                <div class="dynasty-content">
                                    <div class="dynasty-panel">
                                        <div class="dynasty-panel-head"><div><span>"КЛУБ"</span><h2>{move || campaign.get().map(|c| c.controlled_club().name.clone()).unwrap_or_default()}</h2></div></div>
                                        <div class="dynasty-kpi-grid">
                                            <article class="dynasty-kpi"><span>"ГОРОД"</span><strong>{move || campaign.get().map(|c| c.controlled_club().city.clone()).unwrap_or_default()}</strong></article>
                                            <article class="dynasty-kpi"><span>"СТАДИОН"</span><strong>{move || campaign.get().map(|c| format!("{:} мест", c.controlled_club().stadium_capacity)).unwrap_or_default()}</strong></article>
                                            <article class="dynasty-kpi"><span>"АТАКА"</span><strong>{move || campaign.get().map(|c| c.controlled_club().attack.to_string()).unwrap_or_default()}</strong></article>
                                            <article class="dynasty-kpi"><span>"ОБОРОНА"</span><strong>{move || campaign.get().map(|c| c.controlled_club().defence.to_string()).unwrap_or_default()}</strong></article>
                                        </div>
                                        <p class="dynasty-muted">"Репутация и состав клуба влияют на результаты команды в текущем сезоне."</p>
                                    </div>
                                    <div class="dynasty-panel">
                                        <div class="dynasty-panel-head"><div><span>"ЛИГА"</span><h2>"Клубы мира"</h2></div></div>
                                        {move || campaign.get().map(|c| c.clubs.into_iter().map(|club| view! {
                                            <div class="dynasty-list-row">
                                                <div><strong>{club.name}</strong><small>{club.city} " · " {club.founded_year}</small></div>
                                                <span>{if club.is_player_controlled { "ВАШ КЛУБ" } else { "СОПЕРНИК" }}</span>
                                                <b>{club.reputation}"/100"</b>
                                            </div>
                                        }).collect_view()).unwrap_or_else(|| ().into_view())}
                                    </div>
                                    <div class="dynasty-panel">
                                        <div class="dynasty-panel-head"><div><span>"ФИНАНСЫ"</span><h2>"Денежные счета"</h2></div></div>
                                        <div class="dynasty-list-row"><div><strong>"Казна клуба"</strong><small>"EUR · по проводкам"</small></div><b>{move || campaign.get().map(|c| euros_millions(c.cash_balance(CLUB_CASH_ACCOUNT))).unwrap_or_default()}</b></div>
                                        <div class="dynasty-list-row"><div><strong>"Средства основателя"</strong><small>"EUR · личный счёт"</small></div><b>{move || campaign.get().map(|c| euros_millions(c.cash_balance(sherlock_dynasty_core::FOUNDER_CASH_ACCOUNT))).unwrap_or_default()}</b></div>
                                    </div>
                                </div>
                            }.into_any()
                        } else if active_tab.get() == "season" {
                            view! {
                                <div class="dynasty-content">
                                    <div class="dynasty-panel">
                                        <div class="dynasty-panel-head"><div><span>"ТАБЛИЦА"</span><h2>"Чемпионат"</h2></div><span>"4 КЛУБА"</span></div>
                                        <div class="dynasty-table-wrap">
                                            <table class="dynasty-table">
                                                <thead><tr><th>"#"</th><th>"КЛУБ"</th><th>"И"</th><th>"В"</th><th>"Н"</th><th>"П"</th><th>"МЯЧИ"</th><th>"О"</th></tr></thead>
                                                <tbody>
                                                    {move || campaign.get().map(|c| c.standings().into_iter().enumerate().map(|(index, row)| view! {
                                                        <tr>
                                                            <td>{index + 1}</td><td>{row.team_name}</td><td>{row.played}</td><td>{row.wins}</td><td>{row.draws}</td><td>{row.losses}</td><td>{row.goals_for}":"{row.goals_against}</td><td><strong>{row.points}</strong></td>
                                                        </tr>
                                                    }).collect_view()).unwrap_or_else(|| ().into_view())}
                                                </tbody>
                                            </table>
                                        </div>
                                    </div>
                                    <div class="dynasty-panel">
                                        <div class="dynasty-panel-head"><div><span>"КАЛЕНДАРЬ"</span><h2>"Матчи сезона"</h2></div></div>
                                        {move || campaign.get().map(|c| c.fixtures.into_iter().map(|fixture| {
                                            let home = c.clubs.iter().find(|club| club.id == fixture.home_team_id).map(|club| club.name.clone()).unwrap_or_default();
                                            let away = c.clubs.iter().find(|club| club.id == fixture.away_team_id).map(|club| club.name.clone()).unwrap_or_default();
                                            let score = match (fixture.home_goals, fixture.away_goals) {
                                                (Some(h), Some(a)) => format!("{h} : {a}"),
                                                _ => "— : —".to_owned(),
                                            };
                                            view! {
                                                <div class="dynasty-fixture">
                                                    <div><small>"ТУР " {fixture.matchday} " · " {date_label(fixture.match_date)}</small><strong>{home} <i>{score}</i> {away}</strong></div>
                                                    <span class:played=fixture.status == sherlock_dynasty_core::FixtureStatus::Played>{if fixture.status == sherlock_dynasty_core::FixtureStatus::Played { "ЗАВЕРШЁН" } else { "ОЖИДАЕТСЯ" }}</span>
                                                </div>
                                            }
                                        }).collect_view()).unwrap_or_else(|| ().into_view())}
                                    </div>
                                </div>
                            }.into_any()
                        } else if active_tab.get() == "family" {
                            view! {
                                <div class="dynasty-content dynasty-two-column">
                                    <div class="dynasty-panel">
                                        <div class="dynasty-panel-head"><div><span>"РОДОСЛОВНАЯ"</span><h2>"Семья и наследники"</h2></div></div>
                                        {move || campaign.get().map(|c| c.characters.into_iter().map(|character| {
                                            let age = character.age_on(c.game_date);
                                            view! {
                                                <div class="dynasty-list-row">
                                                    <div><strong>{character.full_name}</strong><small>{age} " лет · " {if character.life_status == CharacterStatus::Alive { "Жив" } else { "Умер" }}</small></div>
                                                    <span>{if character.is_heir { "НАСЛЕДНИК" } else if character.id == c.dynasty.head_character_id { "ГЛАВА" } else { "ЧЛЕН СЕМЬИ" }}</span>
                                                </div>
                                            }
                                        }).collect_view()).unwrap_or_else(|| ().into_view())}
                                        {move || campaign.get().filter(|c| c.status == CampaignStatus::Active).map(|_| view! {
                                            <div class="dynasty-form-row">
                                                <input placeholder="Имя наследника" prop:value=move || heir_name.get() on:input=move |event| set_heir_name.set(event_target_value(&event)) />
                                                <input class="age-input" type="number" min="0" max="70" prop:value=move || heir_age.get() on:input=move |event| set_heir_age.set(event_target_value(&event)) />
                                                <button class="dynasty-action" on:click=on_add_heir>"Добавить"</button>
                                            </div>
                                        })}
                                    </div>
                                    <div class="dynasty-panel">
                                        <div class="dynasty-panel-head"><div><span>"НАСЛЕДОВАНИЕ"</span><h2>"Преемственность"</h2></div></div>
                                        {move || campaign.get().map(|c| {
                                            let candidates: Vec<_> = c.succession_candidates().into_iter().cloned().collect();
                                            if c.status == CampaignStatus::SuccessionPending {
                                                view! {
                                                    <div>
                                                        <p class="dynasty-muted">"Основатель умер. Выберите совершеннолетнего наследника."</p>
                                                        {candidates.into_iter().map(|candidate| {
                                                            let id = candidate.id;
                                                            view! {
                                                                <div class="dynasty-list-row">
                                                                    <div><strong>{candidate.full_name}</strong><small>{candidate.age_on(c.game_date)} " лет"</small></div>
                                                                    <button class="dynasty-action" on:click=move |_| apply_campaign(set_campaign, set_feedback, move |active| active.execute_succession(id))>"Назначить главой"</button>
                                                                </div>
                                                            }
                                                        }).collect_view()}
                                                    </div>
                                                }.into_any()
                                            } else if c.status == CampaignStatus::EndedNoHeir {
                                                view! { <p class="dynasty-muted">"У основателя не осталось совершеннолетнего наследника. Кампания завершена."</p> }.into_any()
                                            } else {
                                                view! { <p class="dynasty-muted">"Наследование вступит в силу после ухода главы семьи. Наследник должен быть жив и достигнуть 18 лет."</p> }.into_any()
                                            }
                                        }).unwrap_or_else(|| ().into_view())}
                                    </div>
                                </div>
                            }.into_any()
                        } else if active_tab.get() == "events" {
                            view! {
                                <div class="dynasty-content">
                                    {move || campaign.get().map(|c| {
                                        let events = c.events.clone();
                                        events.into_iter().map(|event| {
                                            let event_id = event.id;
                                            view! {
                                                <article class="dynasty-event">
                                                    <div class="dynasty-event-top">
                                                        <span>{date_label(event.game_date)} " · " {event.priority}</span>
                                                        <b class:pending=event.status == EventStatus::Pending>{match event.status { EventStatus::Pending => "НУЖНО РЕШЕНИЕ", EventStatus::Resolved => "РЕШЕНО", EventStatus::Expired => "СРОК ИСТЁК" }}</b>
                                                    </div>
                                                    <h3>{event.title}</h3>
                                                    <p>{event.body}</p>
                                                    {if event.status == EventStatus::Pending {
                                                        view! {
                                                            <div class="dynasty-action-row">
                                                                {event.choices.into_iter().map(|option| {
                                                                    let choice_id = option.id.clone();
                                                                    view! {
                                                                        <button class="dynasty-action" on:click=move |_| apply_campaign(set_campaign, set_feedback, move |active| active.resolve_event(event_id, &choice_id))>{option.label}</button>
                                                                    }
                                                                }).collect_view()}
                                                            </div>
                                                            <small>"Решить до " {date_label(event.expires_on)}</small>
                                                        }.into_any()
                                                    } else {
                                                        view! { <small>"Решение зафиксировано в хронике"</small> }.into_any()
                                                    }}
                                                </article>
                                            }
                                        }).collect_view()
                                    }).unwrap_or_else(|| ().into_view())}
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="dynasty-content">
                                    <div class="dynasty-panel">
                                        <div class="dynasty-panel-head"><div><span>"ИСТОРИЯ"</span><h2>"Хроника династии"</h2></div></div>
                                        {move || campaign.get().map(|c| c.chronicle.into_iter().rev().map(|entry| view! {
                                            <div class="dynasty-chronicle">
                                                <time>{date_label(entry.game_date)}</time>
                                                <div><strong>{entry.title}</strong><p>{entry.detail}</p></div>
                                            </div>
                                        }).collect_view()).unwrap_or_else(|| ().into_view())}
                                    </div>
                                    <div class="dynasty-panel">
                                        <div class="dynasty-panel-head"><div><span>"ФИНАНСОВЫЙ ЖУРНАЛ"</span><h2>"Проводки"</h2></div></div>
                                        {move || campaign.get().map(|c| c.ledger.into_iter().rev().map(|entry| view! {
                                            <div class="dynasty-list-row">
                                                <div><strong>{entry.memo}</strong><small>{date_label(entry.game_date)} " · " {entry.currency}</small></div>
                                                <b>{entry.lines.iter().find(|line| line.account == CLUB_CASH_ACCOUNT && line.side == "debit").map(|line| euros_millions(line.amount_minor)).unwrap_or_else(|| "—".to_owned())}</b>
                                            </div>
                                        }).collect_view()).unwrap_or_else(|| ().into_view())}
                                    </div>
                                </div>
                            }.into_any()
                        }}
                    </section>
                }.into_any(),
            }}
        </div>
    }
}

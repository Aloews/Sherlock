use gloo_net::http::Request;
use js_sys::{Date, Math, Reflect};
use leptos::{prelude::*, task::spawn_local};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

fn api_url(path: &str) -> String {
    let base = option_env!("SHERLOCK_DYNASTY_API_URL").unwrap_or("");
    format!("{}{}", base.trim_end_matches('/'), path)
}

fn telegram_init_data() -> Option<String> {
    let global = js_sys::global();
    let telegram = Reflect::get(&global, &"Telegram".into()).ok()?;
    let web_app = Reflect::get(&telegram, &"WebApp".into()).ok()?;
    let init_data = Reflect::get(&web_app, &"initData".into())
        .ok()?
        .as_string()?;
    (!init_data.trim().is_empty()).then_some(init_data)
}

async fn api_get<T: DeserializeOwned>(path: &str, init_data: &str) -> Result<T, String> {
    let response = Request::get(&api_url(path))
        .header("x-tg-init-data", init_data)
        .send()
        .await
        .map_err(|error| format!("Сервис недоступен: {error}"))?;

    if !response.ok() {
        return Err(format!("Ошибка сервиса: HTTP {}", response.status()));
    }

    response
        .json::<T>()
        .await
        .map_err(|error| format!("Некорректный ответ сервиса: {error}"))
}

async fn api_post<T, B>(path: &str, init_data: &str, body: &B) -> Result<T, String>
where
    T: DeserializeOwned,
    B: Serialize,
{
    let key = format!(
        "fd-{}-{}",
        Date::now() as u64,
        (Math::random() * 1_000_000_000.0) as u64
    );
    let request = Request::post(&api_url(path))
        .header("x-tg-init-data", init_data)
        .header("idempotency-key", &key)
        .json(body)
        .map_err(|error| format!("Не удалось подготовить запрос: {error}"))?;
    let response = request
        .send()
        .await
        .map_err(|error| format!("Сервис недоступен: {error}"))?;

    if !response.ok() {
        return Err(format!("Ошибка сервиса: HTTP {}", response.status()));
    }

    response
        .json::<T>()
        .await
        .map_err(|error| format!("Некорректный ответ сервиса: {error}"))
}

#[derive(Clone, Debug, Deserialize)]
struct CampaignSummary {
    id: String,
    name: String,
    status: String,
    current_year: i32,
    game_date: String,
}

#[derive(Clone, Debug, Deserialize)]
struct Character {
    full_name: String,
    age: i32,
    life_status: String,
    is_heir: bool,
}

#[derive(Clone, Debug, Deserialize)]
struct Dynasty {
    name: String,
    prestige: i32,
}

#[derive(Clone, Debug, Deserialize)]
struct Club {
    name: String,
    reputation: i32,
}

#[derive(Clone, Debug, Deserialize)]
struct CashAccount {
    name: String,
    currency: String,
    balance_minor: i64,
}

#[derive(Clone, Debug, Deserialize)]
struct ChronicleEntry {
    title: String,
    game_year: i32,
}

#[derive(Clone, Debug, Deserialize)]
struct CampaignView {
    campaign: CampaignSummary,
    player_character_id: String,
    founder: Character,
    dynasty: Dynasty,
    characters: Vec<Character>,
    clubs: Vec<Club>,
    cash_accounts: Vec<CashAccount>,
    chronicle: Vec<ChronicleEntry>,
}

#[derive(Clone, Debug, Deserialize)]
struct SeasonFixture {
    matchday: i32,
    match_date: String,
    home_team: String,
    away_team: String,
    status: String,
    home_goals: Option<i32>,
    away_goals: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
struct Standing {
    team: String,
    played: i32,
    wins: i32,
    draws: i32,
    losses: i32,
    goals_for: i32,
    goals_against: i32,
    points: i32,
}

#[derive(Clone, Debug, Deserialize)]
struct SeasonView {
    season_year: i32,
    fixtures: Vec<SeasonFixture>,
    standings: Vec<Standing>,
}

#[derive(Clone, Debug, Deserialize)]
struct GameEvent {
    id: String,
    title: String,
    body: String,
    game_date: String,
    expires_on: String,
    status: String,
    choices: Vec<EventChoice>,
}

#[derive(Clone, Debug, Deserialize)]
struct EventChoice {
    id: String,
    label: String,
}

#[derive(Clone, Debug, Deserialize)]
struct CampaignCreated {
    id: String,
}

#[derive(Clone, Debug, Deserialize)]
struct EmptyResponse {}

#[derive(Serialize)]
struct CreateCampaignBody<'a> {
    name: &'a str,
    mode: &'a str,
    origin: &'a str,
    founder_name: &'a str,
    dynasty_name: &'a str,
    club_name: &'a str,
}

#[derive(Serialize)]
struct AdvanceBody {
    days: i32,
}

#[derive(Serialize)]
struct AddHeirBody<'a> {
    name: &'a str,
    age: i32,
}

#[derive(Serialize)]
struct ResolveEventBody<'a> {
    choice_id: &'a str,
}

#[derive(Clone)]
struct CampaignBundle {
    detail: CampaignView,
    season: SeasonView,
    events: Vec<GameEvent>,
}

async fn load_campaign_bundle(
    campaign_id: &str,
    init_data: &str,
) -> Result<CampaignBundle, String> {
    let id = campaign_id;
    let detail = api_get::<CampaignView>(&format!("/api/v1/campaigns/{id}"), init_data).await?;
    let season =
        api_get::<SeasonView>(&format!("/api/v1/campaigns/{id}/season"), init_data).await?;
    let events =
        api_get::<Vec<GameEvent>>(&format!("/api/v1/campaigns/{id}/events"), init_data).await?;

    Ok(CampaignBundle {
        detail,
        season,
        events,
    })
}

#[component]
pub fn DynastyApp() -> impl IntoView {
    let init_data = RwSignal::new(telegram_init_data());
    let campaigns = RwSignal::new(Vec::<CampaignSummary>::new());
    let active_bundle = RwSignal::new(None::<CampaignBundle>);
    let selected_campaign = RwSignal::new(None::<String>);
    let busy = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);

    let campaign_name = RwSignal::new("Новая династия".to_string());
    let founder_name = RwSignal::new("Алекс Морган".to_string());
    let dynasty_name = RwSignal::new("Дом Морганов".to_string());
    let club_name = RwSignal::new("Northbridge United".to_string());
    let origin = RwSignal::new("entrepreneur".to_string());
    let heir_name = RwSignal::new("Джордан Морган".to_string());
    let heir_age = RwSignal::new("16".to_string());

    if let Some(auth) = init_data.get() {
        let campaigns_signal = campaigns;
        let error_signal = error;
        spawn_local(async move {
            match api_get::<Vec<CampaignSummary>>("/api/v1/campaigns", &auth).await {
                Ok(found) => {
                    campaigns_signal.set(found);
                    error_signal.set(None);
                }
                Err(message) => error_signal.set(Some(message)),
            }
        });
    }

    let select_campaign = move |id: String| {
        let auth = init_data.get().unwrap_or_default();
        let active = active_bundle;
        let selected = selected_campaign;
        let error_signal = error;
        let busy_signal = busy;
        busy_signal.set(true);
        spawn_local(async move {
            match load_campaign_bundle(&id, &auth).await {
                Ok(bundle) => {
                    active.set(Some(bundle));
                    selected.set(Some(id));
                    error_signal.set(None);
                }
                Err(message) => error_signal.set(Some(message)),
            }
            busy_signal.set(false);
        });
    };

    let refresh_selected = move || {
        if let Some(id) = selected_campaign.get() {
            let auth = init_data.get().unwrap_or_default();
            let active = active_bundle;
            let error_signal = error;
            let busy_signal = busy;
            spawn_local(async move {
                match load_campaign_bundle(&id, &auth).await {
                    Ok(bundle) => {
                        active.set(Some(bundle));
                        error_signal.set(None);
                    }
                    Err(message) => error_signal.set(Some(message)),
                }
                busy_signal.set(false);
            });
        }
    };

    let create_campaign = move |_| {
        let Some(auth) = init_data.get() else {
            error.set(Some("Откройте Sherlock через Telegram Mini App.".into()));
            return;
        };
        let name = campaign_name.get();
        let founder = founder_name.get();
        let dynasty = dynasty_name.get();
        let club = club_name.get();
        let origin_value = origin.get();
        if name.trim().is_empty()
            || founder.trim().is_empty()
            || dynasty.trim().is_empty()
            || club.trim().is_empty()
        {
            error.set(Some(
                "Заполните название кампании, основателя, династию и клуб.".into(),
            ));
            return;
        }

        busy.set(true);
        error.set(None);
        spawn_local(async move {
            let body = CreateCampaignBody {
                name: &name,
                mode: "sandbox",
                origin: &origin_value,
                founder_name: &founder,
                dynasty_name: &dynasty,
                club_name: &club,
            };
            match api_post::<CampaignCreated, _>("/api/v1/campaigns", &auth, &body).await {
                Ok(created) => match load_campaign_bundle(&created.id, &auth).await {
                    Ok(bundle) => {
                        active_bundle.set(Some(bundle));
                        selected_campaign.set(Some(created.id));
                        if let Ok(found) =
                            api_get::<Vec<CampaignSummary>>("/api/v1/campaigns", &auth).await
                        {
                            campaigns.set(found);
                        }
                    }
                    Err(message) => error.set(Some(message)),
                },
                Err(message) => error.set(Some(message)),
            }
            busy.set(false);
        });
    };

    let add_heir = move |_| {
        let Some(auth) = init_data.get() else {
            return;
        };
        let Some(id) = selected_campaign.get() else {
            return;
        };
        let name = heir_name.get();
        let age = heir_age.get().parse::<i32>().unwrap_or(-1);
        if name.trim().is_empty() || !(0..=70).contains(&age) {
            error.set(Some("Проверьте имя и возраст наследника.".into()));
            return;
        }
        busy.set(true);
        spawn_local(async move {
            let body = AddHeirBody { name: &name, age };
            match api_post::<Character, _>(&format!("/api/v1/campaigns/{id}/heirs"), &auth, &body)
                .await
            {
                Ok(_) => refresh_selected(),
                Err(message) => error.set(Some(message)),
            }
            busy.set(false);
        });
    };

    let advance_time = move |days: i32| {
        let Some(auth) = init_data.get() else {
            return;
        };
        let Some(id) = selected_campaign.get() else {
            return;
        };
        busy.set(true);
        error.set(None);
        spawn_local(async move {
            let body = AdvanceBody { days };
            match api_post::<EmptyResponse, _>(
                &format!("/api/v1/campaigns/{id}/time/advance"),
                &auth,
                &body,
            )
            .await
            {
                Ok(_) => refresh_selected(),
                Err(message) => error.set(Some(message)),
            }
            busy.set(false);
        });
    };

    let resolve_event = move |event_id: String, choice_id: String| {
        let Some(auth) = init_data.get() else {
            return;
        };
        let Some(id) = selected_campaign.get() else {
            return;
        };
        busy.set(true);
        spawn_local(async move {
            let body = ResolveEventBody {
                choice_id: &choice_id,
            };
            match api_post::<EmptyResponse, _>(
                &format!("/api/v1/campaigns/{id}/events/{event_id}/resolve"),
                &auth,
                &body,
            )
            .await
            {
                Ok(_) => refresh_selected(),
                Err(message) => error.set(Some(message)),
            }
            busy.set(false);
        });
    };

    view! {
        <section class="dynasty-page">
            <div class="dynasty-heading">
                <div>
                    <div class="eyebrow"><span class="eyebrow-dot"></span>"FOOTBALL DYNASTY"</div>
                    <h1>"Футбольная "<span>"династия"</span></h1>
                </div>
                <span class="dynasty-season-mark">"КАМПАНИЯ / "{move || active_bundle.get().map(|b| b.detail.campaign.current_year.to_string()).unwrap_or_else(|| "НОВАЯ".into())}</span>
            </div>

            {move || error.get().map(|message| view! {
                <div class="dynasty-error" role="alert">{message}</div>
            })}

            {move || {
                if init_data.get().is_none() {
                    view! {
                        <section class="dynasty-panel dynasty-auth-panel">
                            <div class="dynasty-panel-kicker">"ДОСТУП К КАМПАНИИ"</div>
                            <h2>"Откройте Sherlock в Telegram"</h2>
                            <p>"Кампании привязаны к подтверждённому Telegram-профилю. В браузере без Telegram авторизация недоступна."</p>
                        </section>
                    }.into_any()
                } else if let Some(bundle) = active_bundle.get() {
                    let detail = bundle.detail.clone();
                    let season = bundle.season.clone();
                    let events = bundle.events.clone();
                    view! {
                        <div class="dynasty-toolbar">
                            <button class="dynasty-secondary" disabled=move || busy.get() on:click=move |_| { active_bundle.set(None); selected_campaign.set(None); }>"Все кампании"</button>
                            <span>{detail.campaign.name.clone()}</span>
                            <button class="dynasty-secondary" disabled=move || busy.get() on:click=move |_| refresh_selected()>"Обновить"</button>
                        </div>

                        <div class="dynasty-stat-grid">
                            <article class="dynasty-stat"><span>"ДИНАСТИЯ"</span><strong>{detail.dynasty.name.clone()}</strong><small>"Престиж · "{detail.dynasty.prestige}</small></article>
                            <article class="dynasty-stat"><span>"ОСНОВАТЕЛЬ"</span><strong>{detail.founder.full_name.clone()}</strong><small>"Возраст · "{detail.founder.age}</small></article>
                            <article class="dynasty-stat"><span>"КЛУБ"</span><strong>{detail.clubs.first().map(|c| c.name.clone()).unwrap_or_else(|| "—".into())}</strong><small>"Репутация · "{detail.clubs.first().map(|c| c.reputation).unwrap_or(0)}/100</small></article>
                            <article class="dynasty-stat"><span>"ИГРОВОЙ ГОД"</span><strong>{detail.campaign.current_year}</strong><small>{detail.campaign.game_date.clone()}</small></article>
                        </div>

                        <div class="dynasty-columns">
                            <section class="dynasty-panel">
                                <div class="dynasty-section-title"><h2>"Состав семьи"</h2><span>{detail.characters.len()} " ПОКОЛЕНИЙ / ЧЛЕНОВ"</span></div>
                                <div class="dynasty-list">
                                    {detail.characters.iter().map(|character| view! {
                                        <div class="dynasty-list-row">
                                            <div><strong>{character.full_name.clone()}</strong><small>"Возраст · "{character.age} " · " {if character.life_status == "alive" { "Жив" } else { "Умер" }}</small></div>
                                            {if character.is_heir { view!{<span class="dynasty-pill">"НАСЛЕДНИК"</span>}.into_any() } else { view!{<span></span>}.into_any() }}
                                        </div>
                                    }).collect_view()}
                                </div>
                                {if detail.campaign.status == "active" {
                                    view! {
                                        <div class="dynasty-add-heir">
                                            <h3>"Назначить наследника"</h3>
                                            <input prop:value=move || heir_name.get() on:input=move |event| heir_name.set(event_target_value(&event)) placeholder="Имя наследника"/>
                                            <div class="dynasty-inline">
                                                <input type="number" min="0" max="70" prop:value=move || heir_age.get() on:input=move |event| heir_age.set(event_target_value(&event))/>
                                                <button class="dynasty-primary" disabled=move || busy.get() on:click=add_heir>"Добавить"</button>
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <div></div> }.into_any()
                                }}
                            </section>

                            <section class="dynasty-panel">
                                <div class="dynasty-section-title"><h2>"Календарь"</h2><span>{detail.campaign.game_date.clone()}</span></div>
                                <div class="dynasty-calendar-actions">
                                    <button disabled=move || busy.get() on:click=move |_| advance_time(1)>"+1 день"</button>
                                    <button disabled=move || busy.get() on:click=move |_| advance_time(7)>"+7 дней"</button>
                                    <button disabled=move || busy.get() on:click=move |_| advance_time(30)>"+30 дней"</button>
                                </div>
                                <div class="dynasty-section-title dynasty-subtitle"><h2>"Финансы"</h2><span>"EUR"</span></div>
                                {detail.cash_accounts.iter().map(|account| view! {
                                    <div class="dynasty-money-row">
                                        <span>{account.name.clone()}</span>
                                        <strong>{format!("{:.2} {}", account.balance_minor as f64 / 100.0, account.currency)}</strong>
                                    </div>
                                }).collect_view()}
                                <div class="dynasty-section-title dynasty-subtitle"><h2>"Хроника"</h2><span>{detail.chronicle.len()} " ЗАПИСЕЙ"</span></div>
                                {detail.chronicle.iter().rev().take(5).map(|entry| view! {
                                    <div class="dynasty-chronicle-row"><small>{entry.game_year}</small><span>{entry.title.clone()}</span></div>
                                }).collect_view()}
                            </section>
                        </div>

                        <section class="dynasty-panel dynasty-season-panel">
                            <div class="dynasty-section-title"><h2>"Чемпионат · "{season.season_year}</h2><span>{season.fixtures.len()} " МАТЧЕЙ"</span></div>
                            <div class="dynasty-table">
                                <div class="dynasty-table-head"><span>"КЛУБ"</span><span>"И"</span><span>"В"</span><span>"Н"</span><span>"П"</span><span>"МЯЧИ"</span><span>"ОЧКИ"</span></div>
                                {season.standings.iter().map(|row| view! {
                                    <div class="dynasty-table-row"><strong>{row.team.clone()}</strong><span>{row.played}</span><span>{row.wins}</span><span>{row.draws}</span><span>{row.losses}</span><span>{row.goals_for}-{row.goals_against}</span><b>{row.points}</b></div>
                                }).collect_view()}
                            </div>
                            <div class="dynasty-fixtures">
                                {season.fixtures.iter().map(|fixture| view! {
                                    <div class="dynasty-fixture-row">
                                        <small>"ТУР "{fixture.matchday} " · " {fixture.match_date.clone()}</small>
                                        <strong>{fixture.home_team.clone()} "  " {fixture.home_goals.map(|n|n.to_string()).unwrap_or_else(|| "—".into())} " : " {fixture.away_goals.map(|n|n.to_string()).unwrap_or_else(|| "—".into())} "  " {fixture.away_team.clone()}</strong>
                                        <span>{if fixture.status == "played" { "ЗАВЕРШЁН" } else { "В РАСПИСАНИИ" }}</span>
                                    </div>
                                }).collect_view()}
                            </div>
                        </section>

                        <section class="dynasty-panel">
                            <div class="dynasty-section-title"><h2>"Решения и события"</h2><span>{events.iter().filter(|e| e.status == "pending").count()} " ОЖИДАЮТ РЕШЕНИЯ"</span></div>
                            {events.iter().map(|event| {
                                let event_id = event.id.clone();
                                let choices = event.choices.clone();
                                view! {
                                    <article class="dynasty-event-row">
                                        <div><small>{event.game_date.clone()} " · " {event.expires_on.clone()}</small><strong>{event.title.clone()}</strong><p>{event.body.clone()}</p></div>
                                        {if event.status == "pending" {
                                            view! {<div class="dynasty-choice-row">
                                                {choices.into_iter().map(|choice| {
                                                    let id = event_id.clone();
                                                    let choice_id = choice.id.clone();
                                                    view! {<button disabled=move || busy.get() on:click=move |_| resolve_event(id.clone(),choice_id.clone())>{choice.label}</button>}
                                                }).collect_view()}
                                            </div>}.into_any()
                                        } else { view! {<span class="dynasty-pill">{event.status.clone()}</span>}.into_any() }}
                                    </article>
                                }
                            }).collect_view()}
                        </section>
                    }.into_any()
                } else {
                    let available = campaigns.get();
                    view! {
                        <div class="dynasty-columns">
                            <section class="dynasty-panel">
                                <div class="dynasty-section-title"><h2>"Новая кампания"</h2><span>"SANDBOX"</span></div>
                                <label class="dynasty-field"><span>"НАЗВАНИЕ КАМПАНИИ"</span><input prop:value=move || campaign_name.get() on:input=move |event| campaign_name.set(event_target_value(&event))/></label>
                                <label class="dynasty-field"><span>"ОСНОВАТЕЛЬ"</span><input prop:value=move || founder_name.get() on:input=move |event| founder_name.set(event_target_value(&event))/></label>
                                <label class="dynasty-field"><span>"ФАМИЛЬНАЯ ДИНАСТИЯ"</span><input prop:value=move || dynasty_name.get() on:input=move |event| dynasty_name.set(event_target_value(&event))/></label>
                                <label class="dynasty-field"><span>"ФУТБОЛЬНЫЙ КЛУБ"</span><input prop:value=move || club_name.get() on:input=move |event| club_name.set(event_target_value(&event))/></label>
                                <label class="dynasty-field"><span>"ПРОИСХОЖДЕНИЕ"</span>
                                    <select prop:value=move || origin.get() on:change=move |event| origin.set(event_target_value(&event))>
                                        <option value="entrepreneur">"Предприниматель"</option>
                                        <option value="football_family">"Футбольная семья"</option>
                                        <option value="former_player">"Бывший игрок"</option>
                                        <option value="agent">"Футбольный агент"</option>
                                        <option value="executive">"Управленец"</option>
                                        <option value="newcomer">"Новый участник"</option>
                                    </select>
                                </label>
                                <button class="dynasty-primary dynasty-create" disabled=move || busy.get() on:click=create_campaign>
                                    {move || if busy.get() { "Создание..." } else { "Создать династию" }}
                                </button>
                            </section>
                            <section class="dynasty-panel">
                                <div class="dynasty-section-title"><h2>"Сохранённые кампании"</h2><span>{available.len()}</span></div>
                                {available.iter().map(|campaign| {
                                    let id = campaign.id.clone();
                                    view! {<button class="dynasty-campaign-card" disabled=move || busy.get() on:click=move |_| select_campaign(id.clone())>
                                        <strong>{campaign.name.clone()}</strong><small>{campaign.current_year} " · " {campaign.game_date.clone()} " · " {campaign.status.clone()}</small>
                                    </button>}
                                }).collect_view()}
                                {if available.is_empty() { view!{<p class="dynasty-empty">"Сохранённых кампаний пока нет."</p>}.into_any() } else { view!{<span></span>}.into_any() }}
                            </section>
                        </div>
                    }.into_any()
                }
            }}
        </section>
    }
}

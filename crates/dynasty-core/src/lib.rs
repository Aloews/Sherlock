use chrono::{Datelike, Duration, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const ENGINE_VERSION: u32 = 1;
pub const FOUNDER_CASH_ACCOUNT: &str = "founder_cash";
pub const CLUB_CASH_ACCOUNT: &str = "club_treasury";
pub const OPENING_CAPITAL_ACCOUNT: &str = "opening_capital";
pub const SPONSOR_REVENUE_ACCOUNT: &str = "sponsorship_revenue";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CampaignMode {
    Sandbox,
    Scenario,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Origin {
    FootballFamily,
    Entrepreneur,
    FormerPlayer,
    Agent,
    Executive,
    Newcomer,
}

impl Origin {
    pub fn key(self) -> &'static str {
        match self {
            Self::FootballFamily => "football_family",
            Self::Entrepreneur => "entrepreneur",
            Self::FormerPlayer => "former_player",
            Self::Agent => "agent",
            Self::Executive => "executive",
            Self::Newcomer => "newcomer",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::FootballFamily => "Футбольная семья",
            Self::Entrepreneur => "Предприниматель",
            Self::FormerPlayer => "Бывший профессиональный игрок",
            Self::Agent => "Футбольный агент",
            Self::Executive => "Управленец",
            Self::Newcomer => "Новый человек в футболе",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CampaignSetup {
    pub name: String,
    pub mode: CampaignMode,
    pub founder_name: String,
    pub dynasty_name: String,
    pub club_name: String,
    pub origin: Origin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CampaignStatus {
    Active,
    SuccessionPending,
    EndedNoHeir,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CharacterStatus {
    Alive,
    Deceased,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Character {
    pub id: u64,
    pub full_name: String,
    pub birth_date: NaiveDate,
    pub life_status: CharacterStatus,
    pub is_heir: bool,
    pub parent_character_id: Option<u64>,
    pub origin: Origin,
    pub diplomacy: u8,
    pub management: u8,
    pub finance: u8,
    pub football_knowledge: u8,
    pub negotiation: u8,
    pub public_standing: u8,
}

impl Character {
    pub fn age_on(&self, date: NaiveDate) -> u8 {
        let mut years = date.year() - self.birth_date.year();
        if (date.month(), date.day()) < (self.birth_date.month(), self.birth_date.day()) {
            years -= 1;
        }
        years.clamp(0, 120) as u8
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dynasty {
    pub id: u64,
    pub name: String,
    pub head_character_id: u64,
    pub prestige: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Club {
    pub id: u64,
    pub name: String,
    pub city: String,
    pub reputation: i32,
    pub attack: i32,
    pub defence: i32,
    pub stadium_capacity: u32,
    pub founded_year: i32,
    pub is_player_controlled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FixtureStatus {
    Scheduled,
    Played,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fixture {
    pub id: u64,
    pub season_year: i32,
    pub matchday: u8,
    pub match_date: NaiveDate,
    pub home_team_id: u64,
    pub away_team_id: u64,
    pub status: FixtureStatus,
    pub home_goals: Option<u8>,
    pub away_goals: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Standing {
    pub team_id: u64,
    pub team_name: String,
    pub played: u16,
    pub wins: u16,
    pub draws: u16,
    pub losses: u16,
    pub goals_for: i32,
    pub goals_against: i32,
    pub points: u16,
}

impl Standing {
    pub fn goal_difference(&self) -> i32 {
        self.goals_for - self.goals_against
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventStatus {
    Pending,
    Resolved,
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventChoice {
    pub id: String,
    pub label: String,
    pub effect: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameEvent {
    pub id: u64,
    pub event_type: String,
    pub title: String,
    pub body: String,
    pub game_date: NaiveDate,
    pub expires_on: NaiveDate,
    pub priority: u8,
    pub choices: Vec<EventChoice>,
    pub status: EventStatus,
    pub resolution: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronicleEntry {
    pub id: u64,
    pub event_type: String,
    pub title: String,
    pub game_date: NaiveDate,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerLine {
    pub account: String,
    pub side: String,
    pub amount_minor: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub id: u64,
    pub idempotency_key: String,
    pub memo: String,
    pub game_date: NaiveDate,
    pub currency: String,
    pub lines: Vec<LedgerLine>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Campaign {
    pub engine_version: u32,
    pub id: u64,
    pub seed: u64,
    pub name: String,
    pub mode: CampaignMode,
    pub status: CampaignStatus,
    pub game_date: NaiveDate,
    pub start_date: NaiveDate,
    pub founder_character_id: u64,
    pub player_character_id: u64,
    pub dynasty: Dynasty,
    pub characters: Vec<Character>,
    pub clubs: Vec<Club>,
    pub controlled_club_id: u64,
    pub fixtures: Vec<Fixture>,
    pub events: Vec<GameEvent>,
    pub chronicle: Vec<ChronicleEntry>,
    pub ledger: Vec<LedgerEntry>,
    pub generated_weeks: Vec<u32>,
    next_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DynastyError {
    InvalidName(&'static str),
    InvalidAdvanceDays(u8),
    CampaignNotActive,
    EventNotFound,
    EventExpired,
    InvalidChoice,
    InvalidHeir,
    SuccessionNotReady,
    JournalUnbalanced,
    DuplicateTransaction,
}

impl std::fmt::Display for DynastyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidName(field) => write!(f, "Некорректное значение: {field}"),
            Self::InvalidAdvanceDays(days) => {
                write!(f, "Нельзя продвинуть календарь на {days} дней")
            }
            Self::CampaignNotActive => write!(f, "Кампания ожидает решения по наследованию"),
            Self::EventNotFound => write!(f, "Событие не найдено"),
            Self::EventExpired => write!(f, "Срок решения по событию истёк"),
            Self::InvalidChoice => write!(f, "Такого решения нет"),
            Self::InvalidHeir => write!(f, "Наследник не соответствует правилам наследования"),
            Self::SuccessionNotReady => write!(f, "Наследование пока не может быть завершено"),
            Self::JournalUnbalanced => write!(f, "Финансовая операция не сбалансирована"),
            Self::DuplicateTransaction => write!(f, "Эта финансовая операция уже была проведена"),
        }
    }
}

impl std::error::Error for DynastyError {}

impl Campaign {
    pub fn new(setup: CampaignSetup, seed: u64) -> Result<Self, DynastyError> {
        validate_name(&setup.name, 64, "название кампании")?;
        validate_name(&setup.founder_name, 80, "имя основателя")?;
        validate_name(&setup.dynasty_name, 64, "название династии")?;
        validate_name(&setup.club_name, 64, "название клуба")?;

        let start_date = NaiveDate::from_ymd_opt(2026, 1, 1).expect("valid campaign epoch");
        let founder_birth_date =
            NaiveDate::from_ymd_opt(1986, 1, 1).expect("valid founder birth date");
        let founder_id = 1;
        let dynasty_id = 2;
        let club_id = 3;
        let mut campaign = Self {
            engine_version: ENGINE_VERSION,
            id: mix64(seed ^ 0x5348_4552_4c4f_434b),
            seed,
            name: setup.name.trim().to_owned(),
            mode: setup.mode,
            status: CampaignStatus::Active,
            game_date: start_date,
            start_date,
            founder_character_id: founder_id,
            player_character_id: founder_id,
            dynasty: Dynasty {
                id: dynasty_id,
                name: setup.dynasty_name.trim().to_owned(),
                head_character_id: founder_id,
                prestige: 10,
            },
            characters: vec![Character {
                id: founder_id,
                full_name: setup.founder_name.trim().to_owned(),
                birth_date: founder_birth_date,
                life_status: CharacterStatus::Alive,
                is_heir: false,
                parent_character_id: None,
                origin: setup.origin,
                diplomacy: origin_skill(setup.origin, 0),
                management: origin_skill(setup.origin, 1),
                finance: origin_skill(setup.origin, 2),
                football_knowledge: origin_skill(setup.origin, 3),
                negotiation: origin_skill(setup.origin, 4),
                public_standing: origin_skill(setup.origin, 5),
            }],
            clubs: vec![
                Club {
                    id: club_id,
                    name: setup.club_name.trim().to_owned(),
                    city: "Нортбридж".to_owned(),
                    reputation: 35,
                    attack: 67,
                    defence: 64,
                    stadium_capacity: 28_500,
                    founded_year: 1984,
                    is_player_controlled: true,
                },
                Club {
                    id: 4,
                    name: "Истборо Юнайтед".to_owned(),
                    city: "Истборо".to_owned(),
                    reputation: 43,
                    attack: 72,
                    defence: 68,
                    stadium_capacity: 31_000,
                    founded_year: 1912,
                    is_player_controlled: false,
                },
                Club {
                    id: 5,
                    name: "Кингсфорд Атлетик".to_owned(),
                    city: "Кингсфорд".to_owned(),
                    reputation: 39,
                    attack: 65,
                    defence: 71,
                    stadium_capacity: 24_000,
                    founded_year: 1921,
                    is_player_controlled: false,
                },
                Club {
                    id: 6,
                    name: "Риверсайд Сити".to_owned(),
                    city: "Риверсайд".to_owned(),
                    reputation: 31,
                    attack: 62,
                    defence: 61,
                    stadium_capacity: 19_500,
                    founded_year: 1976,
                    is_player_controlled: false,
                },
            ],
            controlled_club_id: club_id,
            fixtures: Vec::new(),
            events: Vec::new(),
            chronicle: Vec::new(),
            ledger: Vec::new(),
            generated_weeks: Vec::new(),
            next_id: 10,
        };

        campaign.post_journal(
            "campaign-opening",
            "Начальный капитал кампании",
            vec![
                LedgerLine {
                    account: FOUNDER_CASH_ACCOUNT.to_owned(),
                    side: "debit".to_owned(),
                    amount_minor: 2_500_000_000,
                },
                LedgerLine {
                    account: CLUB_CASH_ACCOUNT.to_owned(),
                    side: "debit".to_owned(),
                    amount_minor: 1_000_000_000,
                },
                LedgerLine {
                    account: OPENING_CAPITAL_ACCOUNT.to_owned(),
                    side: "credit".to_owned(),
                    amount_minor: 3_500_000_000,
                },
            ],
        )?;

        let fixtures_start = next_saturday(start_date);
        campaign.fixtures = generate_schedule(
            &campaign.clubs,
            fixtures_start,
            start_date.year(),
            &mut campaign.next_id,
        );
        let founder_name = campaign.characters[0].full_name.clone();
        let dynasty_name = campaign.dynasty.name.clone();
        let club_name = campaign.controlled_club().name.clone();
        campaign.push_chronicle(
            "campaign_founded",
            "Начало футбольной династии",
            format!(
                "{founder_name} начинает путь семьи «{dynasty_name}» во главе клуба «{club_name}»."
            ),
        );

        campaign.push_event(
            "club_launch_plan",
            "Первое решение совета",
            "Совет директоров ждёт от нового владельца первого решения. На что направить внимание в начале проекта?",
            95,
            vec![
                choice("reputation", "Развивать клуб и укреплять его репутацию", "club_reputation"),
                choice("prestige", "Укреплять влияние и престиж династии", "dynasty_prestige"),
            ],
            14,
        );
        campaign.push_event(
            "dynasty_public_debut",
            "Публичный дебют династии",
            "Местная пресса готовит материал о новой футбольной династии. Какой образ закрепить за семьёй?",
            85,
            vec![
                choice("reputation", "Открыть клуб болельщикам и показать амбиции", "club_reputation"),
                choice("prestige", "Подчеркнуть традиции и статус семьи", "dynasty_prestige"),
            ],
            14,
        );
        campaign.push_event(
            "board_priorities",
            "Приоритеты совета",
            "Руководители клуба предлагают определить главный ориентир на ближайший месяц.",
            80,
            vec![
                choice(
                    "reputation",
                    "Сосредоточиться на спортивном развитии",
                    "club_reputation",
                ),
                choice("prestige", "Укрепить влияние семьи", "dynasty_prestige"),
            ],
            14,
        );

        Ok(campaign)
    }

    pub fn controlled_club(&self) -> &Club {
        self.clubs
            .iter()
            .find(|club| club.id == self.controlled_club_id)
            .expect("controlled club must exist")
    }

    pub fn current_character(&self) -> Option<&Character> {
        self.characters
            .iter()
            .find(|character| character.id == self.player_character_id)
    }

    pub fn cash_balance(&self, account: &str) -> i64 {
        self.ledger
            .iter()
            .flat_map(|entry| entry.lines.iter())
            .filter(|line| line.account == account)
            .map(|line| {
                if line.side == "debit" {
                    line.amount_minor
                } else {
                    -line.amount_minor
                }
            })
            .sum()
    }

    pub fn advance_days(&mut self, days: u8) -> Result<(), DynastyError> {
        if !(1..=31).contains(&days) {
            return Err(DynastyError::InvalidAdvanceDays(days));
        }
        if self.status != CampaignStatus::Active {
            return Err(DynastyError::CampaignNotActive);
        }

        for _ in 0..days {
            self.game_date += Duration::days(1);
            self.expire_events();
            self.resolve_due_fixtures();
            let week = (self.game_date - self.start_date).num_days().max(0) as u32 / 7;
            if week > 0 && !self.generated_weeks.contains(&week) {
                self.generate_weekly_event(week);
            }
            self.check_founder_lifecycle();
            if self.status != CampaignStatus::Active {
                break;
            }
        }
        Ok(())
    }

    pub fn add_heir(&mut self, name: &str, age: u8) -> Result<u64, DynastyError> {
        if self.status != CampaignStatus::Active {
            return Err(DynastyError::CampaignNotActive);
        }
        validate_name(name, 80, "имя наследника")?;
        if age > 70
            || self
                .characters
                .iter()
                .any(|c| c.full_name.eq_ignore_ascii_case(name.trim()))
        {
            return Err(DynastyError::InvalidHeir);
        }
        let id = self.allocate_id();
        let parent = self.player_character_id;
        let founder_origin = self
            .current_character()
            .map(|character| character.origin)
            .unwrap_or(Origin::Newcomer);
        let birth_year = self.game_date.year() - i32::from(age);
        let birth_date = self
            .game_date
            .with_year(birth_year)
            .or_else(|| {
                self.game_date
                    .with_day(28)
                    .and_then(|date| date.with_year(birth_year))
            })
            .ok_or(DynastyError::InvalidHeir)?;
        self.characters.push(Character {
            id,
            full_name: name.trim().to_owned(),
            birth_date,
            life_status: CharacterStatus::Alive,
            is_heir: true,
            parent_character_id: Some(parent),
            origin: founder_origin,
            diplomacy: 45,
            management: 45,
            finance: 45,
            football_knowledge: 45,
            negotiation: 45,
            public_standing: 45,
        });
        self.push_chronicle(
            "heir_designated",
            "Новый наследник семьи",
            format!(
                "{} включён в линию наследования семьи «{}».",
                name.trim(),
                self.dynasty.name
            ),
        );
        Ok(id)
    }

    pub fn resolve_event(&mut self, event_id: u64, choice_id: &str) -> Result<(), DynastyError> {
        let index = self
            .events
            .iter()
            .position(|event| event.id == event_id)
            .ok_or(DynastyError::EventNotFound)?;
        if self.events[index].status != EventStatus::Pending {
            return Err(DynastyError::EventExpired);
        }
        if self.game_date > self.events[index].expires_on {
            self.events[index].status = EventStatus::Expired;
            return Err(DynastyError::EventExpired);
        }
        let selected = self.events[index]
            .choices
            .iter()
            .find(|choice| choice.id == choice_id)
            .cloned()
            .ok_or(DynastyError::InvalidChoice)?;

        match selected.effect.as_str() {
            "club_reputation" => {
                let club = self
                    .clubs
                    .iter_mut()
                    .find(|club| club.id == self.controlled_club_id)
                    .ok_or(DynastyError::InvalidChoice)?;
                club.reputation = (club.reputation + 5).clamp(0, 100);
            }
            "dynasty_prestige" => self.dynasty.prestige = (self.dynasty.prestige + 5).clamp(0, 100),
            "sponsor_offer" => {
                self.post_journal(
                    &format!("event-{}-sponsor", event_id),
                    "Спонсорский контракт",
                    vec![
                        LedgerLine {
                            account: CLUB_CASH_ACCOUNT.to_owned(),
                            side: "debit".to_owned(),
                            amount_minor: 25_000_000,
                        },
                        LedgerLine {
                            account: SPONSOR_REVENUE_ACCOUNT.to_owned(),
                            side: "credit".to_owned(),
                            amount_minor: 25_000_000,
                        },
                    ],
                )?;
            }
            _ => return Err(DynastyError::InvalidChoice),
        }

        self.events[index].status = EventStatus::Resolved;
        self.events[index].resolution = Some(selected.id.clone());
        let title = self.events[index].title.clone();
        self.push_chronicle(
            "event_resolved",
            "Решение принято",
            format!("«{}»: {}.", title, selected.label),
        );
        Ok(())
    }

    pub fn succession_candidates(&self) -> Vec<&Character> {
        self.characters
            .iter()
            .filter(|character| {
                character.id != self.player_character_id
                    && character.is_heir
                    && character.life_status == CharacterStatus::Alive
                    && character.age_on(self.game_date) >= 18
            })
            .collect()
    }

    pub fn execute_succession(&mut self, heir_id: u64) -> Result<(), DynastyError> {
        if self.status != CampaignStatus::SuccessionPending {
            return Err(DynastyError::SuccessionNotReady);
        }
        let heir = self
            .characters
            .iter()
            .find(|character| {
                character.id == heir_id
                    && character.is_heir
                    && character.life_status == CharacterStatus::Alive
                    && character.age_on(self.game_date) >= 18
            })
            .ok_or(DynastyError::InvalidHeir)?;

        let old_head_name = self
            .characters
            .iter()
            .find(|c| c.id == self.dynasty.head_character_id)
            .map(|c| c.full_name.clone())
            .unwrap_or_default();
        let new_head_name = heir.full_name.clone();
        self.player_character_id = heir_id;
        if let Some(character) = self
            .characters
            .iter_mut()
            .find(|character| character.id == heir_id)
        {
            character.is_heir = false;
        }
        self.dynasty.head_character_id = heir_id;
        self.dynasty.prestige = (self.dynasty.prestige + 10).clamp(0, 100);
        self.status = CampaignStatus::Active;
        self.push_chronicle(
            "succession_completed",
            "Наследник возглавил династию",
            format!(
                "После ухода {} главой семьи стал {}.",
                old_head_name, new_head_name
            ),
        );
        Ok(())
    }

    pub fn standings(&self) -> Vec<Standing> {
        let mut rows = BTreeMap::<u64, Standing>::new();
        for club in &self.clubs {
            rows.insert(
                club.id,
                Standing {
                    team_id: club.id,
                    team_name: club.name.clone(),
                    played: 0,
                    wins: 0,
                    draws: 0,
                    losses: 0,
                    goals_for: 0,
                    goals_against: 0,
                    points: 0,
                },
            );
        }

        for fixture in self
            .fixtures
            .iter()
            .filter(|fixture| fixture.status == FixtureStatus::Played)
        {
            let (Some(home_goals), Some(away_goals)) = (fixture.home_goals, fixture.away_goals)
            else {
                continue;
            };
            {
                let Some(home) = rows.get_mut(&fixture.home_team_id) else {
                    continue;
                };
                home.played += 1;
                home.goals_for += i32::from(home_goals);
                home.goals_against += i32::from(away_goals);
                match home_goals.cmp(&away_goals) {
                    std::cmp::Ordering::Greater => {
                        home.wins += 1;
                        home.points += 3;
                    }
                    std::cmp::Ordering::Less => home.losses += 1,
                    std::cmp::Ordering::Equal => {
                        home.draws += 1;
                        home.points += 1;
                    }
                }
            }
            {
                let Some(away) = rows.get_mut(&fixture.away_team_id) else {
                    continue;
                };
                away.played += 1;
                away.goals_for += i32::from(away_goals);
                away.goals_against += i32::from(home_goals);
                match away_goals.cmp(&home_goals) {
                    std::cmp::Ordering::Greater => {
                        away.wins += 1;
                        away.points += 3;
                    }
                    std::cmp::Ordering::Less => away.losses += 1,
                    std::cmp::Ordering::Equal => {
                        away.draws += 1;
                        away.points += 1;
                    }
                }
            }
        }

        let mut standings: Vec<_> = rows.into_values().collect();
        standings.sort_by(|a, b| {
            b.points
                .cmp(&a.points)
                .then_with(|| b.goal_difference().cmp(&a.goal_difference()))
                .then_with(|| b.goals_for.cmp(&a.goals_for))
                .then_with(|| a.team_name.cmp(&b.team_name))
        });
        standings
    }

    fn check_founder_lifecycle(&mut self) {
        let head_id = self.player_character_id;
        let Some(index) = self
            .characters
            .iter()
            .position(|character| character.id == head_id)
        else {
            return;
        };
        if self.characters[index].life_status == CharacterStatus::Alive
            && self.characters[index].age_on(self.game_date) >= 90
        {
            self.characters[index].life_status = CharacterStatus::Deceased;
            let has_eligible_heir = !self.succession_candidates().is_empty();
            self.status = if has_eligible_heir {
                CampaignStatus::SuccessionPending
            } else {
                CampaignStatus::EndedNoHeir
            };
            let head_name = self.characters[index].full_name.clone();
            let (event_type, title) = if head_id == self.founder_character_id {
                ("founder_deceased", "Завершилась эпоха основателя")
            } else {
                ("dynasty_head_deceased", "Смена главы династии")
            };
            self.push_chronicle(
                event_type,
                title,
                format!("{head_name} ушёл из жизни. Семья готовится к следующему поколению."),
            );
        }
    }

    fn resolve_due_fixtures(&mut self) {
        let due_ids: Vec<u64> = self
            .fixtures
            .iter()
            .filter(|fixture| {
                fixture.status == FixtureStatus::Scheduled && fixture.match_date <= self.game_date
            })
            .map(|fixture| fixture.id)
            .collect();
        for fixture_id in due_ids {
            let Some(index) = self
                .fixtures
                .iter()
                .position(|fixture| fixture.id == fixture_id)
            else {
                continue;
            };
            let fixture = self.fixtures[index].clone();
            let Some((home_name, home_attack, home_defence)) = self
                .clubs
                .iter()
                .find(|club| club.id == fixture.home_team_id)
                .map(|club| (club.name.clone(), club.attack, club.defence))
            else {
                continue;
            };
            let Some((away_name, away_attack, away_defence)) = self
                .clubs
                .iter()
                .find(|club| club.id == fixture.away_team_id)
                .map(|club| (club.name.clone(), club.attack, club.defence))
            else {
                continue;
            };
            let home_strength = home_attack - away_defence;
            let away_strength = away_attack - home_defence;
            let home_roll = mix64(
                self.seed ^ fixture.id.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ fixture.home_team_id,
            );
            let away_roll = mix64(
                self.seed ^ fixture.id.wrapping_mul(0xD1B5_4A32_D192_ED03) ^ fixture.away_team_id,
            );
            let home_goals = ((home_roll % 4) as i32 + home_strength / 30 + 1).clamp(0, 5) as u8;
            let away_goals = ((away_roll % 4) as i32 + away_strength / 30).clamp(0, 5) as u8;
            self.fixtures[index].status = FixtureStatus::Played;
            self.fixtures[index].home_goals = Some(home_goals);
            self.fixtures[index].away_goals = Some(away_goals);
            self.push_chronicle(
                "fixture_completed",
                "Матч завершён",
                format!(
                    "{} {}:{} {} · игровой день {}",
                    home_name, home_goals, away_goals, away_name, fixture.matchday
                ),
            );
        }
    }

    fn expire_events(&mut self) {
        for event in &mut self.events {
            if event.status == EventStatus::Pending && self.game_date > event.expires_on {
                event.status = EventStatus::Expired;
            }
        }
    }

    fn generate_weekly_event(&mut self, week: u32) {
        self.generated_weeks.push(week);
        let (event_type, title, body, choices) = match week % 4 {
            1 => (
                "club_weekly_review",
                "Еженедельное заседание совета",
                "Руководство клуба оценивает результаты и предлагает определить главный приоритет недели.",
                vec![
                    choice(
                        "reputation",
                        "Инвестировать внимание в спортивную репутацию",
                        "club_reputation",
                    ),
                    choice(
                        "prestige",
                        "Укрепить позиции и авторитет семьи",
                        "dynasty_prestige",
                    ),
                ],
            ),
            2 => (
                "sponsor_offer",
                "Предложение спонсора",
                "Региональная компания готова заключить контракт с клубом на текущий сезон.",
                vec![
                    choice("accept", "Принять контракт на €250 000", "sponsor_offer"),
                    choice(
                        "decline",
                        "Отказаться и сохранить независимость",
                        "dynasty_prestige",
                    ),
                ],
            ),
            3 => (
                "supporter_meeting",
                "Встреча с болельщиками",
                "Представители трибун хотят обсудить развитие стадиона и отношения с клубом.",
                vec![
                    choice(
                        "community",
                        "Поддержать программу для болельщиков",
                        "club_reputation",
                    ),
                    choice(
                        "legacy",
                        "Создать семейный фонд истории клуба",
                        "dynasty_prestige",
                    ),
                ],
            ),
            _ => (
                "rival_scouting",
                "Доклад о конкурентах",
                "Спортивный директор подготовил обзор соперников и возможностей укрепить положение клуба.",
                vec![
                    choice("academy", "Усилить спортивную структуру", "club_reputation"),
                    choice("network", "Расширить влияние династии", "dynasty_prestige"),
                ],
            ),
        };
        self.push_event(event_type, title, body, 75, choices, 7);
    }

    fn push_event(
        &mut self,
        event_type: &str,
        title: &str,
        body: &str,
        priority: u8,
        choices: Vec<EventChoice>,
        lifetime_days: i64,
    ) {
        let id = self.allocate_id();
        self.events.push(GameEvent {
            id,
            event_type: event_type.to_owned(),
            title: title.to_owned(),
            body: body.to_owned(),
            game_date: self.game_date,
            expires_on: self.game_date + Duration::days(lifetime_days),
            priority,
            choices,
            status: EventStatus::Pending,
            resolution: None,
        });
    }

    fn push_chronicle(&mut self, event_type: &str, title: &str, detail: String) {
        let id = self.allocate_id();
        self.chronicle.push(ChronicleEntry {
            id,
            event_type: event_type.to_owned(),
            title: title.to_owned(),
            game_date: self.game_date,
            detail,
        });
    }

    fn post_journal(
        &mut self,
        key: &str,
        memo: &str,
        lines: Vec<LedgerLine>,
    ) -> Result<(), DynastyError> {
        if self.ledger.iter().any(|entry| entry.idempotency_key == key) {
            return Err(DynastyError::DuplicateTransaction);
        }
        let debits: i64 = lines
            .iter()
            .filter(|line| line.side == "debit")
            .map(|line| line.amount_minor)
            .sum();
        let credits: i64 = lines
            .iter()
            .filter(|line| line.side == "credit")
            .map(|line| line.amount_minor)
            .sum();
        if debits <= 0
            || debits != credits
            || lines.iter().any(|line| {
                line.amount_minor <= 0 || !matches!(line.side.as_str(), "debit" | "credit")
            })
        {
            return Err(DynastyError::JournalUnbalanced);
        }
        let id = self.allocate_id();
        self.ledger.push(LedgerEntry {
            id,
            idempotency_key: key.to_owned(),
            memo: memo.to_owned(),
            game_date: self.game_date,
            currency: "EUR".to_owned(),
            lines,
        });
        Ok(())
    }

    fn allocate_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        id
    }
}

fn validate_name(value: &str, max_chars: usize, field: &'static str) -> Result<(), DynastyError> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.chars().count() > max_chars {
        Err(DynastyError::InvalidName(field))
    } else {
        Ok(())
    }
}

fn choice(id: &str, label: &str, effect: &str) -> EventChoice {
    EventChoice {
        id: id.to_owned(),
        label: label.to_owned(),
        effect: effect.to_owned(),
    }
}

fn origin_skill(origin: Origin, skill: usize) -> u8 {
    let base = match origin {
        Origin::FootballFamily => [72, 58, 52, 68, 60, 63],
        Origin::Entrepreneur => [55, 67, 82, 48, 74, 58],
        Origin::FormerPlayer => [52, 51, 45, 84, 57, 70],
        Origin::Agent => [70, 58, 60, 61, 88, 62],
        Origin::Executive => [63, 86, 75, 60, 68, 65],
        Origin::Newcomer => [50, 50, 50, 50, 50, 50],
    };
    base[skill]
}

fn next_saturday(date: NaiveDate) -> NaiveDate {
    let days = (5_i64 - i64::from(date.weekday().num_days_from_monday())).rem_euclid(7);
    date + Duration::days(days)
}

fn generate_schedule(
    clubs: &[Club],
    first_date: NaiveDate,
    season_year: i32,
    next_id: &mut u64,
) -> Vec<Fixture> {
    let mut rotation: Vec<Option<u64>> = clubs.iter().map(|club| Some(club.id)).collect();
    if rotation.len() % 2 == 1 {
        rotation.push(None);
    }
    let first_leg_rounds = rotation.len() - 1;
    let mut first_leg = Vec::new();
    for round in 0..first_leg_rounds {
        let match_date = first_date + Duration::days((round * 7) as i64);
        for pair in 0..rotation.len() / 2 {
            let left = rotation[pair];
            let right = rotation[rotation.len() - 1 - pair];
            if let (Some(a), Some(b)) = (left, right) {
                let (home, away) = if (round + pair) % 2 == 0 {
                    (a, b)
                } else {
                    (b, a)
                };
                first_leg.push(Fixture {
                    id: *next_id,
                    season_year,
                    matchday: (round + 1) as u8,
                    match_date,
                    home_team_id: home,
                    away_team_id: away,
                    status: FixtureStatus::Scheduled,
                    home_goals: None,
                    away_goals: None,
                });
                *next_id = (*next_id).saturating_add(1);
            }
        }
        let last = rotation.pop().expect("rotation has members");
        rotation.insert(1, last);
    }

    let mut fixtures = first_leg.clone();
    for fixture in first_leg {
        let next_id_value = *next_id;
        fixtures.push(Fixture {
            id: next_id_value,
            season_year,
            matchday: (fixture.matchday as usize + first_leg_rounds) as u8,
            match_date: fixture.match_date + Duration::days((first_leg_rounds * 7) as i64),
            home_team_id: fixture.away_team_id,
            away_team_id: fixture.home_team_id,
            status: FixtureStatus::Scheduled,
            home_goals: None,
            away_goals: None,
        });
        *next_id = (*next_id).saturating_add(1);
    }
    fixtures
}

fn mix64(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9E37_79B9_7F4A_7C15);
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^ (value >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> CampaignSetup {
        CampaignSetup {
            name: "Первая кампания".to_owned(),
            mode: CampaignMode::Sandbox,
            founder_name: "Александр Орлов".to_owned(),
            dynasty_name: "Дом Орловых".to_owned(),
            club_name: "Northbridge FC".to_owned(),
            origin: Origin::Entrepreneur,
        }
    }

    fn campaign() -> Campaign {
        Campaign::new(setup(), 42).expect("campaign should be valid")
    }

    #[test]
    fn campaign_starts_with_fictional_world_and_balanced_opening_ledger() {
        let campaign = campaign();
        assert_eq!(campaign.clubs.len(), 4);
        assert_eq!(campaign.fixtures.len(), 12);
        assert_eq!(
            campaign
                .current_character()
                .unwrap()
                .age_on(campaign.game_date),
            40
        );
        assert_eq!(campaign.cash_balance(FOUNDER_CASH_ACCOUNT), 2_500_000_000);
        assert_eq!(campaign.cash_balance(CLUB_CASH_ACCOUNT), 1_000_000_000);
        let opening = &campaign.ledger[0];
        let debit: i64 = opening
            .lines
            .iter()
            .filter(|l| l.side == "debit")
            .map(|l| l.amount_minor)
            .sum();
        let credit: i64 = opening
            .lines
            .iter()
            .filter(|l| l.side == "credit")
            .map(|l| l.amount_minor)
            .sum();
        assert_eq!(debit, credit);
    }

    #[test]
    fn deterministic_fixture_results_and_table_reconcile() {
        let mut first = campaign();
        let mut second = campaign();
        first.advance_days(31).unwrap();
        second.advance_days(31).unwrap();
        assert_eq!(first.fixtures, second.fixtures);
        let played = first
            .fixtures
            .iter()
            .filter(|fixture| fixture.status == FixtureStatus::Played)
            .count();
        assert_eq!(played, 10);
        let standings = first.standings();
        assert_eq!(
            standings
                .iter()
                .map(|row| row.played as usize)
                .sum::<usize>(),
            played * 2
        );
        assert_eq!(
            standings.iter().map(|row| row.points).sum::<u16>(),
            standings
                .iter()
                .map(|row| (row.wins * 3) + row.draws)
                .sum::<u16>()
        );
    }

    #[test]
    fn one_team_plays_no_more_than_once_per_matchday() {
        let campaign = campaign();
        for matchday in 1..=6 {
            let fixtures: Vec<_> = campaign
                .fixtures
                .iter()
                .filter(|fixture| fixture.matchday == matchday)
                .collect();
            let participants: Vec<_> = fixtures
                .iter()
                .flat_map(|fixture| [fixture.home_team_id, fixture.away_team_id])
                .collect();
            let mut unique = participants.clone();
            unique.sort_unstable();
            unique.dedup();
            assert_eq!(participants.len(), unique.len());
            assert_eq!(unique.len(), 4);
        }
    }

    #[test]
    fn event_resolution_changes_state_once_and_records_chronicle() {
        let mut campaign = campaign();
        let event = campaign.events[0].clone();
        campaign.resolve_event(event.id, "reputation").unwrap();
        assert_eq!(campaign.controlled_club().reputation, 40);
        assert_eq!(campaign.events[0].status, EventStatus::Resolved);
        assert!(
            campaign
                .chronicle
                .iter()
                .any(|entry| entry.event_type == "event_resolved")
        );
        assert_eq!(
            campaign.resolve_event(event.id, "reputation"),
            Err(DynastyError::EventExpired)
        );
    }

    #[test]
    fn sponsor_offer_posts_a_balanced_idempotent_ledger_transaction() {
        let mut campaign = campaign();
        let event_id = campaign.push_event_for_test();
        let before = campaign.cash_balance(CLUB_CASH_ACCOUNT);
        campaign.resolve_event(event_id, "accept").unwrap();
        assert_eq!(
            campaign.cash_balance(CLUB_CASH_ACCOUNT),
            before + 25_000_000
        );
        assert_eq!(campaign.ledger.len(), 2);
        let entry = campaign.ledger.last().unwrap();
        let debit: i64 = entry
            .lines
            .iter()
            .filter(|line| line.side == "debit")
            .map(|line| line.amount_minor)
            .sum();
        let credit: i64 = entry
            .lines
            .iter()
            .filter(|line| line.side == "credit")
            .map(|line| line.amount_minor)
            .sum();
        assert_eq!(debit, credit);
    }

    #[test]
    fn heir_is_persisted_and_must_be_adult_before_succession() {
        let mut campaign = campaign();
        let heir_id = campaign.add_heir("Мария Орлова", 12).unwrap();
        assert_eq!(campaign.succession_candidates().len(), 0);
        let heir = campaign
            .characters
            .iter()
            .find(|character| character.id == heir_id)
            .unwrap();
        assert_eq!(
            heir.parent_character_id,
            Some(campaign.founder_character_id)
        );
        assert_eq!(heir.age_on(campaign.game_date), 12);
    }

    #[test]
    fn founder_death_and_succession_transfer_campaign_control_once() {
        let mut campaign = campaign();
        let heir_id = campaign.add_heir("Мария Орлова", 25).unwrap();
        let founder_index = campaign
            .characters
            .iter()
            .position(|c| c.id == campaign.founder_character_id)
            .unwrap();
        let founder_birth = campaign.characters[founder_index].birth_date;
        campaign.game_date = NaiveDate::from_ymd_opt(
            founder_birth.year() + 90,
            founder_birth.month(),
            founder_birth.day(),
        )
        .unwrap();
        campaign.advance_days(1).unwrap();
        assert_eq!(campaign.status, CampaignStatus::SuccessionPending);
        assert_eq!(
            campaign
                .succession_candidates()
                .iter()
                .map(|c| c.id)
                .collect::<Vec<_>>(),
            vec![heir_id]
        );
        campaign.execute_succession(heir_id).unwrap();
        assert_eq!(campaign.player_character_id, heir_id);
        assert_eq!(campaign.dynasty.head_character_id, heir_id);
        assert_eq!(campaign.status, CampaignStatus::Active);
    }

    #[test]
    fn succession_can_continue_across_more_than_one_generation() {
        let mut campaign = campaign();
        let first_heir = campaign.add_heir("Мария Орлова", 25).unwrap();
        let first_birth = campaign
            .characters
            .iter()
            .find(|character| character.id == campaign.founder_character_id)
            .unwrap()
            .birth_date;
        campaign.game_date = NaiveDate::from_ymd_opt(
            first_birth.year() + 90,
            first_birth.month(),
            first_birth.day(),
        )
        .unwrap();
        campaign.advance_days(1).unwrap();
        campaign.execute_succession(first_heir).unwrap();

        let second_heir = campaign.add_heir("Елена Орлова", 25).unwrap();
        assert_eq!(
            campaign
                .characters
                .iter()
                .find(|character| character.id == second_heir)
                .unwrap()
                .parent_character_id,
            Some(first_heir)
        );
        let second_birth = campaign
            .characters
            .iter()
            .find(|character| character.id == first_heir)
            .unwrap()
            .birth_date;
        campaign.game_date = NaiveDate::from_ymd_opt(
            second_birth.year() + 90,
            second_birth.month(),
            second_birth.day(),
        )
        .unwrap();
        campaign.advance_days(1).unwrap();
        assert_eq!(campaign.status, CampaignStatus::SuccessionPending);
        campaign.execute_succession(second_heir).unwrap();
        assert_eq!(campaign.dynasty.head_character_id, second_heir);
        assert_eq!(campaign.player_character_id, second_heir);
    }

    #[test]
    fn no_eligible_heir_ends_campaign_at_founder_death() {
        let mut campaign = campaign();
        let founder_index = campaign
            .characters
            .iter()
            .position(|c| c.id == campaign.founder_character_id)
            .unwrap();
        let founder_birth = campaign.characters[founder_index].birth_date;
        campaign.game_date = NaiveDate::from_ymd_opt(
            founder_birth.year() + 90,
            founder_birth.month(),
            founder_birth.day(),
        )
        .unwrap();
        campaign.advance_days(1).unwrap();
        assert_eq!(campaign.status, CampaignStatus::EndedNoHeir);
    }

    #[test]
    fn serialization_round_trip_preserves_game_state() {
        let mut campaign = campaign();
        campaign.add_heir("Мария Орлова", 18).unwrap();
        campaign.advance_days(15).unwrap();
        let encoded = serde_json::to_string(&campaign).unwrap();
        let restored: Campaign = serde_json::from_str(&encoded).unwrap();
        assert_eq!(campaign, restored);
        restored_still_uses_same_seed_and_campaign_identity(&restored);
    }

    fn restored_still_uses_same_seed_and_campaign_identity(campaign: &Campaign) {
        assert_eq!(campaign.seed, 42);
        assert_eq!(campaign.engine_version, ENGINE_VERSION);
        assert_eq!(campaign.name, "Первая кампания");
    }

    impl Campaign {
        fn push_event_for_test(&mut self) -> u64 {
            self.push_event(
                "sponsor_offer",
                "Предложение спонсора",
                "Тестовое предложение",
                75,
                vec![choice("accept", "Принять", "sponsor_offer")],
                7,
            );
            self.events.last().unwrap().id
        }
    }
}

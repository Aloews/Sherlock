use serde::{Deserialize, Serialize};
use std::fmt;

const STARTING_CASH: i64 = 2_500_000;
const MAX_SQUAD_SIZE: usize = 30;
const MIN_SQUAD_SIZE: usize = 18;
const MATCHDAYS_PER_SEASON: u8 = 6;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Position {
    Goalkeeper,
    Defender,
    Midfielder,
    Forward,
}

impl Position {
    pub fn label(self) -> &'static str {
        match self {
            Self::Goalkeeper => "Вратарь",
            Self::Defender => "Защитник",
            Self::Midfielder => "Полузащитник",
            Self::Forward => "Нападающий",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Attributes {
    pub pace: u8,
    pub shooting: u8,
    pub passing: u8,
    pub defending: u8,
    pub physical: u8,
    pub goalkeeping: u8,
    pub stamina: u8,
}

impl Attributes {
    fn average(values: &[u8]) -> u8 {
        (values.iter().map(|value| u16::from(*value)).sum::<u16>() / values.len() as u16) as u8
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub position: Position,
    pub age: u8,
    pub attributes: Attributes,
    pub potential: u8,
    pub fitness: u8,
    pub morale: u8,
    pub injury_days: u8,
    pub suspension_matches: u8,
    pub contract_days: u16,
    pub wage: u32,
    pub appearances: u16,
    pub goals: u16,
    pub assists: u16,
}

impl Player {
    pub fn overall(&self) -> u8 {
        let a = self.attributes;
        match self.position {
            Position::Goalkeeper => Attributes::average(&[a.goalkeeping, a.defending, a.passing, a.physical]),
            Position::Defender => Attributes::average(&[a.defending, a.physical, a.pace, a.passing]),
            Position::Midfielder => Attributes::average(&[a.passing, a.stamina, a.pace, a.shooting]),
            Position::Forward => Attributes::average(&[a.shooting, a.pace, a.physical, a.passing]),
        }
    }

    pub fn market_value(&self) -> u32 {
        let rating = u32::from(self.overall());
        let age_factor = if self.age <= 23 { 135 } else if self.age <= 28 { 100 } else { 65 };
        rating.saturating_mul(rating).saturating_mul(age_factor) / 10
    }

    fn train(&mut self, focus: TrainingFocus, facility_level: u8) -> bool {
        if self.injury_days > 0 || self.suspension_matches > 0 {
            self.fitness = self.fitness.saturating_add(1).min(100);
            return false;
        }

        let before = self.overall();
        let boost = if facility_level >= 4 { 2 } else { 1 };
        let attrs = &mut self.attributes;
        let target = match focus {
            TrainingFocus::Balanced => match self.position {
                Position::Goalkeeper => &mut attrs.goalkeeping,
                Position::Defender => &mut attrs.defending,
                Position::Midfielder => &mut attrs.passing,
                Position::Forward => &mut attrs.shooting,
            },
            TrainingFocus::Attack => &mut attrs.shooting,
            TrainingFocus::Defence => &mut attrs.defending,
            TrainingFocus::Fitness => &mut attrs.stamina,
        };
        *target = target.saturating_add(boost).min(self.potential);
        self.fitness = self.fitness.saturating_sub(5);
        self.morale = self.morale.saturating_add(1).min(100);
        self.overall() > before
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Formation {
    FourThreeThree,
    FourFourTwo,
    ThreeFiveTwo,
}

impl Formation {
    pub fn label(self) -> &'static str {
        match self {
            Self::FourThreeThree => "4–3–3",
            Self::FourFourTwo => "4–4–2",
            Self::ThreeFiveTwo => "3–5–2",
        }
    }

    fn counts(self) -> (usize, usize, usize) {
        match self {
            Self::FourThreeThree => (4, 3, 3),
            Self::FourFourTwo => (4, 4, 2),
            Self::ThreeFiveTwo => (3, 5, 2),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Tactics {
    pub formation: Formation,
    pub pressing: u8,
    pub tempo: u8,
    pub width: u8,
    pub attacking_intent: u8,
}

impl Default for Tactics {
    fn default() -> Self {
        Self {
            formation: Formation::FourThreeThree,
            pressing: 55,
            tempo: 55,
            width: 50,
            attacking_intent: 55,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Heir {
    pub id: u32,
    pub name: String,
    pub age: u8,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct DynastyClub {
    pub name: String,
    pub cash: i64,
    pub reputation: u8,
    pub training_facility: u8,
    pub tactics: Tactics,
    pub players: Vec<Player>,
    pub starting_xi: Vec<u32>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Score {
    pub home: u8,
    pub away: u8,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Fixture {
    pub id: u32,
    pub matchday: u8,
    pub home_team: usize,
    pub away_team: usize,
    pub score: Option<Score>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Standing {
    pub team: String,
    pub played: u8,
    pub wins: u8,
    pub draws: u8,
    pub losses: u8,
    pub goals_for: u16,
    pub goals_against: u16,
    pub points: u16,
}

impl Standing {
    pub fn goal_difference(&self) -> i16 {
        self.goals_for as i16 - self.goals_against as i16
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct LedgerEntry {
    pub game_day: u32,
    pub description: String,
    pub delta: i64,
    pub balance_after: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ChronicleEntry {
    pub game_day: u32,
    pub title: String,
    pub detail: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct TransferTarget {
    pub player: Player,
    pub asking_price: u32,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TrainingFocus {
    Balanced,
    Attack,
    Defence,
    Fitness,
}

impl TrainingFocus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Balanced => "Баланс",
            Self::Attack => "Атака",
            Self::Defence => "Оборона",
            Self::Fitness => "Физика",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Campaign {
    pub manager_name: String,
    pub manager_age: u8,
    pub retired_managers: Vec<String>,
    pub heirs: Vec<Heir>,
    pub game_day: u32,
    pub season: u16,
    pub matchday: u8,
    pub seed: u64,
    pub club: DynastyClub,
    pub opponents: Vec<String>,
    pub fixtures: Vec<Fixture>,
    pub transfer_market: Vec<TransferTarget>,
    pub chronicle: Vec<ChronicleEntry>,
    pub ledger: Vec<LedgerEntry>,
    pub last_training_day: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DynastyError {
    InvalidName,
    InvalidAge,
    SquadTooSmall,
    SquadTooLarge,
    PlayerNotFound,
    PlayerUnavailable,
    PlayerAlreadySelected,
    InvalidLineup,
    TrainingAlreadyUsed,
    InsufficientFunds,
    MarketTargetNotFound,
    TransferWindowFull,
    CannotSellStarter,
    CannotSellBelowMinimum,
    HeirNotFound,
    HeirNotAdult,
    SeasonComplete,
    InvalidSave,
}

impl fmt::Display for DynastyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidName => "Укажите имя и название клуба",
            Self::InvalidAge => "Возраст должен быть от 16 до 100 лет",
            Self::SquadTooSmall => "В команде недостаточно игроков",
            Self::SquadTooLarge => "В команде достигнут лимит игроков",
            Self::PlayerNotFound => "Игрок не найден",
            Self::PlayerUnavailable => "Игрок временно недоступен",
            Self::PlayerAlreadySelected => "Игрок уже находится в стартовом составе",
            Self::InvalidLineup => "Стартовый состав не соответствует выбранной схеме",
            Self::TrainingAlreadyUsed => "Сегодня тренировка уже проводилась",
            Self::InsufficientFunds => "Недостаточно средств клуба",
            Self::MarketTargetNotFound => "Предложение на трансфер больше недоступно",
            Self::TransferWindowFull => "В составе достигнут лимит в 30 игроков",
            Self::CannotSellStarter => "Нельзя продать игрока из стартового состава",
            Self::CannotSellBelowMinimum => "Нельзя сократить состав ниже 18 игроков",
            Self::HeirNotFound => "Наследник не найден",
            Self::HeirNotAdult => "Наследник должен быть не младше 18 лет",
            Self::SeasonComplete => "Сезон уже завершён",
            Self::InvalidSave => "Сохранение повреждено или несовместимо",
        };
        write!(f, "{message}")
    }
}

impl std::error::Error for DynastyError {}

impl Campaign {
    pub fn new(manager_name: &str, club_name: &str, seed: u64) -> Result<Self, DynastyError> {
        let manager_name = clean_name(manager_name);
        let club_name = clean_name(club_name);
        if manager_name.is_empty() || club_name.is_empty() {
            return Err(DynastyError::InvalidName);
        }

        let mut random = seed.max(1);
        let positions = [
            Position::Goalkeeper, Position::Goalkeeper,
            Position::Defender, Position::Defender, Position::Defender, Position::Defender,
            Position::Defender, Position::Defender, Position::Defender,
            Position::Midfielder, Position::Midfielder, Position::Midfielder, Position::Midfielder,
            Position::Midfielder, Position::Midfielder, Position::Midfielder,
            Position::Forward, Position::Forward, Position::Forward, Position::Forward,
            Position::Forward, Position::Forward,
        ];
        let names = [
            "Марк Веллин", "Илья Ровен", "Нико Арден", "Леон Вест", "Данте Морен",
            "Эрик Сольберг", "Алекс Ренар", "Томас Вейл", "Роман Келлер", "Матео Сорен",
            "Лука Эстен", "Кирилл Норден", "Ян Берг", "Милан Роше", "Оскар Леван",
            "Даниэль Коста", "Эмиль Фарен", "Тео Марек", "Рафаэль Дорн", "Саша Лорен",
            "Адриан Вальс", "Макс Ривер",
        ];
        let mut players = Vec::with_capacity(positions.len());
        for (index, position) in positions.into_iter().enumerate() {
            players.push(make_player(index as u32 + 1, names[index], position, 20 + (next(&mut random) % 12) as u8, 54 + (next(&mut random) % 20) as u8, &mut random));
        }

        let opponents = vec![
            "Атлетик Север".to_string(),
            "Риверпорт".to_string(),
            "ФК Вектор".to_string(),
        ];
        let mut campaign = Self {
            manager_name,
            manager_age: 40,
            retired_managers: Vec::new(),
            heirs: Vec::new(),
            game_day: 1,
            season: 1,
            matchday: 1,
            seed: random,
            club: DynastyClub {
                name: club_name,
                cash: STARTING_CASH,
                reputation: 42,
                training_facility: 1,
                tactics: Tactics::default(),
                players,
                starting_xi: Vec::new(),
            },
            opponents,
            fixtures: Vec::new(),
            transfer_market: Vec::new(),
            chronicle: Vec::new(),
            ledger: Vec::new(),
            last_training_day: None,
        };
        campaign.club.starting_xi = best_lineup(&campaign.club.players, campaign.club.tactics.formation);
        campaign.fixtures = season_fixtures();
        campaign.transfer_market = generate_market(&mut random);
        campaign.seed = random;
        campaign.ledger.push(LedgerEntry {
            game_day: 1,
            description: "Стартовый капитал династии".into(),
            delta: STARTING_CASH,
            balance_after: STARTING_CASH,
        });
        campaign.chronicle.push(ChronicleEntry {
            game_day: 1,
            title: "Начало новой династии".into(),
            detail: format!("{} возглавил клуб {}.", campaign.manager_name, campaign.club.name),
        });
        Ok(campaign)
    }

    pub fn add_heir(&mut self, name: &str, age: u8) -> Result<(), DynastyError> {
        let name = clean_name(name);
        if name.is_empty() {
            return Err(DynastyError::InvalidName);
        }
        if !(0..=100).contains(&age) {
            return Err(DynastyError::InvalidAge);
        }
        let id = self.heirs.iter().map(|heir| heir.id).max().unwrap_or(0) + 1;
        self.heirs.push(Heir { id, name: name.clone(), age });
        self.record_event("Новый наследник", format!("{name} включён в семейную династию."));
        Ok(())
    }

    pub fn is_starter(&self, player_id: u32) -> bool {
        self.club.starting_xi.contains(&player_id)
    }

    pub fn set_formation(&mut self, formation: Formation) {
        self.club.tactics.formation = formation;
        self.club.starting_xi = best_lineup(&self.club.players, formation);
        self.record_event("Изменение тактики", format!("Выбрана схема {}.", formation.label()));
    }

    pub fn select_player(&mut self, player_id: u32) -> Result<(), DynastyError> {
        let candidate = self.club.players.iter().find(|player| player.id == player_id)
            .ok_or(DynastyError::PlayerNotFound)?;
        if candidate.injury_days > 0 || candidate.suspension_matches > 0 {
            return Err(DynastyError::PlayerUnavailable);
        }
        if self.is_starter(player_id) {
            return Err(DynastyError::PlayerAlreadySelected);
        }
        let position = candidate.position;
        let (_, required_defenders, required_midfielders, required_forwards) = lineup_requirements(self.club.tactics.formation);
        let required = match position {
            Position::Goalkeeper => 1,
            Position::Defender => required_defenders,
            Position::Midfielder => required_midfielders,
            Position::Forward => required_forwards,
        };
        let selected_same_position = self.club.starting_xi.iter()
            .filter_map(|id| self.club.players.iter().find(|player| player.id == *id))
            .filter(|player| player.position == position)
            .count();
        if selected_same_position >= required {
            let weakest = self.club.starting_xi.iter().filter_map(|id| self.club.players.iter().find(|player| player.id == *id))
                .filter(|player| player.position == position)
                .min_by_key(|player| player.overall())
                .map(|player| player.id);
            if let Some(weakest_id) = weakest {
                if let Some(index) = self.club.starting_xi.iter().position(|id| *id == weakest_id) {
                    self.club.starting_xi[index] = player_id;
                }
            }
        } else {
            self.club.starting_xi.push(player_id);
        }
        Ok(())
    }

    pub fn train(&mut self, focus: TrainingFocus) -> Result<u8, DynastyError> {
        if self.last_training_day == Some(self.game_day) {
            return Err(DynastyError::TrainingAlreadyUsed);
        }
        let cost = 8_000_i64 * i64::from(self.club.training_facility);
        if self.club.cash < cost {
            return Err(DynastyError::InsufficientFunds);
        }
        self.game_day += 1;
        self.last_training_day = Some(self.game_day);
        self.change_balance(-cost, format!("Тренировка: {}", focus.label()));
        let mut improved = 0;
        let facility_level = self.club.training_facility;
        for player in &mut self.club.players {
            if player.train(focus, facility_level) {
                improved += 1;
            }
            player.injury_days = player.injury_days.saturating_sub(1);
            player.contract_days = player.contract_days.saturating_sub(1);
        }
        for heir in &mut self.heirs {
            if self.game_day % 365 == 0 {
                heir.age = heir.age.saturating_add(1);
            }
        }
        self.record_event("Тренировочный день", format!("Проведено занятие «{}». Изменение общего рейтинга получили {improved} игроков.", focus.label()));
        Ok(improved)
    }

    pub fn upgrade_training_facility(&mut self) -> Result<(), DynastyError> {
        if self.club.training_facility >= 5 {
            return Err(DynastyError::InvalidSave);
        }
        let cost = i64::from(self.club.training_facility) * 350_000;
        if self.club.cash < cost {
            return Err(DynastyError::InsufficientFunds);
        }
        self.club.training_facility += 1;
        self.change_balance(-cost, "Модернизация тренировочной базы".into());
        self.record_event("Новая тренировочная база", format!("Уровень базы повышен до {}.", self.club.training_facility));
        Ok(())
    }

    pub fn sign_player(&mut self, player_id: u32) -> Result<(), DynastyError> {
        if self.club.players.len() >= MAX_SQUAD_SIZE {
            return Err(DynastyError::TransferWindowFull);
        }
        let index = self.transfer_market.iter().position(|target| target.player.id == player_id)
            .ok_or(DynastyError::MarketTargetNotFound)?;
        let target = self.transfer_market[index].clone();
        if self.club.cash < i64::from(target.asking_price) {
            return Err(DynastyError::InsufficientFunds);
        }
        self.transfer_market.remove(index);
        self.club.players.push(target.player.clone());
        self.change_balance(-i64::from(target.asking_price), format!("Трансфер: {}", target.player.name));
        self.record_event("Подписан контракт", format!("{} перешёл в {}.", target.player.name, self.club.name));
        Ok(())
    }

    pub fn sell_player(&mut self, player_id: u32) -> Result<u32, DynastyError> {
        if self.is_starter(player_id) {
            return Err(DynastyError::CannotSellStarter);
        }
        if self.club.players.len() <= MIN_SQUAD_SIZE {
            return Err(DynastyError::CannotSellBelowMinimum);
        }
        let index = self.club.players.iter().position(|player| player.id == player_id)
            .ok_or(DynastyError::PlayerNotFound)?;
        let player = self.club.players.remove(index);
        let price = player.market_value().saturating_mul(75) / 100;
        self.change_balance(i64::from(price), format!("Продажа игрока: {}", player.name));
        self.record_event("Трансфер завершён", format!("{} покинул клуб за {} €.", player.name, price));
        Ok(price)
    }

    pub fn retire_to_heir(&mut self, heir_id: u32) -> Result<(), DynastyError> {
        let index = self.heirs.iter().position(|heir| heir.id == heir_id).ok_or(DynastyError::HeirNotFound)?;
        let heir = self.heirs[index].clone();
        if heir.age < 18 {
            return Err(DynastyError::HeirNotAdult);
        }
        self.retired_managers.push(self.manager_name.clone());
        self.manager_name = heir.name.clone();
        self.manager_age = heir.age;
        self.heirs.remove(index);
        self.club.reputation = self.club.reputation.saturating_add(2).min(100);
        self.record_event("Смена поколения", format!("{} продолжил управление династией после {}.", self.manager_name, self.retired_managers.last().map(String::as_str).unwrap_or("предшественника")));
        Ok(())
    }

    pub fn advance_matchday(&mut self) -> Result<Vec<(String, Score)>, DynastyError> {
        if self.matchday > MATCHDAYS_PER_SEASON {
            return Err(DynastyError::SeasonComplete);
        }
        if self.club.starting_xi.len() != 11 {
            return Err(DynastyError::InvalidLineup);
        }
        let fixtures: Vec<Fixture> = self.fixtures.iter()
            .filter(|fixture| fixture.matchday == self.matchday && fixture.score.is_none())
            .cloned()
            .collect();
        if fixtures.is_empty() {
            return Err(DynastyError::SeasonComplete);
        }

        let mut played = Vec::with_capacity(fixtures.len());
        for fixture in fixtures {
            let home_rating = self.team_strength(fixture.home_team);
            let away_rating = self.team_strength(fixture.away_team);
            let score = simulate_score(self.seed, fixture.id, home_rating, away_rating);
            if let Some(stored) = self.fixtures.iter_mut().find(|stored| stored.id == fixture.id) {
                stored.score = Some(score);
            }
            if fixture.home_team == 0 || fixture.away_team == 0 {
                self.apply_user_match(&fixture, score);
            }
            played.push((format!("{} — {}", self.team_name(fixture.home_team), self.team_name(fixture.away_team)), score));
        }

        self.game_day += 7;
        self.pay_weekly_wages();
        self.club.reputation = (i16::from(self.club.reputation) + 1).clamp(0, 100) as u8;
        let user_fixture = played.iter().find(|(title, _)| title.contains(&self.club.name)).cloned();
        if let Some((title, score)) = user_fixture {
            self.record_event("Матч завершён", format!("{title}: {}:{}. Турнирная таблица обновлена.", score.home, score.away));
        }
        self.record_event("Обзор совета директоров", "Совет оценил результаты клуба и финансовое положение.".into());

        if self.matchday == MATCHDAYS_PER_SEASON {
            let champion = self.standings().first().map(|row| row.team.clone()).unwrap_or_default();
            if champion == self.club.name {
                self.change_balance(300_000, "Призовые за победу в чемпионате".into());
                self.club.reputation = self.club.reputation.saturating_add(8).min(100);
            }
            self.record_event("Сезон завершён", format!("Чемпион сезона {}: {champion}.", self.season));
            self.season += 1;
            self.matchday = 1;
            for player in &mut self.club.players {
                player.age = player.age.saturating_add(1);
                player.fitness = player.fitness.saturating_add(20).min(100);
                player.contract_days = player.contract_days.saturating_sub(7);
            }
            for heir in &mut self.heirs {
                heir.age = heir.age.saturating_add(1);
            }
            self.fixtures = season_fixtures();
            self.transfer_market = generate_market(&mut self.seed);
        } else {
            self.matchday += 1;
        }
        self.seed = next(&mut self.seed);
        Ok(played)
    }

    pub fn standings(&self) -> Vec<Standing> {
        let mut rows: Vec<Standing> = (0..4).map(|index| Standing {
            team: self.team_name(index).to_string(),
            played: 0,
            wins: 0,
            draws: 0,
            losses: 0,
            goals_for: 0,
            goals_against: 0,
            points: 0,
        }).collect();

        for fixture in &self.fixtures {
            let Some(score) = fixture.score else { continue };
            rows[fixture.home_team].played += 1;
            rows[fixture.away_team].played += 1;
            rows[fixture.home_team].goals_for += u16::from(score.home);
            rows[fixture.home_team].goals_against += u16::from(score.away);
            rows[fixture.away_team].goals_for += u16::from(score.away);
            rows[fixture.away_team].goals_against += u16::from(score.home);
            match score.home.cmp(&score.away) {
                std::cmp::Ordering::Greater => {
                    rows[fixture.home_team].wins += 1;
                    rows[fixture.home_team].points += 3;
                    rows[fixture.away_team].losses += 1;
                }
                std::cmp::Ordering::Less => {
                    rows[fixture.away_team].wins += 1;
                    rows[fixture.away_team].points += 3;
                    rows[fixture.home_team].losses += 1;
                }
                std::cmp::Ordering::Equal => {
                    rows[fixture.home_team].draws += 1;
                    rows[fixture.away_team].draws += 1;
                    rows[fixture.home_team].points += 1;
                    rows[fixture.away_team].points += 1;
                }
            }
        }
        rows.sort_by(|a, b| b.points.cmp(&a.points)
            .then_with(|| b.goal_difference().cmp(&a.goal_difference()))
            .then_with(|| b.goals_for.cmp(&a.goals_for))
            .then_with(|| a.team.cmp(&b.team)));
        rows
    }

    pub fn validate(&self) -> Result<(), DynastyError> {
        if self.manager_name.trim().is_empty() || self.club.name.trim().is_empty() {
            return Err(DynastyError::InvalidSave);
        }
        if self.club.players.len() < 11 || self.club.players.len() > MAX_SQUAD_SIZE {
            return Err(DynastyError::InvalidSave);
        }
        let mut ids = self.club.players.iter().map(|player| player.id).collect::<Vec<_>>();
        ids.sort_unstable();
        ids.dedup();
        if ids.len() != self.club.players.len() {
            return Err(DynastyError::InvalidSave);
        }
        if self.club.starting_xi.len() != 11 || self.club.starting_xi.iter().any(|id| !ids.contains(id)) {
            return Err(DynastyError::InvalidSave);
        }
        Ok(())
    }

    fn apply_user_match(&mut self, fixture: &Fixture, score: Score) {
        for player_id in self.club.starting_xi.clone() {
            if let Some(player) = self.club.players.iter_mut().find(|player| player.id == player_id) {
                player.appearances = player.appearances.saturating_add(1);
                player.fitness = player.fitness.saturating_sub(18);
                player.morale = if score.home == score.away {
                    player.morale
                } else if (fixture.home_team == 0 && score.home > score.away) || (fixture.away_team == 0 && score.away > score.home) {
                    player.morale.saturating_add(2).min(100)
                } else {
                    player.morale.saturating_sub(2)
                };
                if player.suspension_matches > 0 {
                    player.suspension_matches -= 1;
                }
            }
        }
        let user_goals = if fixture.home_team == 0 { score.home } else { score.away };
        let mut eligible: Vec<u32> = self.club.starting_xi.iter().filter_map(|id| self.club.players.iter().find(|p| p.id == *id))
            .filter(|player| player.position == Position::Forward || player.position == Position::Midfielder)
            .map(|player| player.id).collect();
        eligible.sort_unstable();
        if !eligible.is_empty() {
            for goal_index in 0..user_goals {
                let seed_index = (self.seed.wrapping_add(u64::from(fixture.id) * 31 + u64::from(goal_index))) as usize % eligible.len();
                if let Some(player) = self.club.players.iter_mut().find(|player| player.id == eligible[seed_index]) {
                    player.goals = player.goals.saturating_add(1);
                }
            }
        }
        if self.club.starting_xi.iter().filter_map(|id| self.club.players.iter().find(|p| p.id == *id)).all(|p| p.injury_days == 0)
            && (if fixture.home_team == 0 { score.away } else { score.home }) == 0
        {
            self.club.reputation = self.club.reputation.saturating_add(1).min(100);
        }
    }

    fn team_strength(&self, team: usize) -> i16 {
        if team == 0 {
            let selected: Vec<&Player> = self.club.starting_xi.iter().filter_map(|id| self.club.players.iter().find(|player| player.id == *id)).collect();
            if selected.is_empty() {
                55
            } else {
                (selected.iter().map(|player| i16::from(player.overall()) + i16::from(player.fitness) / 10).sum::<i16>() / selected.len() as i16)
                    + i16::from(self.club.tactics.attacking_intent.saturating_sub(50)) / 8
            }
        } else {
            [59_i16, 63, 57].get(team - 1).copied().unwrap_or(55)
        }
    }

    fn team_name(&self, index: usize) -> &str {
        if index == 0 {
            &self.club.name
        } else {
            self.opponents.get(index - 1).map(String::as_str).unwrap_or("Неизвестный клуб")
        }
    }

    fn pay_weekly_wages(&mut self) {
        let wages = self.club.players.iter().map(|player| i64::from(player.wage)).sum::<i64>();
        let sponsor = 80_000 + i64::from(self.club.reputation) * 1_000;
        self.change_balance(-wages, "Зарплаты игроков за неделю".into());
        self.change_balance(sponsor, "Спонсорский доход за неделю".into());
    }

    fn change_balance(&mut self, delta: i64, description: String) {
        self.club.cash = self.club.cash.saturating_add(delta);
        self.ledger.push(LedgerEntry {
            game_day: self.game_day,
            description,
            delta,
            balance_after: self.club.cash,
        });
    }

    fn record_event(&mut self, title: &str, detail: String) {
        self.chronicle.push(ChronicleEntry {
            game_day: self.game_day,
            title: title.into(),
            detail,
        });
        if self.chronicle.len() > 100 {
            self.chronicle.remove(0);
        }
    }
}

fn clean_name(value: &str) -> String {
    value.trim().chars().take(32).collect::<String>()
}

fn lineup_requirements(formation: Formation) -> (usize, usize, usize, usize) {
    let (defenders, midfielders, forwards) = formation.counts();
    (1, defenders, midfielders, forwards)
}

fn best_lineup(players: &[Player], formation: Formation) -> Vec<u32> {
    let (goalkeepers, defenders, midfielders, forwards) = lineup_requirements(formation);
    let mut selected = Vec::with_capacity(11);
    for (position, count) in [
        (Position::Goalkeeper, goalkeepers),
        (Position::Defender, defenders),
        (Position::Midfielder, midfielders),
        (Position::Forward, forwards),
    ] {
        let mut candidates: Vec<&Player> = players.iter().filter(|player| {
            player.position == position && player.injury_days == 0 && player.suspension_matches == 0
        }).collect();
        candidates.sort_by_key(|player| std::cmp::Reverse(player.overall()));
        selected.extend(candidates.into_iter().take(count).map(|player| player.id));
    }
    selected
}

fn make_player(id: u32, name: &str, position: Position, age: u8, base: u8, seed: &mut u64) -> Player {
    let rating = |offset: i16, random: u64| (i16::from(base) + offset + (random % 7) as i16 - 3).clamp(35, 92) as u8;
    let attributes = Attributes {
        pace: rating(0, next(seed)),
        shooting: rating(1, next(seed)),
        passing: rating(0, next(seed)),
        defending: rating(-1, next(seed)),
        physical: rating(1, next(seed)),
        goalkeeping: rating(-8, next(seed)),
        stamina: rating(2, next(seed)),
    };
    Player {
        id,
        name: name.into(),
        position,
        age,
        attributes,
        potential: base.saturating_add((next(seed) % 17) as u8).min(99),
        fitness: 90,
        morale: 65,
        injury_days: 0,
        suspension_matches: 0,
        contract_days: 730,
        wage: u32::from(base).saturating_mul(85),
        appearances: 0,
        goals: 0,
        assists: 0,
    }
}

fn generate_market(seed: &mut u64) -> Vec<TransferTarget> {
    let candidates = [
        ("Давид Норик", Position::Goalkeeper),
        ("Лоренцо Вайс", Position::Defender),
        ("Микаэль Тарен", Position::Midfielder),
        ("Артур Селин", Position::Forward),
    ];
    candidates.into_iter().enumerate().map(|(index, (name, position))| {
        let player = make_player(1_000 + index as u32 + (next(seed) % 400) as u32, name, position, 19 + (next(seed) % 10) as u8, 60 + (next(seed) % 15) as u8, seed);
        let asking_price = player.market_value();
        TransferTarget { player, asking_price }
    }).collect()
}

fn season_fixtures() -> Vec<Fixture> {
    let pairs = [
        [(0_usize, 1_usize), (2, 3)],
        [(2, 0), (3, 1)],
        [(0, 3), (1, 2)],
        [(1, 0), (3, 2)],
        [(0, 2), (1, 3)],
        [(3, 0), (2, 1)],
    ];
    let mut fixtures = Vec::with_capacity(12);
    let mut id = 1;
    for (index, round) in pairs.into_iter().enumerate() {
        for (home, away) in round {
            fixtures.push(Fixture { id, matchday: index as u8 + 1, home_team: home, away_team: away, score: None });
            id += 1;
        }
    }
    fixtures
}

fn simulate_score(seed: u64, fixture_id: u32, home_rating: i16, away_rating: i16) -> Score {
    let mut home_rng = seed ^ (u64::from(fixture_id) * 0x9E37_79B9);
    let mut away_rng = seed.rotate_left(17) ^ (u64::from(fixture_id) * 0x85EB_CA6B);
    let home_chance = (850_i16 + (home_rating - away_rating) * 18).clamp(250, 1_850) as u64;
    let away_chance = (850_i16 + (away_rating - home_rating) * 18).clamp(250, 1_850) as u64;
    let mut home = 0_u8;
    let mut away = 0_u8;
    for _ in 0..90 {
        if next(&mut home_rng) % 10_000 < home_chance && home < 6 {
            home += 1;
        }
        if next(&mut away_rng) % 10_000 < away_chance && away < 6 {
            away += 1;
        }
    }
    Score { home, away }
}

fn next(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_campaign() -> Campaign {
        Campaign::new("Антон", "ФК Север", 42).unwrap()
    }

    #[test]
    fn campaign_starts_with_complete_fictional_squad_and_schedule() {
        let campaign = new_campaign();
        assert_eq!(campaign.club.players.len(), 22);
        assert_eq!(campaign.club.starting_xi.len(), 11);
        assert_eq!(campaign.fixtures.len(), 12);
        assert_eq!(campaign.standings().iter().map(|row| row.played).sum::<u8>(), 0);
        campaign.validate().unwrap();
    }

    #[test]
    fn formation_changes_rebuild_a_valid_starting_eleven() {
        let mut campaign = new_campaign();
        campaign.set_formation(Formation::FourFourTwo);
        assert_eq!(campaign.club.starting_xi.len(), 11);
        campaign.set_formation(Formation::ThreeFiveTwo);
        assert_eq!(campaign.club.starting_xi.len(), 11);
        campaign.validate().unwrap();
    }

    #[test]
    fn lineup_selection_replaces_only_the_same_position() {
        let mut campaign = new_campaign();
        let forward = campaign.club.players.iter().find(|player| player.position == Position::Forward && !campaign.is_starter(player.id)).unwrap().id;
        campaign.select_player(forward).unwrap();
        assert_eq!(campaign.club.starting_xi.len(), 11);
        assert!(campaign.is_starter(forward));
    }

    #[test]
    fn training_changes_attributes_and_records_cost() {
        let mut campaign = new_campaign();
        let old_balance = campaign.club.cash;
        campaign.train(TrainingFocus::Attack).unwrap();
        assert!(campaign.club.cash < old_balance);
        assert_eq!(campaign.last_training_day, Some(2));
        assert!(campaign.ledger.last().unwrap().description.contains("Тренировка"));
    }

    #[test]
    fn training_is_limited_to_one_session_per_day() {
        let mut campaign = new_campaign();
        campaign.train(TrainingFocus::Fitness).unwrap();
        assert_eq!(campaign.train(TrainingFocus::Fitness), Err(DynastyError::TrainingAlreadyUsed));
    }

    #[test]
    fn matchday_persists_results_updates_table_and_player_careers() {
        let mut campaign = new_campaign();
        let results = campaign.advance_matchday().unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(campaign.matchday, 2);
        assert!(campaign.fixtures.iter().filter(|fixture| fixture.matchday == 1).all(|fixture| fixture.score.is_some()));
        assert_eq!(campaign.club.players.iter().map(|player| player.appearances).sum::<u16>(), 11);
        assert_eq!(campaign.standings().iter().map(|row| row.played).sum::<u8>(), 4);
        assert!(campaign.ledger.len() >= 3);
    }

    #[test]
    fn season_rolls_over_after_six_matchdays() {
        let mut campaign = new_campaign();
        for _ in 0..6 {
            campaign.advance_matchday().unwrap();
        }
        assert_eq!(campaign.season, 2);
        assert_eq!(campaign.matchday, 1);
        assert!(campaign.fixtures.iter().all(|fixture| fixture.score.is_none()));
        assert_eq!(campaign.standings().iter().map(|row| row.played).sum::<u8>(), 0);
    }

    #[test]
    fn transfer_buy_and_sell_reconcile_the_cash_ledger() {
        let mut campaign = new_campaign();
        let candidate = campaign.transfer_market[0].clone();
        let balance = campaign.club.cash;
        campaign.sign_player(candidate.player.id).unwrap();
        assert_eq!(campaign.club.cash, balance - i64::from(candidate.asking_price));
        let reserve = campaign.club.players.iter().find(|player| !campaign.is_starter(player.id)).unwrap().id;
        let before_sale = campaign.club.cash;
        let proceeds = campaign.sell_player(reserve).unwrap();
        assert_eq!(campaign.club.cash, before_sale + i64::from(proceeds));
    }

    #[test]
    fn transfer_cannot_sell_a_starter_or_break_minimum_squad_size() {
        let mut campaign = new_campaign();
        assert_eq!(campaign.sell_player(campaign.club.starting_xi[0]), Err(DynastyError::CannotSellStarter));
        for _ in 0..4 {
            let reserve = campaign.club.players.iter().find(|player| !campaign.is_starter(player.id)).unwrap().id;
            campaign.sell_player(reserve).unwrap();
        }
        assert_eq!(campaign.club.players.len(), 18);
        let reserve = campaign.club.players.iter().find(|player| !campaign.is_starter(player.id)).unwrap().id;
        assert_eq!(campaign.sell_player(reserve), Err(DynastyError::CannotSellBelowMinimum));
    }

    #[test]
    fn transfer_market_and_budget_are_enforced() {
        let mut campaign = new_campaign();
        campaign.club.cash = 0;
        let candidate = campaign.transfer_market[0].player.id;
        assert_eq!(campaign.sign_player(candidate), Err(DynastyError::InsufficientFunds));
    }

    #[test]
    fn succession_requires_an_adult_heir_and_preserves_the_club() {
        let mut campaign = new_campaign();
        campaign.add_heir("Егор", 16).unwrap();
        assert_eq!(campaign.retire_to_heir(1), Err(DynastyError::HeirNotAdult));
        campaign.add_heir("Михаил", 23).unwrap();
        campaign.retire_to_heir(2).unwrap();
        assert_eq!(campaign.manager_name, "Михаил");
        assert_eq!(campaign.retired_managers, vec!["Антон"]);
        assert_eq!(campaign.club.name, "ФК Север");
    }

    #[test]
    fn corrupted_saves_are_rejected() {
        let mut campaign = new_campaign();
        campaign.club.starting_xi.pop();
        assert_eq!(campaign.validate(), Err(DynastyError::InvalidSave));
    }

    #[test]
    fn identical_campaign_seed_reproduces_the_same_first_matchday() {
        let mut first = Campaign::new("Антон", "ФК Север", 42).unwrap();
        let mut second = Campaign::new("Антон", "ФК Север", 42).unwrap();
        assert_eq!(first.advance_matchday().unwrap(), second.advance_matchday().unwrap());
    }
}

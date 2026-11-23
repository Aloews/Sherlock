use serde::Deserialize;
use std::fmt;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Card {
    pub id: u32,
    pub category: String,
    pub category_ru: String,
    pub name: String,
    pub difficulty: String,
}

pub fn bundled_cards() -> Vec<Card> {
    serde_json::from_str(include_str!("../data/cards.json"))
        .expect("bundled football Alias card data must be valid JSON")
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameSettings {
    pub team_names: [String; 2],
    pub seconds_per_round: u16,
    pub rounds: u8,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            team_names: ["Команда 1".into(), "Команда 2".into()],
            seconds_per_round: 60,
            rounds: 4,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GamePhase {
    Setup,
    Active,
    RoundSummary,
    Finished,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameError {
    EmptyDeck,
    EmptyTeamName,
    InvalidRoundLength,
    InvalidRoundCount,
    InvalidPhase,
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDeck => write!(f, "Для игры нужна хотя бы одна карточка"),
            Self::EmptyTeamName => write!(f, "У каждой команды должно быть название"),
            Self::InvalidRoundLength => write!(f, "Длительность раунда должна быть от 1 до 180 секунд"),
            Self::InvalidRoundCount => write!(f, "Количество раундов должно быть от 2 до 8"),
            Self::InvalidPhase => write!(f, "Действие недоступно на текущем этапе игры"),
        }
    }
}

impl std::error::Error for GameError {}

#[derive(Clone, Debug)]
pub struct Game {
    cards: Vec<Card>,
    settings: GameSettings,
    phase: GamePhase,
    scores: [u32; 2],
    active_team: usize,
    round_number: u8,
    seconds_left: u16,
    card_index: usize,
    round_points: u32,
    round_correct: u32,
    round_skipped: u32,
}

impl Game {
    pub fn new(
        mut cards: Vec<Card>,
        settings: GameSettings,
        seed: u64,
    ) -> Result<Self, GameError> {
        if cards.is_empty() {
            return Err(GameError::EmptyDeck);
        }
        if settings.team_names.iter().any(|name| name.trim().is_empty()) {
            return Err(GameError::EmptyTeamName);
        }
        if !(1..=180).contains(&settings.seconds_per_round) {
            return Err(GameError::InvalidRoundLength);
        }
        if !(2..=8).contains(&settings.rounds) {
            return Err(GameError::InvalidRoundCount);
        }

        shuffle(&mut cards, seed);
        Ok(Self {
            cards,
            seconds_left: settings.seconds_per_round,
            settings,
            phase: GamePhase::Setup,
            scores: [0, 0],
            active_team: 0,
            round_number: 1,
            card_index: 0,
            round_points: 0,
            round_correct: 0,
            round_skipped: 0,
        })
    }

    pub fn start(&mut self) -> Result<(), GameError> {
        if self.phase != GamePhase::Setup {
            return Err(GameError::InvalidPhase);
        }
        self.phase = GamePhase::Active;
        Ok(())
    }

    pub fn phase(&self) -> GamePhase {
        self.phase
    }

    pub fn team_name(&self, index: usize) -> &str {
        self.settings.team_names.get(index).map(String::as_str).unwrap_or("")
    }

    pub fn active_team(&self) -> usize {
        self.active_team
    }

    pub fn active_team_name(&self) -> &str {
        self.team_name(self.active_team)
    }

    pub fn round_number(&self) -> u8 {
        self.round_number
    }

    pub fn total_rounds(&self) -> u8 {
        self.settings.rounds
    }

    pub fn seconds_left(&self) -> u16 {
        self.seconds_left
    }

    pub fn seconds_per_round(&self) -> u16 {
        self.settings.seconds_per_round
    }

    pub fn scores(&self) -> [u32; 2] {
        self.scores
    }

    pub fn round_points(&self) -> u32 {
        self.round_points
    }

    pub fn round_correct(&self) -> u32 {
        self.round_correct
    }

    pub fn round_skipped(&self) -> u32 {
        self.round_skipped
    }

    pub fn current_card(&self) -> Option<&Card> {
        if self.phase == GamePhase::Active {
            self.cards.get(self.card_index)
        } else {
            None
        }
    }

    pub fn remaining_cards(&self) -> usize {
        self.cards.len().saturating_sub(self.card_index)
    }

    pub fn correct(&mut self) -> Result<(), GameError> {
        self.ensure_active()?;
        if self.current_card().is_none() {
            self.phase = GamePhase::RoundSummary;
            return Ok(());
        }
        self.scores[self.active_team] += 1;
        self.round_points += 1;
        self.round_correct += 1;
        self.advance_card();
        Ok(())
    }

    pub fn skip(&mut self) -> Result<(), GameError> {
        self.ensure_active()?;
        if self.current_card().is_none() {
            self.phase = GamePhase::RoundSummary;
            return Ok(());
        }
        self.round_skipped += 1;
        self.advance_card();
        Ok(())
    }

    pub fn tick(&mut self) {
        if self.phase != GamePhase::Active {
            return;
        }
        self.seconds_left = self.seconds_left.saturating_sub(1);
        if self.seconds_left == 0 {
            self.phase = GamePhase::RoundSummary;
        }
    }

    pub fn finish_round(&mut self) -> Result<(), GameError> {
        self.ensure_active()?;
        self.phase = GamePhase::RoundSummary;
        Ok(())
    }

    pub fn next_round(&mut self) -> Result<(), GameError> {
        if self.phase != GamePhase::RoundSummary {
            return Err(GameError::InvalidPhase);
        }
        if self.round_number >= self.settings.rounds || self.remaining_cards() == 0 {
            self.phase = GamePhase::Finished;
            return Ok(());
        }
        self.round_number += 1;
        self.active_team = 1 - self.active_team;
        self.seconds_left = self.settings.seconds_per_round;
        self.round_points = 0;
        self.round_correct = 0;
        self.round_skipped = 0;
        self.phase = GamePhase::Active;
        Ok(())
    }

    pub fn winner(&self) -> Option<usize> {
        match self.scores[0].cmp(&self.scores[1]) {
            std::cmp::Ordering::Greater => Some(0),
            std::cmp::Ordering::Less => Some(1),
            std::cmp::Ordering::Equal => None,
        }
    }

    fn ensure_active(&self) -> Result<(), GameError> {
        if self.phase == GamePhase::Active {
            Ok(())
        } else {
            Err(GameError::InvalidPhase)
        }
    }

    fn advance_card(&mut self) {
        self.card_index += 1;
        if self.card_index >= self.cards.len() {
            self.phase = GamePhase::RoundSummary;
        }
    }
}

fn shuffle<T>(items: &mut [T], seed: u64) {
    let mut state = if seed == 0 { 0x9E37_79B9_7F4A_7C15 } else { seed };
    for index in (1..items.len()).rev() {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let other = (state as usize) % (index + 1);
        items.swap(index, other);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cards(count: u32) -> Vec<Card> {
        (1..=count)
            .map(|id| Card {
                id,
                category: "player".into(),
                category_ru: "игроки".into(),
                name: format!("Игрок {id}"),
                difficulty: "medium".into(),
            })
            .collect()
    }

    fn settings() -> GameSettings {
        GameSettings {
            team_names: ["Красные".into(), "Синие".into()],
            seconds_per_round: 2,
            rounds: 2,
        }
    }

    #[test]
    fn bundled_deck_contains_real_legacy_cards() {
        let deck = bundled_cards();
        assert!(deck.len() >= 2_000);
        assert!(deck.iter().any(|card| card.name == "Серхио Агуэро"));
        assert!(deck.iter().any(|card| card.category == "club"));
    }

    #[test]
    fn correct_answer_scores_and_advances_the_card() {
        let mut game = Game::new(cards(3), settings(), 7).unwrap();
        game.start().unwrap();
        let first_id = game.current_card().unwrap().id;

        game.correct().unwrap();

        assert_eq!(game.scores(), [1, 0]);
        assert_eq!(game.round_points(), 1);
        assert_eq!(game.round_correct(), 1);
        assert_ne!(game.current_card().unwrap().id, first_id);
    }

    #[test]
    fn skipping_advances_without_awarding_a_point() {
        let mut game = Game::new(cards(3), settings(), 11).unwrap();
        game.start().unwrap();
        game.skip().unwrap();

        assert_eq!(game.scores(), [0, 0]);
        assert_eq!(game.round_skipped(), 1);
        assert_eq!(game.remaining_cards(), 2);
    }

    #[test]
    fn timer_ends_the_round_exactly_at_zero() {
        let mut game = Game::new(cards(4), settings(), 13).unwrap();
        game.start().unwrap();
        game.tick();
        assert_eq!(game.seconds_left(), 1);
        assert_eq!(game.phase(), GamePhase::Active);
        game.tick();
        assert_eq!(game.seconds_left(), 0);
        assert_eq!(game.phase(), GamePhase::RoundSummary);
    }

    #[test]
    fn teams_alternate_and_the_match_finishes_after_configured_rounds() {
        let mut game = Game::new(cards(20), settings(), 19).unwrap();
        game.start().unwrap();
        game.correct().unwrap();
        game.finish_round().unwrap();
        game.next_round().unwrap();

        assert_eq!(game.round_number(), 2);
        assert_eq!(game.active_team(), 1);
        assert_eq!(game.active_team_name(), "Синие");

        game.finish_round().unwrap();
        game.next_round().unwrap();
        assert_eq!(game.phase(), GamePhase::Finished);
        assert_eq!(game.winner(), Some(0));
    }

    #[test]
    fn actions_are_rejected_outside_the_active_round() {
        let mut game = Game::new(cards(3), settings(), 23).unwrap();
        assert_eq!(game.correct(), Err(GameError::InvalidPhase));
        game.start().unwrap();
        game.finish_round().unwrap();
        assert_eq!(game.skip(), Err(GameError::InvalidPhase));
    }

    #[test]
    fn invalid_configuration_is_rejected() {
        assert_eq!(
            Game::new(Vec::new(), settings(), 1).err(),
            Some(GameError::EmptyDeck)
        );
        let mut invalid = settings();
        invalid.team_names[0] = " ".into();
        assert_eq!(
            Game::new(cards(2), invalid, 1).err(),
            Some(GameError::EmptyTeamName)
        );
    }
}

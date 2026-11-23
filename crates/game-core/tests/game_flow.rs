use sherlock_game_core::{bundled_cards, Game, GamePhase, GameSettings};

#[test]
fn a_complete_two_team_match_has_two_turns_per_team() {
    let cards = bundled_cards();
    let settings = GameSettings {
        team_names: ["Север".into(), "Юг".into()],
        seconds_per_round: 45,
        rounds: 4,
    };
    let mut game = Game::new(cards, settings, 20261123).expect("valid match");
    game.start().expect("match starts");

    for expected_team in [0, 1, 0, 1] {
        assert_eq!(game.active_team(), expected_team);
        game.correct().expect("correct answer");
        game.skip().expect("skip card");
        game.finish_round().expect("round finishes");
        game.next_round().expect("next round");
    }

    assert_eq!(game.phase(), GamePhase::Finished);
    assert_eq!(game.scores(), [2, 2]);
    assert_eq!(game.winner(), None);
}

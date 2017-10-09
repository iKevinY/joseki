extern crate joseki;

use joseki::Game;

#[test]
fn read_game() {
    let game = Game::from_sgf("tests/games/AlphaGo-Lee_Sedol-4.sgf");
    let game_str = format!("{}", game);

    assert!(game_str.contains("White Player: Lee Sedol"));
    assert!(game_str.contains("● ● ● ○ ○ ● ○ ⋅ ○ ○ ● ● ● ○ ● ⋅ ● ○ ⋅"));
}

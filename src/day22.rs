use std::collections::{VecDeque, HashSet};

pub fn day22(input_lines: &[String]) -> (u64, u64) {
    let mut input_iter = input_lines.iter();
    let mut player1 = parse_deck(&mut input_iter);
    let mut player1_clone = player1.clone();
    let mut player2 = parse_deck(&mut input_iter);
    let mut player2_clone = player2.clone();
    play_game(&mut player1, &mut player2, false);
    let part1 = winning_score(&player1, &player2);
    play_game(&mut player1_clone, &mut player2_clone, true);
    let part2 = winning_score(&player1_clone, &player2_clone);
    (part1,part2)
}

fn parse_deck<'a>(input_lines: &mut impl Iterator<Item = &'a String>) -> VecDeque<u64> {
    let mut deck: VecDeque<u64> = VecDeque::new();
    input_lines.next(); // Skip over the player name
    for line in input_lines {
        if line.is_empty() { break; }
        deck.push_back(line.parse::<u64>().expect("Invalid input"));
    }
    deck
}

enum Winner {
    Player1,
    Player2,
}

#[derive(PartialEq,Eq,Hash)]
struct GameState {
    player1: VecDeque<u64>,
    player2: VecDeque<u64>,
}

// static mut GAME_DEPTH: u64 = 0;

fn play_game(player1: &mut VecDeque<u64>, player2: &mut VecDeque<u64>, recursive: bool) -> Winner {
    // unsafe { GAME_DEPTH += 1; }

    let mut seen_states: HashSet<GameState> = HashSet::new();
    while !player1.is_empty() && !player2.is_empty() {
        let state = GameState { player1: player1.clone(), player2: player2.clone() };
        if seen_states.contains(&state) {
            return Winner::Player1;
        } else {
            seen_states.insert(state);
        }

        play_round(player1, player2, recursive)
    }

    // unsafe {
    //     for _ in 0..GAME_DEPTH { print!("--"); }
    //     println!("{}", seen_states.len());
    //     GAME_DEPTH -= 1;
    // }

    if player2.is_empty() { Winner::Player1 } else { Winner::Player2 }
}

fn play_round(player1: &mut VecDeque<u64>, player2: &mut VecDeque<u64>, recursive: bool) {
    let player1_card = player1.pop_front().unwrap();
    let player2_card = player2.pop_front().unwrap();
    match determine_winner(player1, &player1_card, player2, &player2_card, recursive) {
        Winner::Player1 => {
            player1.push_back(player1_card);
            player1.push_back(player2_card);
        },
        Winner::Player2 => {
            player2.push_back(player2_card);
            player2.push_back(player1_card);
        }
    }
}

fn determine_winner(player1: &VecDeque<u64>, player1_card: &u64, player2: &VecDeque<u64>, player2_card: &u64, recursive: bool) -> Winner {
    if !recursive || player1.len() < *player1_card as usize || player2.len() < *player2_card as usize {
        // Determined by the cards.
        if player1_card > player2_card { Winner::Player1 } else { Winner::Player2 }
    } else {
        // Determined by sub-game.
        let mut player1_clone = player1.clone();
        let mut player2_clone = player2.clone();
        player1_clone.truncate(*player1_card as usize);
        player2_clone.truncate(*player2_card as usize);
        play_game(&mut player1_clone, &mut player2_clone, recursive)
    }
}

fn winning_score(player1: &VecDeque<u64>, player2: &VecDeque<u64>) -> u64 {
    if player2.is_empty() {
        score(player1)
    } else {
        score(player2)
    }
}

fn score(deck: &VecDeque<u64>) -> u64 {
    deck.iter().rev().enumerate().map(|(index, card)| (index + 1) as u64 * *card).sum()
}
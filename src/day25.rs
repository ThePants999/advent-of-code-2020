const CARD_PUBLIC_KEY: u64 = 6930903;
const DOOR_PUBLIC_KEY: u64 = 19716708;

pub fn day25(_input_lines: &[String]) -> (u64, u64) {
    let mut candidate_loop_size = 0u64;
    let mut value = 1u64;
    let mut card_loop_size: Option<u64> = None;
    let mut door_loop_size: Option<u64> = None;
    
    while card_loop_size.is_none() || door_loop_size.is_none() {
        candidate_loop_size += 1;
        value = transform_once(value, 7);

        if card_loop_size.is_none() && value == CARD_PUBLIC_KEY {
            card_loop_size = Some(candidate_loop_size);
        }
        if door_loop_size.is_none() && value == DOOR_PUBLIC_KEY {
            door_loop_size = Some(candidate_loop_size);
        }
    }

    let part1 = transform(CARD_PUBLIC_KEY, door_loop_size.unwrap());
    
    (part1,49)
}

fn transform(subject: u64, loop_size: u64) -> u64 {
    let mut value = 1u64;
    for _ in 0..loop_size {
        value = transform_once(value, subject)
    }
    value
}

fn transform_once(value: u64, subject: u64) -> u64 {
    (value * subject) % 20201227
}
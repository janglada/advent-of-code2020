pub fn run() {
    let card_pkey: u64 = 3469259;
    let door_pkey: u64 = 13170438;
    let card_loop_size = find_loop_size(7, card_pkey);
    let door_loop_size = find_loop_size(7, door_pkey);
    println!("{}", card_loop_size);
    println!("{}", door_loop_size);

    let key1 = transform(door_pkey, card_loop_size);
    let key2 = transform(card_pkey, door_loop_size);

    println!("{}", key1);
    println!("{}", key2);
}

fn transform_once(subject_number: u64, value: u64) -> u64 {
    (subject_number * value) % 20201227
}

fn transform(subject_number: u64, loops: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loops {
        value = transform_once(subject_number, value);
    }
    value
}

fn find_loop_size(subject_number: u64, pkey: u64) -> u64 {
    let mut value = 1;
    let mut loops: u64 = 0;
    while value != pkey {
        value = transform_once(subject_number, value);
        loops += 1;
    }
    loops
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hasndshake() {
        assert_eq!(transform(17807724, 8), 14897079);
        assert_eq!(transform(5764801, 11), 14897079);
    }

    #[test]
    fn test_hasndshake2() {
        assert_eq!(find_loop_size(7, 5764801), 8 as u64);
    }
}

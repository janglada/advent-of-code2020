type Card = u32;

type Deck = Vec<Card>;

enum Players {
    Player1(Deck),
    Player2(Deck),
}

pub fn run() {
    let mut iter = include_str!("../day22.txt").split("\n\n");

    let mut player1 = iter
        .next()
        .unwrap()
        .split("\n")
        .skip(1)
        .map(|str| str.parse::<Card>().unwrap())
        .collect::<Deck>();

    let mut player2 = iter
        .next()
        .unwrap()
        .split("\n")
        .skip(1)
        .map(|str| str.parse::<Card>().unwrap())
        .collect::<Deck>();

    while player1.len() != 0 && player2.len() != 0 {
        let first_card1 = player1.remove(0);
        let first_card2 = player2.remove(0);

        if first_card1 > first_card2 {
            player1.push(first_card1);
            player1.push(first_card2);
        } else {
            player2.push(first_card2);
            player2.push(first_card1);
        }
    }

    let n: u32 = player1
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &v)| (i + 1) as u32 * v)
        .sum();

    println!("Player 1 {:?}", player1);
    println!(" {}", n);
}

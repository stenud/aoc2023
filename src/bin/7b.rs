#![allow(unused_imports)]
use std::fs;

#[derive(Debug)]
struct Hand {
    card_2: u32,
    card_3: u32,
    card_4: u32,
    card_5: u32,
    card_6: u32,
    card_7: u32,
    card_8: u32,
    card_9: u32,
    card_t: u32,
    card_j: u32,
    card_q: u32,
    card_k: u32,
    card_a: u32,
}

impl Hand {
    fn new() -> Hand {
        Hand {
            card_2: 0,
            card_3: 0,
            card_4: 0,
            card_5: 0,
            card_6: 0,
            card_7: 0,
            card_8: 0,
            card_9: 0,
            card_t: 0,
            card_j: 0,
            card_q: 0,
            card_k: 0,
            card_a: 0,
        }
    }

    fn fields_as_array(&self) -> [u32; 13] {
        [self.card_j.clone(), self.card_2.clone(), self.card_3.clone(), self.card_4.clone(), self.card_5.clone(),
         self.card_6.clone(), self.card_7.clone(), self.card_8.clone(), self.card_9.clone(), self.card_t.clone(),
         self.card_q.clone(), self.card_k.clone(), self.card_a.clone()]
    }
}

fn main() {
    let contents = fs::read_to_string("input/7.txt").unwrap();
    //let contents = _example();

    let hands_and_bids: Vec<(String, u32)> = contents
        .lines()
        .map(|l| {
            let mut parts = l.split(' ');
            let hand = parts
                .next()
                .unwrap()
                .replace('A', "E")
                .replace('K', "D")
                .replace('Q', "C")
                .replace('J', "1")
                .replace('T', "A");
            let bid = parts.next().unwrap().parse().unwrap();
            (hand, bid)
        })
        .collect();

    let sorted_hands = sort_hands(hands_and_bids);

    let mut sum = 0;
    for (i, (_, b)) in sorted_hands.iter().enumerate() {
        let i = i + 1;
        sum = sum + (i as u32 * b);
    }

    println!("a: {}", sum);
}

fn sort_hands(hands_and_bids: Vec<(String, u32)>) -> Vec<(String, u32)> {
    let mut five: Vec<(String, u32)> = Vec::new();
    let mut four: Vec<(String, u32)> = Vec::new();
    let mut full_house: Vec<(String, u32)> = Vec::new();
    let mut three: Vec<(String, u32)> = Vec::new();
    let mut two_pair: Vec<(String, u32)> = Vec::new();
    let mut pair: Vec<(String, u32)> = Vec::new();
    let mut highest: Vec<(String, u32)> = Vec::new();

    for hand_and_bid in hands_and_bids.into_iter() {
        let mut hand = Hand::new();
        
        for card in hand_and_bid.0.chars() {
            match card {
                '2' => hand.card_2 += 1,
                '3' => hand.card_3 += 1,
                '4' => hand.card_4 += 1,
                '5' => hand.card_5 += 1,
                '6' => hand.card_6 += 1,
                '7' => hand.card_7 += 1,
                '8' => hand.card_8 += 1,
                '9' => hand.card_9 += 1,
                'A' => hand.card_t += 1,
                '1' => hand.card_j += 1,
                'C' => hand.card_q += 1,
                'D' => hand.card_k += 1,
                'E' => hand.card_a += 1,
                c => println!("Card {c} is not a valid card."),
            }
        }

        let mut hand_cards = hand.fields_as_array();
        let mut jokers = 0;

        if hand_cards[0] != 5 {
            jokers = hand_cards[0];
            hand_cards[0] = 0;
        }

        hand_cards.sort();
        hand_cards.reverse();
        hand_cards[0] += jokers;
        let hand_cards = &hand_cards[..2];

        match hand_cards {
            &[5,0] => five.push(hand_and_bid),
            &[4,1] => four.push(hand_and_bid),
            &[3,2] => full_house.push(hand_and_bid),
            &[3,1] => three.push(hand_and_bid),
            &[2,2] => two_pair.push(hand_and_bid),
            &[2,1] => pair.push(hand_and_bid),
            &[1,1] => highest.push(hand_and_bid),
            a => println!("[{}, {}] is not a valid hand!", a[0], a[1]),
        }
    }

    five.sort_by(|a, b| a.0.cmp(&b.0));
    four.sort_by(|a, b| a.0.cmp(&b.0));
    full_house.sort_by(|a, b| a.0.cmp(&b.0));
    three.sort_by(|a, b| a.0.cmp(&b.0));
    two_pair.sort_by(|a, b| a.0.cmp(&b.0));
    pair.sort_by(|a, b| a.0.cmp(&b.0));
    highest.sort_by(|a, b| a.0.cmp(&b.0));

    vec![highest, pair, two_pair, three, full_house, four, five].concat()
}

fn _example() -> String {
    "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483".to_string()
}
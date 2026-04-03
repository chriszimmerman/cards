use crate::{Card, Rank};

pub fn hand_score(mut hand: Vec<Card>) -> u32 {
    let mut score = 0;
    hand.sort_by(|a, b| a.rank.cmp(&b.rank).reverse());
    for card in hand.iter() {
        let card_score = match card.rank {
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
            Rank::Ten => 10,
            Rank::Nine => 9,
            Rank::Eight => 8,
            Rank::Seven => 7,
            Rank::Six => 6,
            Rank::Five => 5,
            Rank::Four => 4,
            Rank::Three => 3,
            Rank::Two => 2,
            Rank::Ace => calculate_ace(score),
        };
        score += card_score;
    }

    score
}

fn calculate_ace(score: u32) -> u32 {
    if score > 10 { 1 } else { 11 }
}

#[cfg(test)]
mod tests {
    use crate::{Card, Rank, Suit};
    use crate::score::hand_score;

    #[test]
    fn test_hand_score_empty_hand() {
        assert_eq!(hand_score(Vec::new()), 0, "Empty hand score");
    }

    #[test]
    fn test_hand_score_single_number_card() {
        let card = Card {
            suit: Suit::Clubs,
            rank: Rank::Two,
            image: String::new()
        };
        assert_eq!(hand_score(vec![card]), 2, "Single card");
    }

    #[test]
    fn test_hand_score_single_number_card_9() {
        let card = Card {
            suit: Suit::Clubs,
            rank: Rank::Nine,
            image: String::new()
        };
        assert_eq!(hand_score(vec![card]), 9, "Number nine");
    }

    #[test]
    fn test_hand_score_single_face_card() {
        let card = Card {
            suit: Suit::Clubs,
            rank: Rank::Jack,
            image: String::new()
        };
        assert_eq!(hand_score(vec![card]), 10, "Single face card");
    }

    #[test]
    fn test_hand_score_single_ace() {
        let card = Card {
            suit: Suit::Clubs,
            rank: Rank::Ace,
            image: String::new()
        };
        assert_eq!(hand_score(vec![card]), 11, "Single ace");
    }

    #[test]
    fn test_hand_score_two_number_cards() {
        let card_1 = Card {
            suit: Suit::Clubs,
            rank: Rank::Seven,
            image: String::new()
        };
        let card_2 = Card {
            suit: Suit::Clubs,
            rank: Rank::Four,
            image: String::new()
        };
        assert_eq!(hand_score(vec![card_1, card_2]), 11, "Two number cards");
    }

    #[test]
    fn test_hand_score_two_face_cards() {
        let card_1 = Card {
            suit: Suit::Clubs,
            rank: Rank::King,
            image: String::new()
        };
        let card_2 = Card {
            suit: Suit::Clubs,
            rank: Rank::Jack,
            image: String::new()
        };
        assert_eq!(hand_score(vec![card_1, card_2]), 20, "Two face cards");
    }

    #[test]
    fn test_hand_score_ace_and_number_card() {
        let card_1 = Card {
            suit: Suit::Clubs,
            rank: Rank::Ace,
            image: String::new()
        };
        let card_2 = Card {
            suit: Suit::Clubs,
            rank: Rank::Five,
            image: String::new()
        };
        assert_eq!(hand_score(vec![card_1, card_2]), 16, "Ace and five is sixteen");
    }

    #[test]
    fn test_hand_score_two_aces() {
        let card_1 = Card {
            suit: Suit::Clubs,
            rank: Rank::Ace,
            image: String::new()
        };
        let card_2 = Card {
            suit: Suit::Diamonds,
            rank: Rank::Ace,
            image: String::new()
        };
        assert_eq!(hand_score(vec![card_1, card_2]), 12, "Two aces won't go over 21");
    }

    #[test]
    fn test_hand_more_than_two_cards() {
        let card_1 = Card {
            suit: Suit::Clubs,
            rank: Rank::Seven,
            image: String::new()
        };
        let card_2 = Card {
            suit: Suit::Diamonds,
            rank: Rank::Ace,
            image: String::new()
        };
        let card_3 = Card {
            suit: Suit::Diamonds,
            rank: Rank::Jack,
            image: String::new()
        };
        assert_eq!(hand_score(vec![card_1, card_2, card_3]), 18, "Ace will be worth 11 until other cards add up to over 10");
    }

    #[test]
    fn test_hand_multiple_aces() {
        let card_1 = Card {
            suit: Suit::Clubs,
            rank: Rank::Seven,
            image: String::new()
        };
        let card_2 = Card {
            suit: Suit::Diamonds,
            rank: Rank::Ace,
            image: String::new()
        };
        let card_3 = Card {
            suit: Suit::Diamonds,
            rank: Rank::Jack,
            image: String::new()
        };
        let card_4 = Card {
            suit: Suit::Diamonds,
            rank: Rank::Ace,
            image: String::new()
        };
        assert_eq!(hand_score(vec![card_1, card_2, card_3, card_4]), 19, "Ace will be worth 11 until other cards add up to over 10");
    }
}
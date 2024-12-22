use super::{get_next_secret, PriceChangeIterator};

#[test]
fn test_next_secret_generator() {
    let mut starting_secret = 123;
    let following_secrets = vec![
        15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432, 5908254,
    ];

    for i in following_secrets {
        starting_secret = get_next_secret(starting_secret);
        assert_eq!(starting_secret, i);
    }
}

#[test]
fn test_finding_first_occurance_of_price_changes() {
    let test_list = vec![(1, 7), (2, 7), (3, 0), (2024, 9)];
    for (starting_value, test_value) in test_list {
        let mut iter = PriceChangeIterator::new(starting_value);
        let highest_value = iter.highest_value_given_price_changes((-2, 1, -1, 3), 2000);
        assert_eq!(highest_value, test_value);
    }
}

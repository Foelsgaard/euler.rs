// Project Euler: Problem 31

fn main() {

    let coins: Vec<u64> = vec![200, 100, 50, 20, 10, 5, 2, 1];

    let sum = coin_sums(200, coins);

    println!("There are a total of {} number of ways \
              you can make 2£ with the coins 1p, 2p, \
              5p, 10p, 20p, 50p, 1£, and 2£.", sum);
}

fn coin_sums(total: u64, coins: Vec<u64>) -> u64 {

    match coins.split_first() {
        None => match total {
            0 => 1,
            _ => 0
        },
        Some((coin, rest)) => {
            if *coin == 1 {
                return 1;
            }
            
            (0..total/coin+1)
                .map(|n| coin_sums(total - n * coin,
                                   Vec::from(rest)))
                .sum()
        }
    }
    
}

// Project Euler: Problem 36

fn main() {

    println!("These are the double-base palindromes below one million:");
    
    let double_base_palindromes = (1..1000000)
        .filter(|n| {
            let n_dec = format!("{}", n);
            let n_bin = format!("{:b}", n);

            let n_dec_rev: String = n_dec.chars().rev().collect();
            let n_bin_rev: String = n_bin.chars().rev().collect();

            n_dec == n_dec_rev && n_bin == n_bin_rev
            
        })
        .inspect(|n| {
            println!("{}", n);
        });

    println!("They sum up to {}.", double_base_palindromes.sum::<u64>());
}

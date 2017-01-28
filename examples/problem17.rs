// Project Euler: Problem 17

fn main() {
    let n: u64 = (1..1001)
        .map(|n| write_out_number(n).chars()
                  .filter(|&c| c != ' ')
                  .count() as u64)
        .sum();

    println!("A total of {} letters is needed to spell out all numbers from one to one thousand.", n);
}

fn write_out_number(n: u64) -> String {

    fn write_out_digit(d: u64) -> String {
        String::from(match d {
            0 => "zero",
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            _ => ""
        })
    }

    fn write_out_teens(t: u64) -> String {
        String::from(match t {
            0 => "ten",
            1 => "eleven",
            2 => "twelve",
            3 => "thirteen",
            4 => "fourteen",
            5 => "fifteen",
            6 => "sixteen",
            7 => "seventeen",
            8 => "eighteen",
            9 => "nineteen",
            _ => ""
        })
    }

    fn write_out_tens(t: u64) -> String {
        String::from(match t {
            2 => "twenty",
            3 => "thirty",
            4 => "fourty",
            5 => "fifty",
            6 => "sixty",
            7 => "seventy",
            8 => "eighty",
            9 => "ninety",
            _ => ""
        })
    }

    fn write_out_scale(e: u64) -> String {
        if e == 0 {
            String::new()
        } else {
            String::from(match (e - 1) % 8 {
                0 => "thousand",
                1 => "million",
                2 => "billion",
                3 => "trillion",
                4 => "quadrillion",
                5 => "quintillion",
                6 => "sextillion",
                7 => "septillion",
                _ => ""
            })
        }
    }

    fn go(n: u64, e: u64) -> String {
        let hundreds = n % 1000 / 100;
        let tens     = n % 100 / 10;
        let ones     = n % 10;

        let mut number_string = String::new();

        if n / 1000 != 0 {
            number_string.push(' ');
        }
        
        if hundreds != 0 {
            number_string.push_str(write_out_digit(hundreds).as_str());
            number_string.push_str(" hundred");
            
            if tens + ones > 0 {
                number_string.push_str(" and ");
            }
            
        }

        if tens + ones > 0 {

            if tens == 1 {
                number_string.push_str(write_out_teens(ones).as_str());
            } else {
                if tens != 0 {
                    number_string.push_str(write_out_tens(tens).as_str());
                    if ones != 0 {
                        number_string.push(' ');
                    }
                }
                if ones != 0 {
                    number_string.push_str(write_out_digit(ones).as_str());
                }
            }
        }

        if e != 0 {
            number_string.push(' ');
            number_string.push_str(write_out_scale(e).as_str());
        }

        if n / 1000 == 0 {
            number_string
        } else {
            go(n/1000, e+1) + number_string.as_str()
        }
    }

    go(n, 0)
}

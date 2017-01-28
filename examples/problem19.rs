// Project Euler: Problem 18

fn main() {
    let d = Date::new(0, 0, 1900);

    let sundays_on_the_first: usize = (1901i64..2001)
        .map(|year| {
            (0u64..12)
                .filter(|&month| {
                    let dd = d.until(Date::new(0, month, year));
                    dd % 7 == 6
                })
                .count()
        })
        .sum();

    println!("There were {} sundays in the 1st of a month \
              between 01-01-1900 and 31-12-2000",
             sundays_on_the_first);
}

struct Date {
    eday: i64
}

impl Date {
    pub fn new(day: u64, month: u64, year: i64) -> Date {

        fn days_in_month(m: u64, year: i64) -> u64 {
            match m {
                0 => 31,
                1 => match
                    year % 4 == 0 &&
                    (year % 100 != 0 || year % 400 == 0) {
                        true  => 29,
                        false => 28
                    },
                2 => 31,
                3 => 30,
                4 => 31,
                5 => 30,
                6 => 31,
                7 => 31,
                8 => 30,
                9 => 31,
                10 => 30,
                11 => 31,
                _ => 0
            }
        }

        assert!(month < 12);
        assert!(day < days_in_month(month, year));

        let eday =
            year * 365 + year / 4 - year / 100 + year / 400 +
            (0..month+1).map(|m| days_in_month(m, year) as i64).sum::<i64>() + (day as i64);

        Date{eday: eday}
    }
    
    pub fn until(&self, d: Date) -> i64 {
        d.eday - self.eday
    }
}

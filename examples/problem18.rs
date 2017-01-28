// Project Euler: Problem 18

use std::cmp::max;

/* This only finds the maximum path sum and not the path itself, so it's a bit hard to check the result. Meh, maybe I'll amend it some day.
 */

fn main() {
    let triangle: Vec<Vec<u64>> =
        vec![vec![75],
             vec![95,64],
             vec![17,47,82],
             vec![18,35,87,10],
             vec![20,04,82,47,65],
             vec![19,01,23,75,03,34],
             vec![88,02,77,73,07,63,67],
             vec![99,65,04,28,06,16,70,92],
             vec![41,41,26,56,83,40,80,70,33],
             vec![41,48,72,33,47,32,37,16,94,29],
             vec![53,71,44,65,25,43,91,52,97,51,14],
             vec![70,11,33,28,77,73,17,78,39,68,17,57],
             vec![91,71,52,38,17,14,91,43,58,50,27,29,48],
             vec![63,66,04,68,89,53,67,30,73,16,69,87,40,31],
             vec![04,62,98,27,23,09,70,98,73,93,38,53,60,04,23]];

    let sum = triangle.last().map(|last| {
        let mut acc: Vec<u64> = last.clone();
        for r in triangle.iter().rev().skip(1) {
            acc = r.iter().zip(acc.iter().zip(acc.iter().skip(1)))
                .map(|(a, (b, c))| a + max(b, c))
                .collect();
        }

        acc[0]
    });

    for n in sum {
        println!("The maximum path sum through the triangle is {}.", n);
    }
}

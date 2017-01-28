// Project Euler: Problem 15

extern crate euler;
extern crate num;

use euler::util::n_choose_k;

/* This one requires some explaining.
 * To move through an n x n lattice requires exactly n moves
 * along the x-axis and n moves along the y-axis, hence
 * 
 *  dx_1 + dx_2 + ... + dx_2n = n,  dx_i in {0, 1}
 *  dy_1 + dy_2 + ... + dy_2n = n,  dy_i in {0, 1}
 *
 * where dx_i and dy_i are the increments along the respective axes
 * at move i. Since you move along exactly one axis with each move
 * the sequence of moves along one axis is completely dependent on
 * that of the other axis (i.e. dx_i + dy_i = 1). Therefore we need
 * only consider the moves along one axis. The problem thus boils
 * down to picking n moves in a total of 2n where we move along one
 * axis.
 * 
 * The number of ways to move through an n x n lattice is therefore
 * simply n_choose_k(2n, n)
 */

fn main() {
    println!("There are {} ways to move through an 20 x 20 lattice.",
             n_choose_k(40u64, 20));
}

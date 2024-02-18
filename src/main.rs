extern crate alloc;

mod check_unconstrained;
mod context;
mod field;
mod keccak_air;
mod round_flags_air;

// use keccak_air::{generate_trace_rows, KeccakAir};
use p3_baby_bear::BabyBear;
use p3_uni_stark::check_constraints;
// use rand::random;
use round_flags_air::{generate_trace_rows, RoundFlagsAir};

use crate::check_unconstrained::check_unconstrained;

// const NUM_HASHES: usize = 1;

fn main() {
    type Val = BabyBear;

    // let inputs = (0..NUM_HASHES).map(|_| random()).collect::<Vec<_>>();
    let trace = generate_trace_rows::<Val>();

    check_constraints(&RoundFlagsAir {}, &trace);
    check_unconstrained(&RoundFlagsAir {}, &trace)

    // check_constraints(&KeccakAir {}, &trace);
    // check_unconstrained(&KeccakAir {}, &trace)
}

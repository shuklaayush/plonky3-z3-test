mod check_unconstrained;
mod context;
mod field;
mod round_flags_air;

use p3_baby_bear::BabyBear;
use p3_uni_stark::check_constraints;
use round_flags_air::{generate_trace_rows, RoundFlagsAir};

use crate::check_unconstrained::check_unconstrained;

fn main() {
    type Val = BabyBear;

    let trace = generate_trace_rows::<Val>();

    check_constraints(&RoundFlagsAir {}, &trace);
    check_unconstrained(&RoundFlagsAir {}, &trace)
}

use core::borrow::Borrow;

use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::MatrixRowSlices;

use super::columns::KeccakCols;
use super::columns::NUM_KECCAK_COLS;
use super::NUM_ROUNDS;

pub struct KeccakAir {}

impl<F> BaseAir<F> for KeccakAir {
    fn width(&self) -> usize {
        NUM_KECCAK_COLS
    }
}

impl<AB: AirBuilder> Air<AB> for KeccakAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local: &KeccakCols<AB::Var> = main.row_slice(0).borrow();
        let next: &KeccakCols<AB::Var> = main.row_slice(1).borrow();

        // Initially, the first step flag should be 1 while the others should be 0.
        builder.when_first_row().assert_one(local.step_flags[0]);
        for i in 1..NUM_ROUNDS {
            builder.when_first_row().assert_zero(local.step_flags[i]);
        }

        for i in 0..NUM_ROUNDS {
            let current_round_flag = local.step_flags[i];
            let next_round_flag = next.step_flags[(i + 1) % NUM_ROUNDS];
            builder
                .when_transition()
                .assert_eq(next_round_flag, current_round_flag);
        }
    }
}

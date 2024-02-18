extern crate alloc;

use alloc::vec;

use p3_field::PrimeField64;
use p3_matrix::dense::RowMajorMatrix;

use super::columns::{KeccakCols, NUM_KECCAK_COLS};
use super::NUM_ROUNDS;

pub fn generate_trace_rows<F: PrimeField64>() -> RowMajorMatrix<F> {
    let num_rows = NUM_ROUNDS.next_power_of_two();
    let mut trace =
        RowMajorMatrix::new(vec![F::zero(); num_rows * NUM_KECCAK_COLS], NUM_KECCAK_COLS);
    let (prefix, rows, suffix) = unsafe { trace.values.align_to_mut::<KeccakCols<F>>() };
    assert!(prefix.is_empty(), "Alignment should match");
    assert!(suffix.is_empty(), "Alignment should match");
    assert_eq!(rows.len(), num_rows);

    for row in rows.chunks_mut(NUM_ROUNDS) {
        generate_trace_rows_for_perm(row);
    }

    trace
}

/// `rows` will normally consist of 24 rows, with an exception for the final row.
fn generate_trace_rows_for_perm<F: PrimeField64>(rows: &mut [KeccakCols<F>]) {
    generate_trace_row_for_round(&mut rows[0], 0);

    for round in 1..rows.len() {
        generate_trace_row_for_round(&mut rows[round], round);
    }
}

fn generate_trace_row_for_round<F: PrimeField64>(row: &mut KeccakCols<F>, round: usize) {
    row.step_flags[round] = F::one();
}
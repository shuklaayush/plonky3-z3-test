use p3_air::{Air, AirBuilder, TwoRowMatrixView};
use p3_field::{Field, PrimeField64};
use p3_matrix::dense::RowMajorMatrix;
use p3_matrix::{Matrix, MatrixRowSlices};

use crate::field::Felt;

pub fn check_constraints<F, A>(air: &A, main: &RowMajorMatrix<F>)
where
    F: Field,
    A: for<'a> Air<Z3ConstraintBuilder<'a, F>>,
{
    let height = main.height();

    (0..height).for_each(|i| {
        let i_next = (i + 1) % height;

        let main_local = main.row_slice(i);
        let main_next = main.row_slice(i_next);
        let main = TwoRowMatrixView {
            local: main_local,
            next: main_next,
        };

        let mut builder = Z3ConstraintBuilder {
            row_index: i,
            main,
            is_first_row: F::from_bool(i == 0),
            is_last_row: F::from_bool(i == height - 1),
            is_transition: F::from_bool(i != height - 1),
        };

        air.eval(&mut builder);
    });
}

/// An `AirBuilder` which asserts that each constraint is zero in Z3
pub struct Z3ConstraintBuilder<'a, F: Field> {
    row_index: usize,
    main: TwoRowMatrixView<'a, F>,
    is_first_row: F,
    is_last_row: F,
    is_transition: F,
}

impl<'a, F> AirBuilder for Z3ConstraintBuilder<'a, F>
where
    F: PrimeField64,
{
    type F = F;
    type Expr = Felt<'a, F>;
    type Var = Felt<'a, F>;
    type M = TwoRowMatrixView<'a, Self::Var>;

    fn is_first_row(&self) -> Self::Expr {
        self.is_first_row
    }

    fn is_last_row(&self) -> Self::Expr {
        self.is_last_row
    }

    fn is_transition_window(&self, size: usize) -> Self::Expr {
        if size == 2 {
            self.is_transition
        } else {
            panic!("only supports a window size of 2")
        }
    }

    fn main(&self) -> Self::M {
        self.main
    }

    fn assert_zero<I: Into<Self::Expr>>(&mut self, x: I) {
        assert_eq!(
            x.into(),
            F::zero(),
            "constraints had nonzero value on row {}",
            self.row_index
        );
    }

    fn assert_eq<I1: Into<Self::Expr>, I2: Into<Self::Expr>>(&mut self, x: I1, y: I2) {
        let x = x.into();
        let y = y.into();
        assert_eq!(
            x, y,
            "values didn't match on row {}: {} != {}",
            self.row_index, x, y
        );
    }
}
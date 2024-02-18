use p3_air::Air;
use p3_field::PrimeField64;
use p3_matrix::dense::RowMajorMatrix;
use p3_matrix::{Matrix, MatrixGet};
use p3_uni_stark::{SymbolicAirBuilder, SymbolicExpression};
use z3::ast::{Ast, Bool};
use z3::{SatResult, Solver};

use crate::context::context;
use crate::field::Felt;

pub fn check_unconstrained<F, A>(air: &A, main: &RowMajorMatrix<F>)
where
    F: PrimeField64,
    A: Air<SymbolicAirBuilder<F>>,
{
    let ctx = context();
    let solver = Solver::new(ctx);

    let width = main.width();
    let height = main.height();

    let mut builder = SymbolicAirBuilder::new(width);
    air.eval(&mut builder);
    let constraints = builder.constraints();

    let vars = RowMajorMatrix::new(
        (0..width * height)
            .map(|row| {
                Felt::<F>::new_const(&solver, format!("T[{}][{}]", row / width, row % width))
            })
            .collect(),
        width,
    );

    (0..height).for_each(|i| {
        constraints.iter().for_each(|constraint| {
            let exp = parse_symbolic_expression(constraint, &solver, &vars, i, height);
            exp.assert_zero(&solver);
        });
    });

    // Ignore trace as solution
    let solution = vars
        .values
        .iter()
        .zip(main.values.iter())
        .map(|(var, val)| var._eq(&Felt::from_u64(ctx, val.as_canonical_u64())).not())
        .collect::<Vec<_>>();
    solver.assert(&Bool::or(ctx, &solution));

    match solver.check() {
        SatResult::Sat => {
            let model = solver.get_model().unwrap();
            for i in 0..height {
                for j in 0..width {
                    print!("{} ", model.eval(&vars.get(i, j), true).unwrap());
                }
                println!()
            }
        }
        _ => println!("No solution"),
    }
}

fn parse_symbolic_expression<'ctx, F>(
    exp: &SymbolicExpression<F>,
    solver: &'ctx Solver<'ctx>,
    vars: &RowMajorMatrix<Felt<'ctx, F>>,
    row: usize,
    height: usize,
) -> Felt<'ctx, F>
where
    F: PrimeField64,
{
    match exp {
        SymbolicExpression::Variable(var) => {
            if var.is_next {
                vars.get((row + 1) % height, var.column)
            } else {
                vars.get(row, var.column)
            }
        }
        SymbolicExpression::IsFirstRow => Felt::from_bool(solver.get_context(), row == 0),
        SymbolicExpression::IsLastRow => Felt::from_bool(solver.get_context(), row == height - 1),
        SymbolicExpression::IsTransition => {
            Felt::from_bool(solver.get_context(), row != height - 1)
        }
        SymbolicExpression::Constant(f) => Felt::from_f(solver.get_context(), *f),
        SymbolicExpression::Add { x, y, .. } => {
            parse_symbolic_expression(x, solver, vars, row, height)
                + parse_symbolic_expression(y, solver, vars, row, height)
        }
        SymbolicExpression::Sub { x, y, .. } => {
            parse_symbolic_expression(x, solver, vars, row, height)
                - parse_symbolic_expression(y, solver, vars, row, height)
        }
        SymbolicExpression::Neg { x, .. } => {
            -parse_symbolic_expression(x, solver, vars, row, height)
        }
        SymbolicExpression::Mul { x, y, .. } => {
            parse_symbolic_expression(x, solver, vars, row, height)
                * parse_symbolic_expression(y, solver, vars, row, height)
        }
    }
}

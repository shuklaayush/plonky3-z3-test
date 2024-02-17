mod check_constraints;
mod context;
mod field;

use context::context;
use field::Felt;
use p3_goldilocks::Goldilocks;
use z3::{ast::*, *};

const NUM_ROUNDS: usize = 24;

pub fn main() {
    let ctx = context();
    let solver = Solver::new(ctx);

    let step_flags: [Felt<Goldilocks>; NUM_ROUNDS] = (0..NUM_ROUNDS)
        .map(|i| Felt::new_const(&solver, format!("F[{i}]")))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let zero = Felt::from_u64(0);
    let one = Felt::from_u64(1);
    solver.assert(&(&step_flags[0] - &one)._eq(&zero));
    for i in 1..NUM_ROUNDS {
        solver.assert(&step_flags[i]._eq(&zero));
    }

    // Fill trace
    // let mut trace = Vec::with_capacity(NUM_ROUNDS);
    // trace.push(step_flags[0]._eq(&one).not());
    // for i in 1..NUM_ROUNDS {
    //     trace.push(step_flags[i]._eq(&zero).not());
    // }
    // solver.assert(&Bool::or(&ctx, &trace));

    match solver.check() {
        SatResult::Sat => {
            let model = solver.get_model().unwrap();
            for i in 0..NUM_ROUNDS {
                print!("{} ", model.eval(&step_flags[i], true).unwrap());
            }
            println!();
        }
        _ => println!("No solution"),
    }
}

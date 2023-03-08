use russol_contracts::*;

#[pure]
fn is_ok<T, V>(x: &Result<T, V>) -> bool {
  matches!(x, Ok(_))
}

// This annotation restricts synthesis to this function
#[synth]
#[ensures(result.0 == is_ok(x))]
fn foo<T, V>(x: &mut Result<T, V>) -> (bool, Result<&mut V, &mut T>) {
    todo!()
}

#[ensures(result == x as i32 + y as i32)]
fn bar(x: i8, y: i8) -> i32 {
  todo!()
}

#[extern_spec]
#[ensures(matches!(^x, None))]
#[ensures(*x === result)]
fn take<T>(x: &mut Option<T>) -> Option<T> {
  std::mem::take(x)
}

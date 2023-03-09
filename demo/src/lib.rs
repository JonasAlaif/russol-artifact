use russol_contracts::*;

// Example pure function
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

// This function will not be synthesized (no #[synth] annotation)
#[ensures(result == x as i32 + y as i32)]
fn bar(x: i8, y: i8) -> i32 {
    todo!()
}

#[pure]
fn is_none<T>(x: &Option<T>) -> bool {
    matches!(x, None)
}

// Example function which can be used (called)
// by synthesized implementations
#[extern_spec]
#[ensures(is_none(&^x))]
#[ensures(*x === result)]
fn take<T>(x: &mut Option<T>) -> Option<T> {
    std::mem::take(x)
}

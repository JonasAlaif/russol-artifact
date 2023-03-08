use russol_contracts::*;

#[ensures(result.0 == matches!(x, Ok(_)))]
fn foo<T, V>(x: &mut Result<T, V>) -> (bool, Result<&mut V, &mut T>) {
    todo!()
}

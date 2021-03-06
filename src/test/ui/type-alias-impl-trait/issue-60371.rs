// ignore-compare-mode-chalk

trait Bug {
    type Item: Bug;

    const FUN: fn() -> Self::Item;
}

impl Bug for &() {
    type Item = impl Bug; //~ ERROR `impl Trait` in type aliases is unstable
    //~^ ERROR the trait bound `(): Bug` is not satisfied
    //~^^ ERROR the trait bound `(): Bug` is not satisfied

    const FUN: fn() -> Self::Item = || ();
    //~^ ERROR type alias impl trait is not permitted here
}

fn main() {}

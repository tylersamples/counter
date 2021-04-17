fn new() -> () {
    ()
}

#[rustler::nif]
fn increment(counter: ()) -> () {
    ()
}

#[rustler::nif]
fn decrement(counter: ()) -> () {
    ()
}
#[rustler::nif]
fn read(a: ()) -> i64 {
    0
}

rustler::init!("Elixir.Counter", [new, increment, decrement, read]);

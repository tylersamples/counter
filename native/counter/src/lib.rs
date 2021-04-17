use rustler::Env;
use rustler::ResourceArc;
use std::mem::drop;
use std::sync::Mutex;

struct Counter {
    num: Mutex<i64>,
}

type CounterArc = ResourceArc<Counter>;

#[rustler::nif]
fn new() -> CounterArc {
    ResourceArc::new(Counter { num: Mutex::new(0) })
}

#[rustler::nif]
fn increment(counter: CounterArc) -> CounterArc {
    let mut counter_lock = counter.num.lock().unwrap();
    *counter_lock = *counter_lock + 1;

    drop(counter_lock);

    counter
}

#[rustler::nif]
fn decrement(counter: CounterArc) -> CounterArc {
    let mut counter_lock = counter.num.lock().unwrap();
    *counter_lock = *counter_lock - 1;

    drop(counter_lock);

    counter
}

#[rustler::nif]
fn read(counter: CounterArc) -> i64 {
    let counter_lock = counter.num.lock().unwrap();
    let data = *counter_lock;

    drop(counter_lock);

    data
}

rustler::init!(
    "Elixir.Counter",
    [new, increment, decrement, read],
    load = |env: Env, _| {
        rustler::resource!(Counter, env);
        true
    }
);

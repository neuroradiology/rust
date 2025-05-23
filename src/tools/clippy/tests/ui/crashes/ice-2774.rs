use std::collections::HashSet;

// See rust-lang/rust-clippy#2774.

#[derive(Eq, PartialEq, Debug, Hash)]
pub struct Bar {
    foo: Foo,
}

#[derive(Eq, PartialEq, Debug, Hash)]
pub struct Foo;

#[allow(clippy::implicit_hasher)]
// This should not cause a "cannot relate bound region" ICE.
pub fn add_barfoos_to_foos<'a>(bars: &HashSet<&'a Bar>) {
    //~^ needless_lifetimes

    let mut foos = HashSet::new();
    foos.extend(bars.iter().map(|b| &b.foo));
}

#[allow(clippy::implicit_hasher)]
// Also, this should not cause a "cannot relate bound region" ICE.
pub fn add_barfoos_to_foos2(bars: &HashSet<&Bar>) {
    let mut foos = HashSet::new();
    foos.extend(bars.iter().map(|b| &b.foo));
}

fn main() {}

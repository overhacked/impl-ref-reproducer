#![allow(dead_code)]

#[derive(Clone)]
struct ThingThatDoesAThing {}

trait DoesAThing {}

impl DoesAThing for ThingThatDoesAThing {}

#[cfg(feature = "bad_error")]
fn clones_impl_ref_inline(thing: &impl DoesAThing) {
    drops_impl_owned(thing.clone());
}

#[cfg(feature = "better_error")]
fn clones_impl_ref_statement(thing: &impl DoesAThing) {
    let thing_clone = thing.clone();
    drops_impl_owned(thing_clone);
}

#[cfg(feature = "good_error")]
fn clones_and_drops_impl_ref(thing: &impl DoesAThing) {
    let _ = thing.clone();
}

fn correct_usage<T>(thing: &T)
where
    T: DoesAThing + Clone
{
    drops_impl_owned(thing.clone());
}

fn drops_impl_owned(_thing: impl DoesAThing) { }

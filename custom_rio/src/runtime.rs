//! Here live the runtime implementation of the crate
// code source https://github.com/vincenzopalazzo/rio/pull/15/files#diff-0fa9d453eacf7c8cfca82ff169556a166e1c2fec6bee4132c16e0bac76c094c7
use std::future::Future;

pub trait Runtime {
    fn new() -> &'static Self;

    fn block_on(&self, future: impl Future<Output = ()> + Send + 'static);
}

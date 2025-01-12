// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-flags: --mir-linker --enable-unstable
//! Check the basic coercion when using boxes.
//! Tests are broken down into different crates to ensure that the reachability works for each case.
//! FIXME: Drop of struct with dyn member unsupported for now.
//! https://github.com/model-checking/kani/issues/1072

mod defs;
use defs::*;

#[kani::proof]
fn check_inner_dyn_coercion() {
    let inner_id = kani::any();
    let outer_id = kani::any();
    let outer: Box<Outer<dyn Identity>> =
        Box::new(Outer { inner: Inner { id: inner_id }, outer_id });
    assert_eq!(id_from_dyn(&outer.inner), inner_id.into());
}

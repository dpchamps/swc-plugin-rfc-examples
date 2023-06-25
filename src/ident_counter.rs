/*
Toy example of a visitor with module-level state
 */

use once_cell::sync::Lazy;
use std::sync::Mutex;
use swc_ecma_ast::Ident;
use swc_ecma_visit::{noop_visit_mut_type, VisitMut};

/// Toy example of a visitor with module-level state

pub struct IdentVisitor;

static IDENT_COUNT: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

impl VisitMut for IdentVisitor {
    noop_visit_mut_type!();

    fn visit_mut_ident(&mut self, _module: &mut Ident) {
        let mut guard = IDENT_COUNT.lock().expect("Failed to lock");
        println!("{}", *guard);
        *guard += 1;
    }
}

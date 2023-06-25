/*
Toy example of a visitor with module-level state
 */

use std::collections::HashSet;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use swc_ecma_ast::Ident;
use swc_ecma_visit::{noop_visit_mut_type, VisitMut, VisitMutWith};
use rand::{prelude, Rng};

/// Toy example of a visitor with module-level state

static IDENT_COUNT: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

static RAND_ID: Lazy<String> = Lazy::new(|| {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..100).to_string()
});

#[derive(Default)]
pub struct IdentVisitor {
    renamed: HashSet<String>
}


impl VisitMut for IdentVisitor {
    noop_visit_mut_type!();

    fn visit_mut_ident(&mut self, ident: &mut Ident) {
        let name = ident.sym.to_string();

        if self.renamed.contains(&name) {
            return
        }

        let mut guard = IDENT_COUNT.lock().expect("Failed to lock");

        ident.sym = format!("{}_{}_{}", name, *RAND_ID, *guard).into();

        self.renamed.insert(name);

        *guard += 1;
    }
}

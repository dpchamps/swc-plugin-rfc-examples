use std::{path::Path, sync::Arc};
use std::sync::Mutex;
use swc::{self, config::Options};
use swc::atoms::once_cell::sync::Lazy;
use swc_common::{errors::{ColorConfig, Handler}, SourceMap, Globals, GLOBALS, SourceFile};
use swc_ecma_ast::{EsVersion, Ident};
use swc_ecma_transforms::pass::noop;
use swc_ecma_visit::{as_folder, noop_visit_mut_type, VisitMut};
use glob::{glob, GlobError};
use swc::config::{Config, JscConfig, ModuleConfig};

///

fn main() {
    let globals = Globals::new();
    GLOBALS.set(&globals,|| {
        let cm = Arc::<SourceMap>::default();
        let handler = Arc::new(Handler::with_tty_emitter(
            ColorConfig::Auto,
            true,
            false,
            Some(cm.clone()),
        ));
        let compiler = swc::Compiler::new(cm.clone());

        let files: Result<Vec<Arc<SourceFile>>, _> = glob("./js-fixtures/*.js").expect("Failed to glob files").into_iter().map(|glob_result| {
            glob_result.map(|path| {
                cm.load_file(&path.into_boxed_path())
            }).expect("Received a glob error")
        }).collect();

        for fm in files.expect("Failed reading files") {
            compiler
                .process_js_with_custom_pass(
                    fm,
                    None,
                    &handler,
                    &Options {
                        config: Config {
                            jsc: JscConfig {
                                target: Some(EsVersion::Es5),
                                ..Default::default()
                            },
                            module: Some(ModuleConfig::CommonJs(Default::default())),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Default::default(),
                    |_| noop(),
                    |_| as_folder(IdentVisitor),
                )
                .expect("Failed to compile");
        }
    });
}


/*
Toy example of a visitor with module-level state
 */

struct IdentVisitor;

static IDENT_COUNT: Lazy<Mutex<usize>> =
    Lazy::new(|| Mutex::new(0));

impl VisitMut for IdentVisitor {
    noop_visit_mut_type!();

    fn visit_mut_ident(&mut self, module: &mut Ident) {
        let mut guard = IDENT_COUNT.lock().expect("Failed to lock");
        println!("{}", *guard);
        *guard += 1;
    }
}

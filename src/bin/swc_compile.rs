use glob::glob;
use std::sync::Arc;

use swc::config::{Config, JscConfig, ModuleConfig};
use swc::{self, config::Options};
use swc_common::{
    errors::{ColorConfig, Handler},
    Globals, SourceFile, SourceMap, GLOBALS,
};
use swc_ecma_ast::EsVersion;
use swc_ecma_transforms::pass::noop;
use swc_ecma_visit::as_folder;

use swc_plugin_rfc_examples::ident_counter::IdentVisitor;

fn main() {
    let globals = Globals::new();
    GLOBALS.set(&globals, || {
        let cm = Arc::<SourceMap>::default();
        let handler = Arc::new(Handler::with_tty_emitter(
            ColorConfig::Auto,
            true,
            false,
            Some(cm.clone()),
        ));
        let compiler = swc::Compiler::new(cm.clone());

        let files: Result<Vec<Arc<SourceFile>>, _> = glob("./js-fixtures/*.js")
            .expect("Failed to glob files")
            .map(|glob_result| {
                glob_result
                    .map(|path| cm.load_file(&path.into_boxed_path()))
                    .expect("Received a glob error")
            })
            .collect();

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

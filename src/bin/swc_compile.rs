use std::{path::Path, sync::Arc};
use swc::{self, config::Options};
use swc_common::{
    errors::{ColorConfig, Handler},
    SourceMap,
};
use swc_ecma_transforms::pass::noop;

fn main() {
    let cm = Arc::<SourceMap>::default();
    let handler = Arc::new(Handler::with_tty_emitter(
        ColorConfig::Auto,
        true,
        false,
        Some(cm.clone()),
    ));
    let compiler = swc::Compiler::new(cm.clone());

    let fm = cm
        // filepath that actually exists relative to the binary
        .load_file(Path::new("src/pages/index.js"))
        .expect("failed to load file");

    let result = compiler.process_js_with_custom_pass(fm, None, &handler, &Options {
        ..Default::default()
    }, Default::default(), |_| noop(), |_| noop()).expect("Failed to compile");
}

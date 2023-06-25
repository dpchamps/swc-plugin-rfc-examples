use std::{env, fs};
use glob::glob;
use std::sync::Arc;

use swc::config::{Config, JscConfig, ModuleConfig};
use swc::{self, Compiler, config::Options};
use swc_common::{errors::{ColorConfig, Handler}, Globals, SourceFile, SourceMap, GLOBALS, FileName};
use swc_ecma_ast::EsVersion;
use swc_ecma_transforms::pass::noop;
use swc_ecma_visit::as_folder;
use anyhow::Error;
use ident_counter::IdentVisitor;
use fs::{create_dir_all, write};
use std::fmt::format;
use std::path::Path;

fn read_files(input: &str, cm: &Arc<SourceMap>) -> Result<Vec<Arc<SourceFile>>, Error> {
    glob(input)?
        .map(|glob_result| {
            glob_result
                .map(|path| cm.load_file(&path.into_boxed_path()).map_err(Error::from)).map_err(Error::from)?
        })
        .collect()
}

fn compiler_config() -> Options {
    Options {
        config: Config {
            jsc: JscConfig {
                target: Some(EsVersion::Es5),
                ..Default::default()
            },
            module: Some(ModuleConfig::CommonJs(Default::default())),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn main() {
    let globals = Globals::new();
    let file_glob = env::args().nth(1).expect("Pass glob string as first arg");
    let dest_folder_arg = env::args().nth(2).expect("Pass dest folder as second arg");
    let dest_folder = Path::new(&dest_folder_arg);

    GLOBALS.set(&globals, || {
        let cm = Arc::<SourceMap>::default();
        let handler = Arc::new(Handler::with_tty_emitter(
            ColorConfig::Auto,
            true,
            false,
            Some(cm.clone()),
        ));
        let compiler = swc::Compiler::new(cm.clone());

        let files = read_files(&file_glob, &cm).expect("Failed to read files");

        let results = files.into_iter().map(|file| {
            (
                file.name.to_string(),
                compiler
                .process_js_with_custom_pass(
                    file,
                    None,
                    &handler,
                    &compiler_config(),
                    Default::default(),
                    |_| noop(),
                    |_| as_folder(IdentVisitor::default()),
                )
                .expect("Failed to compile")
                .code
            )
        }).collect::<Vec<(String, String)>>();

        create_dir_all(dest_folder).expect("Failed to create output dir");

        for (filename, compiled_code) in results {
            let file_part = Path::new(&filename).file_name().expect("Expected to find a file");
            let out_path = dest_folder.join(file_part);
            write(out_path, compiled_code).expect("Failed to write a file during compilation");
        }

    });
}

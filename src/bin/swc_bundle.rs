use anyhow::Error;
use std::collections::HashMap;
use std::sync::Arc;
use swc::atoms::js_word;
use swc_bundler::{Bundler, Load, ModuleData, ModuleRecord};
use swc_common::errors::{ColorConfig, Handler};
use swc_common::sync::Lrc;
use swc_common::{FileName, SourceMap, Span};
use swc_ecma_ast::*;
use swc_ecma_loader::resolvers::lru::CachingResolver;
use swc_ecma_loader::resolvers::node::NodeModulesResolver;
use swc_ecma_loader::TargetEnv;
use swc_ecma_parser::{parse_file_as_module, Syntax};

fn main() {
    let cm = Arc::<SourceMap>::default();
    let entries = HashMap::from([(
        String::from("index"),
        FileName::Real("./js-fixtures/index.js".into()),
    )]);
    let globals = Box::leak(Box::default());
    let mut bundler = Bundler::new(
        globals,
        cm.clone(),
        Loader { cm },
        CachingResolver::new(
            4096,
            NodeModulesResolver::new(TargetEnv::Node, Default::default(), true),
        ),
        swc_bundler::Config {
            require: false,
            disable_inliner: true,
            external_modules: Default::default(),
            disable_fixer: true,
            disable_hygiene: true,
            disable_dce: false,
            module: Default::default(),
        },
        Box::new(Hook),
    );

    let result = bundler.bundle(entries).expect("Could not bundle");

    println!("{:?}", result)
}

pub struct Loader {
    pub cm: Lrc<SourceMap>,
}

impl Load for Loader {
    fn load(&self, f: &FileName) -> Result<ModuleData, Error> {
        let fm = match f {
            FileName::Real(path) => self.cm.load_file(path)?,
            _ => unreachable!(),
        };

        // let compiler = swc::Compiler::new(cm.clone());
        //
        // compiler.process_js_with_custom_pass();

        let module = parse_file_as_module(
            &fm,
            Syntax::Es(Default::default()),
            EsVersion::Es2020,
            None,
            &mut vec![],
        )
        .unwrap_or_else(|err| {
            let handler =
                Handler::with_tty_emitter(ColorConfig::Always, false, false, Some(self.cm.clone()));
            err.into_diagnostic(&handler).emit();
            panic!("failed to parse")
        });

        Ok(ModuleData {
            fm,
            module,
            helpers: Default::default(),
        })
    }
}

struct Hook;

impl swc_bundler::Hook for Hook {
    fn get_import_meta_props(
        &self,
        span: Span,
        module_record: &ModuleRecord,
    ) -> Result<Vec<KeyValueProp>, Error> {
        let file_name = module_record.file_name.to_string();

        Ok(vec![
            KeyValueProp {
                key: PropName::Ident(Ident::new(js_word!("url"), span)),
                value: Box::new(Expr::Lit(Lit::Str(Str {
                    span,
                    raw: None,
                    value: file_name.into(),
                }))),
            },
            KeyValueProp {
                key: PropName::Ident(Ident::new(js_word!("main"), span)),
                value: Box::new(if module_record.is_entry {
                    Expr::Member(MemberExpr {
                        span,
                        obj: Box::new(Expr::MetaProp(MetaPropExpr {
                            span,
                            kind: MetaPropKind::ImportMeta,
                        })),
                        prop: MemberProp::Ident(Ident::new(js_word!("main"), span)),
                    })
                } else {
                    Expr::Lit(Lit::Bool(Bool { span, value: false }))
                }),
            },
        ])
    }
}

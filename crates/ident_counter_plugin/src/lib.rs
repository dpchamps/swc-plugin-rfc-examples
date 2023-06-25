use swc_ecma_visit::{as_folder, FoldWith, VisitMut};
use swc_plugin_macro::plugin_transform;
use swc_ecma_ast::Program;
use ident_counter::IdentVisitor;
use swc_plugin_proxy::TransformPluginProgramMetadata;

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(IdentVisitor::default()))
}
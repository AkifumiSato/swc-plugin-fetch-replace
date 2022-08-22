use serde::{Deserialize, Serialize};
use swc_core::{
    ast::Program,
    ast::*,
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
    visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
};

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Function name to replace
    #[serde()]
    pub replace_name: String,
}

pub struct TransformVisitor {
    config: Config,
}

impl TransformVisitor {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl VisitMut for TransformVisitor {
    fn visit_mut_callee(&mut self, callee: &mut Callee) {
        callee.visit_mut_children_with(self);

        if let Callee::Expr(expr) = callee {
            if let Expr::Member(parent) = &mut **expr {
                if let Expr::Ident(i) = &mut *parent.obj {
                    if &*i.sym == "window" || &*i.sym == "globalThis" {
                        if let MemberProp::Ident(i) = &mut parent.prop {
                            if &*i.sym == "fetch" {
                                let replace_name: &str = &self.config.replace_name;
                                i.sym = replace_name.into();
                            }
                        }
                    }
                }
            }

            if let Expr::Ident(i) = &mut **expr {
                if &*i.sym == "fetch" {
                    let replace_name: &str = &self.config.replace_name;
                    i.sym = replace_name.into();
                }
            }
        }
    }
}

/// An example plugin function with macro support.
/// `plugin_transform` macro interop pointers into deserialized structs, as well
/// as returning ptr back to host.
///
/// It is possible to opt out from macro by writing transform fn manually
/// if plugin need to handle low-level ptr directly via
/// `__transform_plugin_process_impl(
///     ast_ptr: *const u8, ast_ptr_len: i32,
///     unresolved_mark: u32, should_enable_comments_proxy: i32) ->
///     i32 /*  0 for success, fail otherwise.
///             Note this is only for internal pointer interop result,
///             not actual transform result */`
///
/// This requires manual handling of serialization / deserialization from ptrs.
/// Refer swc_plugin_macro to see how does it work internally.
#[plugin_transform]
pub fn process_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<Config>(
        &metadata
            .get_transform_plugin_config()
            .expect("failed to parse plugin config"),
    )
    .expect("invalid plugin config");

    program.fold_with(&mut as_folder(TransformVisitor::new(config)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use swc_core::testing_transform::test;

    fn test_visiter() -> TransformVisitor {
        let config = Config {
            replace_name: String::from("my_test_fetch"),
        };
        TransformVisitor { config }
    }

    test!(
        Default::default(),
        |_| as_folder(test_visiter()),
        replace_fetch,
        // Input codes
        r#"
        const res = await fetch('http://localhost:9999');
        "#,
        // Output codes after transformed with plugin
        r#"
        const res = await my_test_fetch('http://localhost:9999');
        "#
    );

    test!(
        Default::default(),
        |_| as_folder(test_visiter()),
        global_this_fetch,
        // Input codes
        r#"
        const res = await globalThis.fetch('http://localhost:9999');
        "#,
        // Output codes after transformed with plugin
        r#"
        const res = await globalThis.my_test_fetch('http://localhost:9999');
        "#
    );

    test!(
        Default::default(),
        |_| as_folder(test_visiter()),
        widow_fetch,
        // Input codes
        r#"
        const res = await window.fetch('http://localhost:9999');
        "#,
        // Output codes after transformed with plugin
        r#"
        const res = await window.my_test_fetch('http://localhost:9999');
        "#
    );

    test!(
        Default::default(),
        |_| as_folder(test_visiter()),
        not_replace_fetch,
        // Input codes
        r#"
        const res = await custom_fetch('http://localhost:9999');
        "#,
        // Output codes after transformed with plugin
        r#"
        const res = await custom_fetch('http://localhost:9999');
        "#
    );
}

use zed_extension_api::{self as zed, lsp::CompletionKind, Result};

struct C3Extension;

impl zed::Extension for C3Extension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let c3lsp_cmd = worktree.which("c3lsp");
        let path = c3lsp_cmd.ok_or_else(|| "c3lsp must be in your path".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: Default::default(),
        })
    }

    fn label_for_completion(
        &self,
        _language_server_id: &zed::LanguageServerId,
        completion: zed::lsp::Completion,
    ) -> Option<zed::CodeLabel> {
        // Abuse syntax highlighting for a nice colored colon
        let precolon = "int x = s(a";
        let colon = ": ";
        let postcolon = " 5);\n";
        let colon_prelude = format!("{precolon}{colon}{postcolon}\n");

        // precolon(: )postcolon etc
        let colon_span = zed::CodeLabelSpan::code_range({
            let start = precolon.len();
            start..start + colon.len()
        });

        let name = &completion.label;
        let detail = &completion.detail?;

        let (code, spans) = match completion.kind? {
            // For these completion kinds, 'detail' is a type
            kind @ (CompletionKind::Variable
            | CompletionKind::Field
            | CompletionKind::Method
            | CompletionKind::Function) => {
                // Use a 'def' to syntax highlight a type in the RHS
                let mut ty_prelude = "def Mt = ";
                let ty = detail;
                let mut ty_post = ";\nint a = ";
                let mut call = "";

                if matches!(kind, CompletionKind::Function | CompletionKind::Method) {
                    call = "()";
                    if ty.starts_with("macro") {
                        // Must add '{}' to highlight macro:
                        // 'macro void(int a, int b){}'
                        ty_prelude = "";
                        ty_post = "{}\nint a = ";
                    }
                }

                let code = format!("{colon_prelude}{ty_prelude}{ty}{ty_post}{name}{call};");

                (
                    code,
                    vec![
                        // (colonstuff def Mt = ty; int a = )name(;)
                        zed::CodeLabelSpan::code_range({
                            let start =
                                colon_prelude.len() + ty_prelude.len() + ty.len() + ty_post.len();
                            start..start + name.len()
                        }),
                        // :
                        colon_span,
                        // (colonstuff def Mt = )ty(; int a = name;)
                        zed::CodeLabelSpan::code_range({
                            let start = colon_prelude.len() + ty_prelude.len();
                            start..start + ty.len()
                        }),
                    ],
                )
            }

            // For other completion kinds, 'detail' is arbitrary text
            _ => {
                // Some invalid stuff to turn off highlighting
                let detail_prelude = "; ])>)";
                let name_prelude = "int a = ";
                let code = format!("{colon_prelude}{name_prelude}{name}{detail_prelude}{detail}");

                (
                    code,
                    vec![
                        // (colonstuff int a = )name(; invalidstuff detail)
                        zed::CodeLabelSpan::code_range({
                            let start = colon_prelude.len() + name_prelude.len();
                            start..start + name.len()
                        }),
                        // :
                        colon_span,
                        // (colonstuff int a = name; invalidstuff)detail
                        zed::CodeLabelSpan::code_range({
                            let start = colon_prelude.len()
                                + name_prelude.len()
                                + name.len()
                                + detail_prelude.len();
                            start..start + detail.len()
                        }),
                    ],
                )
            }
        };

        Some(zed::CodeLabel {
            spans,
            filter_range: (0..name.len()).into(),
            code,
        })
    }
}

zed::register_extension!(C3Extension);

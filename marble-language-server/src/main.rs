use dashmap::DashMap;
use line_index::{LineCol, TextRange};
use marble::compiler::Compiler;
use marble::error::AnnotatedError;
use marble::expr::ExprRef;
use marble::scanner::Scanner;
use marble::source::Source;
use tower_lsp::jsonrpc::Error;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
    document_map: DashMap<Url, (String, Result<ExprRef, AnnotatedError>)>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult, Error> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(true),
                        })),
                        ..Default::default()
                    },
                )),

                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    completion_item: None,
                }),

                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),

                ..ServerCapabilities::default()
            },
        })
    }

    async fn completion(
        &self,
        params: CompletionParams,
    ) -> Result<Option<CompletionResponse>, Error> {
        Ok(None)
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.file_changed(params.text_document.uri, params.text_document.text)
            .await
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.file_changed(
            params.text_document.uri,
            params.content_changes[0].text.clone(),
        )
        .await
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        if let Some(content) = params.text {
            self.file_changed(params.text_document.uri, content).await
        }
    }

    async fn shutdown(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Backend {
    async fn file_changed(&self, uri: Url, content: String) {
        let source = Source::new(&content);
        let scanner = Scanner::new(&source);

        let mut compiler = Compiler::new(&source, scanner);
        compiler.with_bindings(Compiler::default_bindings());
        let expr = compiler.compile();

        if let Err(error) = &expr {
            let msg = error.of_source(&source);

            let diagnostic = Diagnostic {
                message: msg,
                severity: Some(DiagnosticSeverity::ERROR),
                range: textrange_to_range(source.start(&error.token), source.end(&error.token)),
                ..Default::default()
            };

            self.client
                .publish_diagnostics(uri.clone(), vec![diagnostic], None)
                .await
        } else {
            // Clear diagnostics
            self.client
                .publish_diagnostics(uri.clone(), vec![], None)
                .await
        }

        self.document_map.insert(uri, (content, expr));
    }
}

fn textrange_to_range(start: LineCol, end: LineCol) -> Range {
    Range {
        start: Position::new(start.line, start.col),
        end: Position::new(end.line, end.col),
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| Backend {
        client,
        document_map: DashMap::new(),
    })
    .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}

import path = require("path");
import {
    workspace,
    ExtensionContext,
    window,
} from "vscode";

import {
    Executable,
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind,
} from "vscode-languageclient/node";

let client: LanguageClient;

export async function activate(context: ExtensionContext) {
    const command = context.asAbsolutePath(path.join("server", "bin", "marble-language-server"));
    const run: Executable = {
        command,
        transport: TransportKind.stdio,
        options: {

        },
    };

    const serverOptions: ServerOptions = {
        run,
        debug: run,
    };
    // If the extension is launched in debug mode then the debug server options are used
    // Otherwise the run options are used
    // Options to control the language client
    let clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: "file", language: "marble" }],
        synchronize: {
            // Notify the server about file changes to '.clientrc files contained in the workspace
            fileEvents: workspace.createFileSystemWatcher("**/.clientrc"),
        }
    };

    // Create the language client and start the client.
    client = new LanguageClient("marble-language-support", "Marble Language Server", serverOptions, clientOptions);

    client.start();
}

export function deactivate(): Thenable<void> | undefined {
    return client?.stop();
}
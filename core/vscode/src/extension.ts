import * as vscode from 'vscode';
import { execSync } from 'child_process';
import * as path from 'path';

const languageMap: { [key: string]: string } = {
    'python': 'python',
    'javascript': 'javascript',
    'typescript': 'javascript',
    'bash': 'bash',
    'shell': 'bash',
};

export function activate(context: vscode.ExtensionContext) {
    const config = vscode.workspace.getConfiguration('olaforge');
    const binaryPath = config.get<string>('binaryPath', 'olaforge');
    const securityLevel = config.get<string>('securityLevel', 'L2');
    const timeout = config.get<number>('timeout', 60);

    // Run code command
    const runCodeCmd = vscode.commands.registerCommand('olaforge.runCode', async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showErrorMessage('No active editor');
            return;
        }

        const document = editor.document;
        const code = document.getText();
        const language = languageMap[document.languageId] || 'python';

        try {
            const result = executeCode(binaryPath, code, language, securityLevel, timeout);
            showResult(result);
        } catch (error: any) {
            vscode.window.showErrorMessage(`Execution failed: ${error.message}`);
        }
    });

    // Run selection command
    const runSelectionCmd = vscode.commands.registerCommand('olaforge.runSelection', async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showErrorMessage('No active editor');
            return;
        }

        const selection = editor.selection;
        const code = editor.document.getText(selection);
        
        if (!code.trim()) {
            vscode.window.showWarningMessage('No selected code');
            return;
        }

        const language = languageMap[editor.document.languageId] || 'python';

        try {
            const result = executeCode(binaryPath, code, language, securityLevel, timeout);
            showResult(result);
        } catch (error: any) {
            vscode.window.showErrorMessage(`Execution failed: ${error.message}`);
        }
    });

    // Set security level command
    const setSecurityCmd = vscode.commands.registerCommand('olaforge.setSecurityLevel', async () {
        const level = await vscode.window.showQuickPick(['L0', 'L1', 'L2', 'L3'], {
            placeHolder: 'Select security level',
        });

        if (level) {
            await config.update('securityLevel', level, vscode.ConfigurationTarget.Global);
            vscode.window.showInformationMessage(`Security level set to ${level}`);
        }
    });

    context.subscriptions.push(runCodeCmd, runSelectionCmd, setSecurityCmd);

    // Add status bar item
    const statusBar = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBar.text = `OlaForge: ${securityLevel}`;
    statusBar.command = 'olaforge.setSecurityLevel';
    statusBar.tooltip = 'Click to change security level';
    statusBar.show();

    context.subscriptions.push(statusBar);
}

function executeCode(
    binaryPath: string,
    code: string,
    language: string,
    securityLevel: string,
    timeout: number
): string {
    const args = [
        'execute',
        '--code', code,
        '--language', language,
        '--security', securityLevel,
        '--timeout', timeout.toString()
    ];

    const result = execSync(`${binaryPath} ${args.join(' ')}`, {
        encoding: 'utf8',
        timeout: timeout * 1000 + 5000,
    });

    return result;
}

function showResult(result: string) {
    try {
        const data = JSON.parse(result);
        
        if (data.success) {
            const outputChannel = vscode.window.createOutputChannel('OlaForge Output');
            outputChannel.clear();
            outputChannel.appendLine('=== Output ===');
            outputChannel.appendLine(data.output || '(no output)');
            
            if (data.sandbox?.issues?.length > 0) {
                outputChannel.appendLine('');
                outputChannel.appendLine('=== Security Issues ===');
                data.sandbox.issues.forEach((issue: string) => {
                    outputChannel.appendLine(issue);
                });
            }
            
            outputChannel.show();
        } else {
            vscode.window.showErrorMessage(data.error || 'Execution failed');
        }
    } catch {
        vscode.window.showErrorMessage(`Invalid result: ${result}`);
    }
}

export function deactivate() {}
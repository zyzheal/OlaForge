export interface ExecutionResultData {
  success: boolean;
  output: string;
  error?: string;
  execution_time_ms: number;
  sandbox: {
    enabled: boolean;
    level: string;
    scanned: boolean;
    passed: boolean;
    risk_level: string;
    issues_count: number;
    issues: string[];
    recommendations: string[];
  };
  language: string;
  scan_timestamp: string;
  security_issues?: string[];
}

export class ExecutionResult {
  readonly success: boolean;
  readonly output: string;
  readonly error: string | null;
  readonly executionTimeMs: number;
  readonly sandbox: ExecutionResultData['sandbox'];
  readonly riskLevel: string;
  readonly passed: boolean;
  readonly securityIssues: string[];

  constructor(data: ExecutionResultData);
  toJSON(): ExecutionResultData;
}

export class SkillInfo {
  readonly name: string;
  readonly version: string;
  readonly description: string;
  readonly language: string;
  readonly entryPoint: string;

  constructor(data: any);
}

export class LogStats {
  readonly totalExecutions: number;
  readonly successful: number;
  readonly failed: number;
  readonly blocked: number;
  readonly avgExecutionTimeMs: number;

  constructor(data: any);
}

export interface OlaForgeOptions {
  binaryPath?: string;
  configPath?: string;
  timeout?: number;
}

export interface ExecuteOptions {
  security?: 'L0' | 'L1' | 'L2' | 'L3';
  timeout?: number;
  noSandbox?: boolean;
}

export interface RunSkillOptions {
  inputJson?: string;
  goal?: string;
}

export interface ChatOptions {
  model?: string;
  agent?: boolean;
}

export class OlaForge {
  constructor(options?: OlaForgeOptions);
  
  execute(code: string, language?: string, options?: ExecuteOptions): ExecutionResult;
  runSkill(skillDir: string, options?: RunSkillOptions): ExecutionResult;
  listSkills(): SkillInfo[];
  audit(path: string, format?: 'json' | 'text'): any;
  getLogs(limit?: number): any[];
  getLogStats(): LogStats;
  healthCheck(): boolean;
  version(): string;
  chat(prompt: string, options?: ChatOptions): string;
}

export function execute(code: string, language?: string, options?: ExecuteOptions): ExecutionResult;
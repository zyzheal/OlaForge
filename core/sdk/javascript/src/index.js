/**
 * OlaForge JavaScript SDK
 * AI Agent 安全沙箱执行引擎
 * 
 * 用法:
 *   const { OlaForge } = require('olaforge');
 *   
 *   const client = new OlaForge();
 *   const result = await client.execute("console.log('hello')", "javascript");
 *   console.log(result.output);
 */

const { execSync, spawnSync } = require('child_process');
const path = require('path');
const os = require('os');

class ExecutionResult {
  constructor(data) {
    this._data = data;
  }

  get success() {
    return this._data?.success ?? false;
  }

  get output() {
    return this._data?.output ?? '';
  }

  get error() {
    return this._data?.error ?? null;
  }

  get executionTimeMs() {
    return this._data?.execution_time_ms ?? 0;
  }

  get sandbox() {
    return this._data?.sandbox ?? {};
  }

  get riskLevel() {
    return this.sandbox?.risk_level ?? 'none';
  }

  get passed() {
    return this.sandbox?.passed ?? true;
  }

  get securityIssues() {
    return this._data?.security_issues ?? [];
  }

  toJSON() {
    return this._data;
  }
}

class SkillInfo {
  constructor(data) {
    this._data = data;
  }

  get name() {
    return this._data?.name ?? '';
  }

  get version() {
    return this._data?.version ?? '';
  }

  get description() {
    return this._data?.description ?? '';
  }

  get language() {
    return this._data?.language ?? '';
  }

  get entryPoint() {
    return this._data?.entry_point ?? '';
  }
}

class LogStats {
  constructor(data) {
    this._data = data;
  }

  get totalExecutions() {
    return this._data?.total_executions ?? 0;
  }

  get successful() {
    return this._data?.successful ?? 0;
  }

  get failed() {
    return this._data?.failed ?? 0;
  }

  get blocked() {
    return this._data?.blocked ?? 0;
  }

  get avgExecutionTimeMs() {
    return this._data?.avg_execution_time_ms ?? 0;
  }
}

class OlaForge {
  /**
   * 初始化 OlaForge 客户端
   * @param {Object} options
   * @param {string} options.binaryPath - olaforge 二进制路径
   * @param {string} options.configPath - 配置文件路径
   * @param {number} options.timeout - 默认超时时间 (秒)
   */
  constructor(options = {}) {
    this.timeout = options.timeout || 60;
    this.binaryPath = options.binaryPath || this._findBinary();
    this.configPath = options.configPath;

    if (!this.binaryPath) {
      throw new Error('找不到 olaforge 二进制，请确保已安装');
    }
  }

  _findBinary() {
    const candidates = [
      'olaforge',
      './olaforge',
      path.join(os.homedir(), 'OlaForge/core/target/release/olaforge'),
      '/usr/local/bin/olaforge',
      path.join(os.homedir(), '.cargo/bin/olaforge'),
    ];

    for (const candidate of candidates) {
      try {
        require('fs').accessSync(candidate, require('fs').constants.X_OK);
        return candidate;
      } catch {
        // 继续尝试下一个
      }
    }

    // 尝试 which
    try {
      const result = execSync('which olaforge', { encoding: 'utf8' });
      return result.trim();
    } catch {
      return 'olaforge';
    }
  }

  _run(args) {
    const cmd = [this.binaryPath];
    if (this.configPath) {
      cmd.push('--config', this.configPath);
    }
    cmd.push(...args);

    try {
      const result = execSync(cmd.join(' '), {
        encoding: 'utf8',
        timeout: (this.timeout + 10) * 1000,
        maxBuffer: 10 * 1024 * 1024,
      });

      try {
        return JSON.parse(result);
      } catch {
        return { success: false, error: result, output: '' };
      }
    } catch (error) {
      if (error.stdout) {
        try {
          return JSON.parse(error.stdout);
        } catch {
          return { success: false, error: error.stdout, output: '' };
        }
      }
      return { success: false, error: error.message, output: '' };
    }
  }

  /**
   * 在沙箱中执行代码
   * @param {string} code - 要执行的代码
   * @param {string} language - 语言 (python/javascript/bash/ruby/go/perl)
   * @param {Object} options
   * @param {string} options.security - 安全级别 (L0/L1/L2/L3)
   * @param {number} options.timeout - 超时时间 (秒)
   * @param {boolean} options.noSandbox - 禁用沙箱
   * @returns {ExecutionResult}
   */
  execute(code, language = 'python', options = {}) {
    const { security = 'L2', timeout, noSandbox = false } = options;
    const effectiveTimeout = timeout || this.timeout;

    const args = [
      'execute',
      '--code', code,
      '--language', language,
      '--security', security,
      '--timeout', String(effectiveTimeout),
    ];

    if (noSandbox) {
      args.push('--no-sandbox');
    }

    const result = this._run(args);
    return new ExecutionResult(result);
  }

  /**
   * 运行技能
   * @param {string} skillDir - 技能目录
   * @param {Object} options
   * @param {string} options.inputJson - 输入 JSON
   * @param {string} options.goal - 目标描述
   * @returns {ExecutionResult}
   */
  runSkill(skillDir, options = {}) {
    const { inputJson, goal } = options;
    const args = ['run', skillDir];

    if (inputJson) {
      args.push('--input-json', inputJson);
    }
    if (goal) {
      args.push('--goal', goal);
    }

    const result = this._run(args);
    return new ExecutionResult(result);
  }

  /**
   * 列出可用技能
   * @returns {SkillInfo[]}
   */
  listSkills() {
    const result = this._run(['skills']);
    const skills = result?.skills ?? [];
    return skills.map(s => new SkillInfo(s));
  }

  /**
   * 依赖安全审计
   * @param {string} auditPath - 要审计的目录
   * @param {string} format - 输出格式 (json/text)
   * @returns {Object}
   */
  audit(auditPath, format = 'json') {
    return this._run(['audit', '--path', auditPath, '--format', format]);
  }

  /**
   * 获取执行日志
   * @param {number} limit - 返回数量
   * @returns {Object[]}
   */
  getLogs(limit = 10) {
    return this._run(['logs', '--limit', String(limit)]);
  }

  /**
   * 获取日志统计
   * @returns {LogStats}
   */
  getLogStats() {
    const result = this._run(['logs', '--stats', 'true']);
    return new LogStats(result);
  }

  /**
   * 健康检查
   * @returns {boolean}
   */
  healthCheck() {
    const result = this._run(['health']);
    return result?.status === 'healthy';
  }

  /**
   * 获取版本
   * @returns {string}
   */
  version() {
    const result = this._run(['version']);
    return result?.version ?? 'unknown';
  }

  /**
   * 与 AI 对话
   * @param {string} prompt - 输入提示
   * @param {Object} options
   * @param {string} options.model - 模型名称
   * @param {boolean} options.agent - 是否使用 Agent 模式
   * @returns {string}
   */
  chat(prompt, options = {}) {
    const { model = 'gpt-3.5-turbo', agent = false } = options;
    const args = ['chat', '--prompt', prompt, '--model', model];
    if (agent) {
      args.push('--agent');
    }

    const result = this._run(args);
    return result?.output ?? result?.error ?? '';
  }
}

// 便捷函数
async function execute(code, language = 'python', options = {}) {
  const client = new OlaForge();
  return client.execute(code, language, options);
}

module.exports = { OlaForge, ExecutionResult, SkillInfo, LogStats, execute };
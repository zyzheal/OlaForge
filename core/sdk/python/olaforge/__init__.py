"""
OlaForge Python SDK
AI Agent 安全沙箱执行引擎

用法:
    from olaforge import OlaForge
    
    client = OlaForge()
    result = client.execute("print('hello')", language="python")
    print(result.output)
"""

import json
import subprocess
import os
from typing import Optional, Dict, Any, List

class ExecutionResult:
    """执行结果"""
    def __init__(self, data: Dict[str, Any]):
        self._data = data
    
    @property
    def success(self) -> bool:
        return self._data.get("success", False)
    
    @property
    def output(self) -> str:
        return self._data.get("output", "")
    
    @property
    def error(self) -> Optional[str]:
        return self._data.get("error")
    
    @property
    def execution_time_ms(self) -> int:
        return self._data.get("execution_time_ms", 0)
    
    @property
    def sandbox(self) -> Dict[str, Any]:
        return self._data.get("sandbox", {})
    
    @property
    def risk_level(self) -> str:
        return self.sandbox.get("risk_level", "none")
    
    @property
    def passed(self) -> bool:
        return self.sandbox.get("passed", True)
    
    @property
    def security_issues(self) -> List[str]:
        return self._data.get("security_issues", [])
    
    def __repr__(self):
        return f"<ExecutionResult success={self.success} output={self.output[:50]!r}...>"

class SkillInfo:
    """技能信息"""
    def __init__(self, data: Dict[str, Any]):
        self._data = data
    
    @property
    def name(self) -> str:
        return self._data.get("name", "")
    
    @property
    def version(self) -> str:
        return self._data.get("version", "")
    
    @property
    def description(self) -> str:
        return self._data.get("description", "")
    
    @property
    def language(self) -> str:
        return self._data.get("language", "")
    
    @property
    def entry_point(self) -> str:
        return self._data.get("entry_point", "")

class LogStats:
    """日志统计"""
    def __init__(self, data: Dict[str, Any]):
        self._data = data
    
    @property
    def total_executions(self) -> int:
        return self._data.get("total_executions", 0)
    
    @property
    def successful(self) -> int:
        return self._data.get("successful", 0)
    
    @property
    def failed(self) -> int:
        return self._data.get("failed", 0)
    
    @property
    def blocked(self) -> int:
        return self._data.get("blocked", 0)
    
    @property
    def avg_execution_time_ms(self) -> int:
        return self._data.get("avg_execution_time_ms", 0)

class OlaForge:
    """OlaForge 客户端"""
    
    def __init__(
        self,
        binary_path: Optional[str] = None,
        config_path: Optional[str] = None,
        timeout: int = 60
    ):
        """
        初始化 OlaForge 客户端
        
        Args:
            binary_path: olaforge 二进制路径 (默认自动查找)
            config_path: 配置文件路径
            timeout: 默认超时时间 (秒)
        """
        self.timeout = timeout
        self.binary_path = binary_path or self._find_binary()
        self.config_path = config_path
        
        if not self.binary_path:
            raise RuntimeError("找不到 olaforge 二进制，请确保已安装")
    
    def _find_binary(self) -> str:
        """查找 olaforge 二进制"""
        # 检查 PATH
        path = os.environ.get("PATH", "").split(os.pathsep)
        
        # 常见位置
        candidates = [
            "olaforge",
            "./olaforge",
            os.path.expanduser("~/OlaForge/core/target/release/olaforge"),
            "/usr/local/bin/olaforge",
            os.path.expanduser("~/.cargo/bin/olaforge"),
        ]
        
        for p in path:
            candidates.insert(0, os.path.join(p, "olaforge"))
        
        for candidate in candidates:
            if os.path.isfile(candidate) and os.access(candidate, os.X_OK):
                return candidate
        
        # 尝试 which
        try:
            result = subprocess.run(
                ["which", "olaforge"],
                capture_output=True,
                text=True
            )
            if result.returncode == 0:
                return result.stdout.strip()
        except:
            pass
        
        return "olaforge"  # 最后尝试
    
    def _run(self, args: List[str]) -> Dict[str, Any]:
        """运行命令"""
        cmd = [self.binary_path]
        if self.config_path:
            cmd.extend(["--config", self.config_path])
        cmd.extend(args)
        
        try:
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=self.timeout + 10
            )
            
            # 尝试解析 JSON
            try:
                return json.loads(result.stdout)
            except json.JSONDecodeError:
                return {
                    "success": False,
                    "error": result.stdout or result.stderr,
                    "output": ""
                }
        except subprocess.TimeoutExpired:
            return {
                "success": False,
                "error": "执行超时",
                "output": ""
            }
        except Exception as e:
            return {
                "success": False,
                "error": str(e),
                "output": ""
            }
    
    def execute(
        self,
        code: str,
        language: str = "python",
        security: str = "L2",
        timeout: Optional[int] = None,
        no_sandbox: bool = False
    ) -> ExecutionResult:
        """
        在沙箱中执行代码
        
        Args:
            code: 要执行的代码
            language: 语言 (python/javascript/bash/ruby/go/perl)
            security: 安全级别 (L0/L1/L2/L3)
            timeout: 超时时间 (秒)
            no_sandbox: 禁用沙箱
            
        Returns:
            ExecutionResult: 执行结果
        """
        timeout = timeout or self.timeout
        
        args = [
            "execute",
            "--code", code,
            "--language", language,
            "--security", security,
            "--timeout", str(timeout)
        ]
        
        if no_sandbox:
            args.append("--no-sandbox")
        
        result = self._run(args)
        return ExecutionResult(result)
    
    def run_skill(
        self,
        skill_dir: str,
        input_json: Optional[str] = None,
        goal: Optional[str] = None
    ) -> ExecutionResult:
        """
        运行技能
        
        Args:
            skill_dir: 技能目录
            input_json: 输入 JSON
            goal: 目标描述
            
        Returns:
            ExecutionResult: 执行结果
        """
        args = ["run", skill_dir]
        
        if input_json:
            args.extend(["--input-json", input_json])
        if goal:
            args.extend(["--goal", goal])
        
        result = self._run(args)
        return ExecutionResult(result)
    
    def list_skills(self) -> List[SkillInfo]:
        """列出可用技能"""
        result = self._run(["skills"])
        skills = result.get("skills", [])
        return [SkillInfo(s) for s in skills]
    
    def audit(self, path: str, format: str = "json") -> Dict[str, Any]:
        """
        依赖安全审计
        
        Args:
            path: 要审计的目录
            format: 输出格式 (json/text)
            
        Returns:
            审计结果
        """
        return self._run(["audit", "--path", path, "--format", format])
    
    def get_logs(self, limit: int = 10) -> List[Dict[str, Any]]:
        """
        获取执行日志
        
        Args:
            limit: 返回数量
            
        Returns:
            日志列表
        """
        return self._run(["logs", "--limit", str(limit)])
    
    def get_log_stats(self) -> LogStats:
        """获取日志统计"""
        result = self._run(["logs", "--stats", "true"])
        return LogStats(result)
    
    def health_check(self) -> bool:
        """健康检查"""
        result = self._run(["health"])
        return result.get("status") == "healthy"
    
    def version(self) -> str:
        """获取版本"""
        result = self._run(["version"])
        return result.get("version", "unknown")
    
    def chat(
        self,
        prompt: str,
        model: str = "gpt-3.5-turbo",
        agent: bool = False
    ) -> str:
        """
        与 AI 对话
        
        Args:
            prompt: 输入提示
            model: 模型名称
            agent: 是否使用 Agent 模式
            
        Returns:
            AI 回复
        """
        args = ["chat", "--prompt", prompt, "--model", model]
        if agent:
            args.append("--agent")
        
        result = self._run(args)
        return result.get("output", result.get("error", ""))

# 便捷函数
def execute(code: str, language: str = "python", **kwargs) -> ExecutionResult:
    """快速执行代码"""
    client = OlaForge()
    return client.execute(code, language, **kwargs)

if __name__ == "__main__":
    # 测试
    client = OlaForge()
    
    # 健康检查
    print(f"健康状态: {client.health_check()}")
    
    # 执行代码
    result = client.execute("print('Hello from OlaForge Python SDK!')")
    print(f"执行成功: {result.success}")
    print(f"输出: {result.output}")
    
    # 安全执行
    result = client.execute("eval('1+1')", security="L2")
    print(f"危险代码被阻止: {not result.passed}")
    print(f"风险级别: {result.risk_level}")
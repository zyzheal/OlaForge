# OlaForge Python SDK 详细文档

## 安装

```bash
pip install olaforge
```

## 快速开始

```python
from olaforge import OlaForge

# 初始化客户端
client = OlaForge(
    api_key="sk-your-key",
    model="claude-3-5-sonnet"
)

# 简单对话
response = client.chat("你好")
print(response.text)
```

## 类与方法

### OlaForge 客户端

```python
class OlaForge:
    """OlaForge SDK 主客户端"""
    
    def __init__(
        self,
        api_key: str = None,
        model: str = "claude-3-5-sonnet",
        base_url: str = "http://localhost:7860/api/v1",
        security: str = "L2",
        timeout: int = 60,
        max_retries: int = 3
    ):
        """初始化客户端
        
        Args:
            api_key: API Key
            model: 使用的模型
            base_url: API 基础URL
            security: 安全等级 L0-L3
            timeout: 请求超时(秒)
            max_retries: 最大重试次数
        """
        pass
    
    # ==================== 对话方法 ====================
    
    def chat(
        self,
        message: str,
        model: str = None,
        system: str = None,
        stream: bool = False
    ) -> "ChatResponse":
        """发送对话请求
        
        Args:
            message: 用户消息
            model: 覆盖默认模型
            system: 系统提示词
            stream: 是否流式响应
            
        Returns:
            ChatResponse: 对话响应对象
        """
        pass
    
    def stream_chat(self, message: str) -> Generator[str, None, None]:
        """流式对话
        
        Args:
            message: 用户消息
            
        Yields:
            str: 响应文本块
        """
        pass
    
    # ==================== 执行方法 ====================
    
    def run_code(
        self,
        code: str,
        language: str = "python",
        timeout: int = 60,
        memory_limit: int = 1024,
        env: dict = None
    ) -> "ExecuteResponse":
        """执行代码
        
        Args:
            code: 要执行的代码
            language: 编程语言
            timeout: 超时时间(秒)
            memory_limit: 内存限制(MB)
            env: 环境变量
            
        Returns:
            ExecuteResponse: 执行响应
        """
        pass
    
    def run_bash(self, command: str, timeout: int = 60) -> "ExecuteResponse":
        """执行 Bash 命令
        
        Args:
            command: Bash 命令
            timeout: 超时时间(秒)
            
        Returns:
            ExecuteResponse: 执行响应
        """
        return self.run_code(command, language="bash", timeout=timeout)
    
    # ==================== Skill 方法 ====================
    
    def list_skills(self) -> List["Skill"]:
        """获取 Skill 列表
        
        Returns:
            List[Skill]: Skill 列表
        """
        pass
    
    def get_skill(self, name: str) -> "SkillDetail":
        """获取 Skill 详情
        
        Args:
            name: Skill 名称
            
        Returns:
            SkillDetail: Skill 详情
        """
        pass
    
    def add_skill(self, source: str) -> "Skill":
        """添加 Skill
        
        Args:
            source: Skill 来源 (GitHub repo 或本地路径)
            
        Returns:
            Skill: 添加的 Skill
        """
        pass
    
    def remove_skill(self, name: str) -> None:
        """删除 Skill
        
        Args:
            name: Skill 名称
        """
        pass
    
    def run_skill(
        self,
        name: str,
        input: dict = None,
        **kwargs
    ) -> "SkillResponse":
        """执行 Skill
        
        Args:
            name: Skill 名称
            input: 输入参数
            **kwargs: 其他参数
            
        Returns:
            SkillResponse: 执行结果
        """
        pass
    
    # ==================== 系统方法 ====================
    
    def get_status(self) -> "SystemStatus":
        """获取系统状态
        
        Returns:
            SystemStatus: 系统状态
        """
        pass
    
    def get_config(self) -> dict:
        """获取配置
        
        Returns:
            dict: 当前配置
        """
        pass
    
    def update_config(self, config: dict) -> dict:
        """更新配置
        
        Args:
            config: 新配置
            
        Returns:
            dict: 更新后的配置
        """
        pass
    
    def health_check(self) -> bool:
        """健康检查
        
        Returns:
            bool: 是否健康
        """
        pass


# ==================== 响应对象 ====================

class ChatResponse:
    """对话响应"""
    
    def __init__(self, data: dict):
        self.id = data.get("id")
        self.text = data.get("text")
        self.usage = data.get("usage", {})
        self.metadata = data.get("metadata", {})
    
    @property
    def input_tokens(self) -> int:
        return self.usage.get("input_tokens", 0)
    
    @property
    def output_tokens(self) -> int:
        return self.usage.get("output_tokens", 0)
    
    @property
    def total_tokens(self) -> int:
        return self.usage.get("total_tokens", 0)


class ExecuteResponse:
    """代码执行响应"""
    
    def __init__(self, data: dict):
        self.id = data.get("id")
        self.output = data.get("output", "")
        self.error = data.get("error")
        self.exit_code = data.get("exit_code", -1)
        self.execution_time_ms = data.get("execution_time_ms", 0)
        self.memory_used_mb = data.get("memory_used_mb", 0)
    
    @property
    def success(self) -> bool:
        return self.exit_code == 0 and self.error is None
    
    def __str__(self) -> str:
        if self.error:
            return f"Error: {self.error}"
        return self.output


class Skill:
    """Skill 信息"""
    
    def __init__(self, data: dict):
        self.name = data.get("name")
        self.version = data.get("version")
        self.description = data.get("description", "")
        self.author = data.get("author", "")
        self.enabled = data.get("enabled", True)


class SkillDetail(Skill):
    """Skill 详情"""
    
    def __init__(self, data: dict):
        super().__init__(data)
        self.dependencies = data.get("dependencies", [])
        self.trust_level = data.get("trust_level", "low")
        self.input_schema = data.get("input_schema", {})
        self.output_schema = data.get("output_schema", {})


class SkillResponse:
    """Skill 执行响应"""
    
    def __init__(self, data: dict):
        self.result = data.get("result", {})
        self.metadata = data.get("metadata", {})
    
    def __getitem__(self, key):
        return self.result.get(key)


class SystemStatus:
    """系统状态"""
    
    def __init__(self, data: dict):
        self.version = data.get("version")
        self.uptime_seconds = data.get("uptime_seconds", 0)
        self.active_sandboxes = data.get("active_sandboxes", 0)
        self.total_executions = data.get("total_executions", 0)
        self.cpu_usage_percent = data.get("cpu_usage_percent", 0)
        self.memory_usage_mb = data.get("memory_usage_mb", 0)
    
    def __str__(self) -> str:
        return (
            f"OlaForge v{self.version}\n"
            f"Uptime: {self.uptime_seconds}s\n"
            f"Active Sandboxes: {self.active_sandboxes}\n"
            f"Total Executions: {self.total_executions}\n"
            f"CPU: {self.cpu_usage_percent}%\n"
            f"Memory: {self.memory_usage_mb}MB"
        )


# ==================== 异常类 ====================

class OlaForgeError(Exception):
    """基础异常"""
    pass


class RateLimitError(OlaForgeError):
    """限流异常"""
    def __init__(self, message: str, retry_after: int = None):
        super().__init__(message)
        self.retry_after = retry_after


class SecurityError(OlaForgeError):
    """安全异常"""
    pass


class ExecutionError(OlaForgeError):
    """执行异常"""
    pass


# ==================== 便捷函数 ====================

def chat(message: str, **kwargs) -> ChatResponse:
    """快速对话 (单行使用)"""
    return OlaForge().chat(message, **kwargs)


def run(code: str, language: str = "python", **kwargs) -> ExecuteResponse:
    """快速执行代码 (单行使用)"""
    return OlaForge().run_code(code, language, **kwargs)


# ==================== 使用示例 ====================

# 示例 1: 基本对话
client = OlaForge(api_key="sk-xxx")
response = client.chat("什么是 OlaForge?")
print(response.text)

# 示例 2: 执行代码
result = client.run_code("print('hello')", language="python")
print(result.output)

# 示例 3: 使用 Skill
skills = client.list_skills()
print(skills[0].name)

# 示例 4: 流式对话
for chunk in client.stream_chat("写一首诗"):
    print(chunk, end="", flush=True)

# 示例 5: 检查状态
status = client.get_status()
print(status)

# 示例 6: 错误处理
try:
    result = client.run_code("import os; os.system('rm -rf /')")
except SecurityError as e:
    print(f"安全拦截: {e}")

# 示例 7: 自定义配置
client = OlaForge(
    api_key="sk-xxx",
    model="gpt-4",
    security="L3",
    timeout=120
)
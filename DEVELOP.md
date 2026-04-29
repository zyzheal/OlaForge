# OlaForge 开发指南

## 快速开始

```bash
# 1. 克隆项目
git clone https://github.com/zyzheal/OlaForge.git
cd OlaForge

# 2. 安装依赖
npm install

# 3. 构建
npm run build

# 4. 运行
node dist/cli.js --help
```

## 开发命令

| 命令 | 说明 |
|------|------|
| `npm run build` | 构建项目 |
| `npm run dev` | 开发模式 (热重载) |
| `npm test` | 运行测试 |
| `npm run typecheck` | 类型检查 |

## 项目结构

```
src/
├── cli/                 # CLI 入口
│   ├── index.ts        # 主入口
│   └── commands/       # 命令实现
│       ├── chat.ts
│       ├── run.ts
│       ├── config.ts
│       └── ...
├── sandbox/            # 沙箱执行
│   └── executor.ts    # 代码执行器
├── config/             # 配置管理
│   ├── manager.ts    # 配置管理器
│   └── project.ts   # 项目初始化
├── evolution/          # 智能进化
│   └── skill.ts     # Skill 管理
├── api/               # API 服务
│   └── status.ts   # 系统状态
└── utils/             # 工具函数

tests/
├── unit/              # 单元测试
└── integration/       # 集成测试
```

## 开发任务清单

### Phase 1: MVP (第1-2周)

- [x] 项目结构搭建
- [ ] CLI 框架完善
- [ ] 基础命令实现
- [ ] 配置文件加载
- [ ] 本地执行器 (L0)

### Phase 2: 安全 (第3周)

- [ ] L1 隔离 (namespace)
- [ ] L2 隔离 (bwrap)
- [ ] 安全扫描
- [ ] 危险命令拦截

### Phase 3: 智能化 (第4周)

- [ ] 进化引擎框架
- [ ] 数据收集
- [ ] 质量 Gate

## 如何贡献

1. Fork 项目
2. 创建分支: `git checkout -b feature/xxx`
3. 开发提交: `git commit -m 'feat: 添加 xxx'`
4. 推送分支: `git push origin feature/xxx`
5. 提交 PR

## 编码规范

- 使用 TypeScript
- 使用 ESM 模块
- 遵循 ESLint 规则
- 编写单元测试

## 技术栈

- **语言**: TypeScript, JavaScript
- **运行时**: Node.js 18+
- **构建**: esbuild
- **测试**: Vitest
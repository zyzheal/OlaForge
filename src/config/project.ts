// 项目初始化

import { writeFileSync, mkdirSync } from 'fs'
import { join } from 'path'

export async function initProject(dir: string, force: boolean): Promise<void> {
  const projectDir = join(dir, '.olaforge')
  
  // 创建项目目录
  mkdirSync(projectDir, { recursive: true })
  
  // 创建配置文件
  writeFileSync(join(projectDir, 'config.yaml'), `version: "1.0"
model:
  provider: "openai"
  name: "claude-3-5-sonnet"
security:
  level: "L2"
  auto_level: true
`)
  
  console.log('项目目录:', projectDir)
}

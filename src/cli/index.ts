#!/usr/bin/env node

import { Command } from 'commander'
import chalk from 'chalk'
import { chatCommand } from './commands/chat.js'
import { runCommand } from './commands/run.js'
import { configCommand } from './commands/config.js'
import { initCommand } from './commands/init.js'
import { skillCommand } from './commands/skill.js'
import { statusCommand } from './commands/status.js'

const program = new Command()

program
  .name('olaforge')
  .description('OlaForge - 智能安全沙箱系统')
  .version('1.0.0')

// 注册命令
program.addCommand(chatCommand)
program.addCommand(runCommand)
program.addCommand(configCommand)
program.addCommand(initCommand)
program.addCommand(skillCommand)
program.addCommand(statusCommand)

// 全局选项
program.option('-v, --verbose', '详细输出')

// 解析参数
program.parse()

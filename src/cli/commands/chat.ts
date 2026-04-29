import { Command } from 'commander'
import chalk from 'chalk'

export const chatCommand = new Command('chat')
  .description('启动交互式对话')
  .option('-m, --model <name>', '指定模型', 'claude-3-5-sonnet')
  .option('-s, --security <level>', '安全等级 L0-L3', 'L2')
  .option('-v, --verbose', '详细输出')
  .action(async (options) => {
    console.log(chalk.blue('OlaForge 对话模式'))
    console.log(chalk.gray(`模型: ${options.model}`))
    console.log(chalk.gray(`安全: ${options.security}`))
    console.log()
    console.log(chalk.yellow('提示: 输入 :exit 退出对话'))
    console.log()
    
    // TODO: 实现对话逻辑
    console.log(chalk.green('✓ 交互式对话功能开发中...'))
  })

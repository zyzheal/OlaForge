import { Command } from 'commander'
import chalk from 'chalk'
import { getStatus } from '../../api/status.js'

export const statusCommand = new Command('status')
  .description('查看系统状态')
  .action(async () => {
    const status = await getStatus()
    
    console.log(chalk.blue('OlaForge 系统状态'))
    console.log(chalk.gray('─'.repeat(30)))
    console.log(`版本: ${status.version}`)
    console.log(`运行时间: ${status.uptime}`)
    console.log(`活跃沙箱: ${status.activeSandboxes}`)
    console.log(`总执行次数: ${status.totalExecutions}`)
  })

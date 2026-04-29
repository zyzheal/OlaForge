import { Command } from 'commander'
import chalk from 'chalk'
import { initProject } from '../../config/project.js'

export const initCommand = new Command('init')
  .description('初始化项目')
  .option('-f, --force', '强制初始化')
  .option('-d, --dir <path>', '目标目录', '.')
  .action(async (options) => {
    console.log(chalk.blue('初始化 OlaForge 项目...'))
    
    await initProject(options.dir, options.force)
    
    console.log(chalk.green('✓ 项目初始化完成'))
    console.log(chalk.gray('\n下一步:'))
    console.log('  olaforge config set api_key <your-key>')
    console.log('  olaforge chat')
  })

import { Command } from 'commander'
import chalk from 'chalk'
import { executeCode } from '../../sandbox/executor.js'

export const runCommand = new Command('run')
  .description('执行代码')
  .argument('<code>', '要执行的代码')
  .option('-l, --language <lang>', '语言', 'python')
  .option('-s, --security <level>', '安全等级', 'L2')
  .option('-t, --timeout <seconds>', '超时时间', '60')
  .action(async (code: string, options) => {
    console.log(chalk.blue('OlaForge 代码执行'))
    console.log(chalk.gray(`语言: ${options.language}`))
    console.log(chalk.gray(`安全: ${options.security}`))
    
    try {
      const result = await executeCode(code, {
        language: options.language,
        security: options.security,
        timeout: parseInt(options.timeout)
      })
      
      if (result.success) {
        console.log(chalk.green('\n输出:'))
        console.log(result.output)
      } else {
        console.log(chalk.red('\n错误:'))
        console.log(result.error)
      }
    } catch (error) {
      console.error(chalk.red('执行失败:'), error)
    }
  })

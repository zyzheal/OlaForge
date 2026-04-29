import { Command } from 'commander'
import { getConfig, setConfig, listConfig } from '../../config/manager.js'
import chalk from 'chalk'

export const configCommand = new Command('config')
  .description('配置管理')
  .addCommand(new Command('get')
    .description('获取配置')
    .argument('[key]', '配置键')
    .action(async (key) => {
      if (key) {
        console.log(chalk.gray(`${key}:`), getConfig(key))
      } else {
        console.log(chalk.blue('当前配置:'))
        Object.entries(listConfig()).forEach(([k, v]) => {
          console.log(chalk.gray(`  ${k}:`), v)
        })
      }
    }))
  .addCommand(new Command('set')
    .description('设置配置')
    .argument('<key>')
    .argument('<value>')
    .action(async (key, value) => {
      setConfig(key, value)
      console.log(chalk.green(`✓ 已设置 ${key} = ${value}`))
    }))

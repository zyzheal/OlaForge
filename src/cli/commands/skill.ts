import { Command } from 'commander'
import chalk from 'chalk'
import { listSkills, addSkill, removeSkill } from '../../evolution/skill.js'

export const skillCommand = new Command('skill')
  .description('Skill 管理')
  .addCommand(new Command('list').description('列出 Skills').action(() => {
    const skills = listSkills()
    if (skills.length === 0) {
      console.log(chalk.gray('暂无安装的 Skills'))
    } else {
      console.log(chalk.blue('已安装的 Skills:'))
      skills.forEach(s => console.log(`  - ${s.name} (${s.version})`))
    }
  }))
  .addCommand(new Command('add').description('添加 Skill').argument('<source>').action(async (source) => {
    console.log(chalk.blue(`添加 Skill: ${source}...`))
    await addSkill(source)
    console.log(chalk.green('✓ Skill 添加成功'))
  }))
  .addCommand(new Command('remove').description('移除 Skill').argument('<name>').action(async (name) => {
    await removeSkill(name)
    console.log(chalk.green('✓ Skill 已移除'))
  }))

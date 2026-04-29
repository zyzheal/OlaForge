// Skill 管理

export interface Skill {
  name: string
  version: string
  description: string
}

const skills: Skill[] = []

export function listSkills(): Skill[] {
  return [...skills]
}

export async function addSkill(source: string): Promise<void> {
  console.log(`添加 Skill from: ${source}`)
  // TODO: 实现 Skill 添加逻辑
}

export async function removeSkill(name: string): Promise<void> {
  const idx = skills.findIndex(s => s.name === name)
  if (idx >= 0) skills.splice(idx, 1)
}

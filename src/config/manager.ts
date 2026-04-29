// 配置管理器

const configStore = new Map<string, string>()

export function getConfig(key: string): string | undefined {
  return configStore.get(key)
}

export function setConfig(key: string, value: string): void {
  configStore.set(key, value)
}

export function listConfig(): Record<string, string> {
  return Object.fromEntries(configStore)
}

export function loadConfig(): void {
  // TODO: 从 ~/.olaforge/config.yaml 加载
}

export function saveConfig(): void {
  // TODO: 保存到 ~/.olaforge/config.yaml
}

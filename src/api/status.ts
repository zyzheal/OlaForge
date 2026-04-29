// 系统状态

export interface SystemStatus {
  version: string
  uptime: string
  activeSandboxes: number
  totalExecutions: number
}

export async function getStatus(): Promise<SystemStatus> {
  return {
    version: '1.0.0',
    uptime: '1h 23m',
    activeSandboxes: 0,
    totalExecutions: 10
  }
}

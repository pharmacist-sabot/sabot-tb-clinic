import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface DbConfig {
  host: string
  port: number
  database: string
  username: string
  password: string
}

export const useSettingsStore = defineStore('settings', () => {
  const dbConfig = ref<DbConfig>({
    host: 'localhost',
    port: 3306,
    database: 'hosxp',
    username: 'root',
    password: '',
  })
  const isConnected = ref(false)
  const isConnecting = ref(false)
  const connectionError = ref<string | null>(null)

  const staffNames = ref<string[]>(['พยาบาลวิชาชีพ', 'เภสัชกร', 'แพทย์'])

  const drugCodes = ref({
    H: ['1430104'],
    R: ['1000265', '1000264'],
    E: ['1600004', '1000129'],
    Z: ['1000258'],
  })

  const regimens = ref<string[]>(['2HRZE/4HR', '2HRZE/6HR'])

  async function testConnection(config: DbConfig): Promise<boolean> {
    try {
      isConnecting.value = true
      connectionError.value = null
      const result = await invoke<boolean>('test_mysql_connection', { config })
      return result
    } catch (e) {
      connectionError.value = String(e)
      return false
    } finally {
      isConnecting.value = false
    }
  }

  async function connect(config: DbConfig): Promise<void> {
    try {
      isConnecting.value = true
      connectionError.value = null
      await invoke('connect_mysql', { config })
      dbConfig.value = config
      isConnected.value = true
    } catch (e) {
      connectionError.value = String(e)
      isConnected.value = false
    } finally {
      isConnecting.value = false
    }
  }

  async function checkConnection(): Promise<void> {
    try {
      const status = await invoke<boolean>('get_mysql_status')
      isConnected.value = status
    } catch {
      isConnected.value = false
    }
  }

  return {
    dbConfig,
    isConnected,
    isConnecting,
    connectionError,
    staffNames,
    drugCodes,
    regimens,
    testConnection,
    connect,
    checkConnection,
  }
})
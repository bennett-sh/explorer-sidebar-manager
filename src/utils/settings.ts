
import { Store } from 'tauri-plugin-store-api'

export const settingsStore = new Store('.settings.dat')

export type SettingsKey = 'shortcuts'

export async function setSetting<T>(key: SettingsKey, value: T, save = false) {
  await settingsStore.set(key, value)
  if(save) await settingsStore.save()
}

export async function getSetting<T>(key: SettingsKey, fallback?: T): Promise<T> {
  return await settingsStore.get(key) ?? fallback
}

export async function saveSettings(): Promise<void> {
  await settingsStore.save()
}

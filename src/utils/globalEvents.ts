import { writable } from 'svelte/store'

export const globalEventState = writable<{type: GlobalEvent, data?: any} | null>(null)

export function emitEvent(event: { type: GlobalEvent, data?: any }) {
  globalEventState.set(event)
}

export enum GlobalEvent {
  DataChange
}

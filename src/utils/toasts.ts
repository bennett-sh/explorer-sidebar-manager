import { writable, type Writable } from 'svelte/store'
import type { ToastData, ToastRef } from '../types'
import { v4 as uuidv4 } from 'uuid'

export const toastStore: Writable<ToastData[]> = writable([])

export function showToast(toast: ToastData): ToastRef {
  let id: string = toast.id ?? uuidv4()
  toastStore.update(
    data => [
      ...data,
      { ...toast, id }
    ]
  )

  return {
    id
  }
}

export function closeToast(toastRef: ToastRef) {
  toastStore.update(
    data => data.map(
      toast => toastRef.id === toast.id ? (
        {
          ...toast,
          open: false
        }
      ) : toast
    )
  )
}
export function openToast(toastRef: ToastRef) {
  toastStore.update(
    data => data.map(
      toast => toastRef.id === toast.id ? (
        {
          ...toast,
          open: true
        }
      ) : toast
    )
  )
}

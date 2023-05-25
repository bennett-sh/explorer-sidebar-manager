
export interface Shortcut {
  name: string
  iconPath: string
  iconIndex: number
  path: string
  guid: string
}

export interface ToastData {
  type: 'Okay' | 'Error' | 'Info' | null
  message: string
  open: boolean
  id?: string
}
export interface ToastRef {
  id: string
}

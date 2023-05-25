
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
  onClose?: (ref: ToastRef) => boolean | void
  open?: boolean
  id?: string
}
export interface ToastRef {
  id: string
}

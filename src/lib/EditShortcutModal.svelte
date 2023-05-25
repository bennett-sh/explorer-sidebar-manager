<script lang="ts">
  import { Button, ButtonGroup, Input, Label, Modal } from 'flowbite-svelte'
  import { open as openFilePicker } from '@tauri-apps/api/dialog'
  import { GlobalEvent, emitEvent } from '../utils/globalEvents'
  import { showQuickToast } from '../utils/toasts'
  import { createEventDispatcher } from 'svelte'
  import { invoke } from '@tauri-apps/api/tauri'
  import { FolderOpen } from 'svelte-heros-v2'

  const dispatch = createEventDispatcher()

  export let open = false

  export let guid
  export let name
  export let path
  export let iconPath = ''
  export let iconIndex = 0

  console.log(path)

  async function updateShortcut(data: any) {
    (invoke('add_shortcut', data) as Promise<[boolean, string]>)
      .then(([success, msg]) => {
        if(!success) {
          showQuickToast({ type: 'Error', message: `Failed to edit shortcut: ${msg}` })
          return
        }
        open = false
        let message = 'Edited shortcut successfully'
        if(msg) message += `: ${msg}`
        showQuickToast({ type: 'Okay', message })
        emitEvent({ type: GlobalEvent.DataChange })
      })
      .catch(err => showQuickToast({ type: 'Error', message: `Failed to edit shortcut: ${err}` }))
  }
</script>

<Modal
  bind:open={open}
  on:close={e => dispatch('close', e)}
  {...$$restProps}>
  <form
    class="flex flex-col space-y-6"
    action="#"
    on:submit={async event => {
      event.preventDefault()
      const data = { name, path, iconPath, iconIndex: iconIndex.toString(), guid }
      dispatch('submit', data)
      await updateShortcut(data)
    }}>
    <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Edit Shortcut</h3>
    <Label class="space-y-2">
      <span>Name</span>
      <Input
        type="text"
        name="name"
        autocomplete="off"
        placeholder="My Shortcut"
        required
        bind:value={name} />
    </Label>
    <Label class="space-y-2">
      <span>Path</span>
      <div class="w-full flex flex-row justify-center items-center gap-3">
        <Input
          type="text"
          name="path"
          autocomplete="off"
          placeholder="C:\my\folder"
          bind:value={path}
          required />
        <Button outline class="alternative" color="alternative" on:click={async () => {
          const folder = await openFilePicker({ directory: true, multiple: false, title: 'Choose Path' })

          if(!folder || folder instanceof Array) return

          path = folder
        }}>
          <FolderOpen class="focus:outline-none" size="1.5rem" />
        </Button>
      </div>
    </Label>
    <Label class="space-y-2">
      <span>Icon</span>
      <Input
        type="text"
        name="iconPath"
        autocomplete="off"
        placeholder="C:\Windows\System32\imageres.dll"
        bind:value={iconPath} />
      <Input
        type="number"
        name="iconIndex"
        placeholder="0"
        bind:value={iconIndex}
        disabled={iconPath?.length < 1} />
    </Label>

    <div class="flex flex-row justify-end">
      <ButtonGroup>
        <Button on:click={() => {
          open = false
          dispatch('close')
        }}>Cancel</Button>
        <Button color="primary" type="submit">Edit</Button>
      </ButtonGroup>
    </div>
  </form>
</Modal>

<script lang="ts">
  import { Heading, Accordion, AccordionItem, Button, Tooltip, Toast } from 'flowbite-svelte'
  import { Plus, Pencil, Trash, WrenchScrewdriver } from 'svelte-heros-v2'
  import { GlobalEvent, globalEventState } from '../utils/globalEvents'
  import EditShortcutModal from '../lib/EditShortcutModal.svelte'
  import AddShortcutModal from '../lib/AddShortcutModal.svelte'
  import { getSetting, setSetting } from '../utils/settings'
  import SingleLineCode from '../lib/SingleLineCode.svelte'
  import { showQuickToast } from '../utils/toasts'
  import { invoke } from '@tauri-apps/api/tauri'
  import { navigate } from 'svelte-routing'
	import type { Shortcut } from '../types'
  import { onMount } from 'svelte'

  let editShortcutModalOpen = false
  let editShortcutModalData: Shortcut | null = null

  let addShortcutModalOpen = false
  let shortcutIDs: string[] | null = null
  let shortcuts: Shortcut[] | null = null
  let invalidOrMissingShortcutIDs: Set<string> = new Set()

  onMount(async () => {
    await loadShortcuts()
  })

  globalEventState.subscribe(async event => {
    if(event?.type === GlobalEvent.DataChange) await loadShortcuts()
  })

  async function loadShortcuts() {
    shortcutIDs = await getSetting('shortcuts', [])
    invalidOrMissingShortcutIDs = new Set<string>()
    shortcuts = []
    for(const id of shortcutIDs) {
      try {
        let [success, message, shortcut] = await invoke('get_shortcut_by_id', { id }) as [boolean, string, Shortcut]
        if(!success) {
          if(message === 'key doesn\'t exist anymore') {
            invalidOrMissingShortcutIDs.add(id)
            continue
          }
          showQuickToast({ type: 'Error', message: `Failed to load shortcut ${id}: ${message}` })
          continue
        }
        shortcuts.push(shortcut)
      } catch(err) {
        showQuickToast({ type: 'Error', message: `Failed to load shortcut ${id}: ${err}` })
      }
    }
    // reassign to update UI
    shortcuts = shortcuts
    console.log({ shortcuts, shortcutIDs })

    for(const id of invalidOrMissingShortcutIDs) {
      console.log(`cleaning ${id}`)
      await invoke('cleanup_shortcut', { id })
    }
    await setSetting('shortcuts', (await getSetting('shortcuts', [])).filter(id => !invalidOrMissingShortcutIDs.has(id)))
  }
</script>

<main class="flex flex-col pt-12 px-8 h-full">
  <header class="flex flex-row justify-between items-center">
    <Heading>Sidebar Shortcuts</Heading>
    <div class="flex flex-row justify-end items-center gap-2">
      <Button
        outline
        color="alternative"
        class="alternative !p-2 !h-9"
        on:click={() => navigate('/settings')}>
        <WrenchScrewdriver class="outline-none" size="1.25rem" />
      </Button>
      <Tooltip placement="bottom">Settings</Tooltip>
      <Button
        outline
        class="!p-2 !h-9"
        on:click={() => {
          addShortcutModalOpen = true
        }}>
        <Plus class="outline-none" size="1.25rem" />
      </Button>
      <Tooltip placement="bottom">Add new shortcut</Tooltip>
    </div>
  </header>
  <br/>

  {#if shortcuts && shortcuts?.length > 0}
    <Accordion>
      {#each shortcuts as { name, path, iconPath, iconIndex, guid: id, ...other }}
        <AccordionItem transitionType='blur'>
          <div slot="header" class="w-full mr-4 flex justify-between items-center">
            <span>{name}</span>
            <div class="flex flex-row gap-2 justify-center items-center">
              <Button class="alternative !p-3 !h-10" color="alternative" outline on:click={() => {
                editShortcutModalOpen = true
                editShortcutModalData = { name, path, iconPath, iconIndex, guid: id, ...other }
              }}>
                <Pencil class="outline-none" size="1rem" />
              </Button>
              <Tooltip placement="bottom">Edit this shortcut</Tooltip>
              <Button color="red" outline class="!p-3 !h-10 text-red-500 hover:text-white" on:click={event => {
                invoke('cleanup_shortcut', { id })
                  .then(() => {
                    showQuickToast({ type: 'Okay', message: `Deleted ${name}.` })

                    loadShortcuts()
                      .then(() => {})
                      .catch(err => showQuickToast({ type: 'Error', message: `Failed to reload shortcuts: ${err}` }))
                  })
                  .catch(err => showQuickToast({ type: 'Error', message: `Failed to delete ${name}: ${err}` }))
              }}>
                <Trash class="outline-none" size="1rem" />
              </Button>
              <Tooltip color="red" placement="bottom">Delete this shortcut</Tooltip>
            </div>
          </div>
          <b>Class ID</b> <SingleLineCode>{id}</SingleLineCode>
          <b>Path</b> <SingleLineCode>{path}</SingleLineCode>
          {#if iconPath?.length > 0}
            <b>Icon</b> <SingleLineCode>{iconPath},{iconIndex}</SingleLineCode>
          {/if}
        </AccordionItem>
      {/each}
    </Accordion>
  {:else}
    <div class="flex flex-row flex-grow justify-center items-center w-full text-center">
      <Heading tag="h2" class="text-background-light-700 dark:text-background-mix-300">No Shortcuts</Heading>
    </div>
  {/if}
</main>

<AddShortcutModal bind:open={addShortcutModalOpen} />
<EditShortcutModal bind:open={editShortcutModalOpen} {...editShortcutModalData} on:close={() => editShortcutModalData = null} />

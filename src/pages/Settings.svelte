<script lang="ts">
  import { CheckCircle, XCircle, ChevronLeft, Trash, ArrowPath } from 'svelte-heros-v2'
  import { Heading, Accordion, AccordionItem, Button, Toast } from 'flowbite-svelte'
  import { getVersion, getTauriVersion, getName } from '@tauri-apps/api/app'
  import ToastContainer from '../lib/ToastContainer.svelte'
  import { getSetting } from '../utils/settings'
  import { invoke } from '@tauri-apps/api/tauri'
  import { onMount } from 'svelte'

  let okayMessage = ''
  let showOkay = false
  let errorMessage = ''
  let showError = false

  let appName
  let appVersion
  let tauriVersion

  onMount(async () => {
    appVersion = await getVersion()
    tauriVersion = await getTauriVersion()
    appName = await getName()
  })

  function getCopyrightNoticeYear() {
    const releaseYear = 2023
    const now = new Date()

    if(now.getFullYear() === releaseYear) return `${releaseYear}`

    return `${releaseYear}-${now.getFullYear()}`
  }

  const unitButtonIconSize = '16'
</script>

<main class="flex flex-col pt-12 px-8 h-full">
  <header class="flex flex-row justify-between items-center">
    <Heading>Settings</Heading>
    <div class="flex flex-row justify-end items-center gap-2">
      <Button
        outline
        color="alternative"
        class="alternative !p-2 !h-9"
        on:click={() => window.history.back()}>
        <ChevronLeft class="outline-none" size="1.25rem" />
      </Button>
    </div>
  </header>
  <br/>

  <Accordion>
    <AccordionItem>
      <span slot="header" class="flex flex-row gap-1 items-center">
        <Trash size={unitButtonIconSize} />
        Clean & Fix
      </span>
      <div class="flex flex-col gap-3">
        <Button color="red" class="flex flex-row items-center gap-2" on:click={async () => {
          const ids = await getSetting('shortcuts', [])

          await Promise.all(
            ids.map(async id => {
              console.log(`cleaning ${id}`)
              await invoke('cleanup_shortcut', { id })
            })
          )

          showOkay = false
          setTimeout(() => {
            showOkay = true
            okayMessage = 'Cleared!'
          }, 100)
        }}>
          <Trash size="16" />
          Clear Shortcuts
        </Button>
        <Button
          color="alternative"
          class="flex flex-row items-center gap-2"
          on:click={async () => {
            await invoke('restart_explorer')
          }}>
          <ArrowPath size={unitButtonIconSize} />
          Restart Explorer
        </Button>
      </div>
    </AccordionItem>
    <AccordionItem>
      <div slot="header" class="w-full mr-4 flex justify-between items-center">
        <span>ⓘ About</span>
      </div>
      <span>
        This app was made using <b>Svelte</b>, <b>Tauri</b>, <b>Svelte Heros v2</b> & <b>Flowbite</b>. <br/><br/>
        <b>App Version</b>: {appVersion} <br/>
        <b>Tauri Version</b>: {tauriVersion} <br/>
        <b>App Bundle Name</b>: {appName} <br/> <br/>

        © {getCopyrightNoticeYear()} bennett-sh
      </span>
    </AccordionItem>
  </Accordion>
</main>

<ToastContainer showOverModal>
  <Toast bind:open={showError} on:close={() => showError = false} color="red">
    <svelte:fragment slot="icon">
      <XCircle color="#f98080" />
    </svelte:fragment>
    {errorMessage}
  </Toast>
  <Toast bind:open={showOkay} on:close={() => showOkay = false}>
    <svelte:fragment slot="icon">
      <CheckCircle color="#00ff00" />
    </svelte:fragment>
    {okayMessage}
  </Toast>
</ToastContainer>


<script lang="ts">
  import { Button, Toast, Tooltip } from 'flowbite-svelte'
  import { writeText } from '@tauri-apps/api/clipboard'
  import { Clipboard, XCircle } from 'svelte-heros-v2'
  import ToastContainer from './ToastContainer.svelte'

  let error = ''
  let showError = false
  let hasCopied = false
  let content: HTMLElement
</script>

<span class="flex flex-row items-center gap-1">
  <code bind:this={content} class="text-sm select-none">
    <slot />
  </code>
  <Button
    color="alternative"
    class="alternative"
    btnClass="text-button"
    on:click={() => {
      writeText(content.textContent)
        .catch(err => {
          console.error('failed to write to clipboard', err)
          error = err.toString()
          showError = true
        })
        .then(() => {
          hasCopied = true
          setTimeout(() => hasCopied = false, 3000)
        })
    }}>
    <Clipboard size="1rem" />
  </Button>
  <Tooltip
    placement="right">{hasCopied ? 'Copied!' : 'Copy'}</Tooltip>
</span>

<ToastContainer>
  <Toast bind:open={showError} on:close={() => showError = false} color="red">
    <svelte:fragment slot="icon">
      <XCircle color="#f98080" />
    </svelte:fragment>
    Failed to copy to clipboard{error.length > 0 ? `: ${error}` : ''}
  </Toast>
</ToastContainer>

<script lang="ts">
  import { writeText } from '@tauri-apps/api/clipboard'
  import { Button, Tooltip } from 'flowbite-svelte'
  import { showQuickToast } from '../utils/toasts'
  import { Clipboard } from 'svelte-heros-v2'

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
          showQuickToast({ type: 'Error', message: `Failed to write to clipboard: ${err}` })
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

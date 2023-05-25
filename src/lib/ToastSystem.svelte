<script lang="ts">
	import { XCircle, CheckCircle } from 'svelte-heros-v2'
  import { toastStore } from '../utils/toasts'
  import type { ToastData } from '../types'
  import { Toast } from 'flowbite-svelte'

  let toasts: ToastData[] = []

  toastStore.subscribe(data => toasts = data)
</script>

<div class="absolute bottom-4 right-4 flex flex-col-reverse gap-2 items-center justify-start z-20">
  {#each toasts as { type, message, open, onClose, id }, index}
    <Toast bind:open={open} on:close={() => {
      toastStore.update(toastsArray => {
        const result = onClose({ id })
        if(result !== false) {
          toastsArray[index].open = false
        }
        return toastsArray
      })
    }}>
      <svelte:fragment slot="icon">
        {#if type === 'Okay'}
          <CheckCircle color="#00ff00" />
        {:else if type === 'Error'}
          <XCircle color="#ff0000" />
        {/if}
      </svelte:fragment>
      {message}
    </Toast>
  {/each}
</div>
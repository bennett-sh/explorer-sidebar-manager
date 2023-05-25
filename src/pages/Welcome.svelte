<script lang="ts">
  import { Heading, Button, Card } from 'flowbite-svelte'
  import { setSetting } from '../utils/settings'
  import { PaintBrush } from 'svelte-heros-v2'
  import { navigate } from 'svelte-routing'

  async function exit() {
    await setSetting('hasBeenWelcomed', true)
    navigate('/')
  }
</script>

<main class="flex flex-row justify-around items-center px-8 h-full">
  <div class="flex flex-col justify-center gap-16 items-center">
    <Heading class="text-center">Welcome to ESM</Heading>
    <Button
      class="
        animate-bounce
      "
      on:click={exit}>
      <div class="flex flex-row items-center gap-1">
        <PaintBrush class="outline-none" size="1.25rem" />
        Start Customizing
      </div>
    </Button>
  </div>

  <div class="flex flex-col h-full items-center justify-center min-w-[50vw] gap-6">
    <Card>
      <Heading tag="h3">Icons</Heading>
      To add icons to your shortcuts you'll need to specify the path to an executable, a dll, an ico or an icl file and an icon index. You can use a tool like <a class="inline" href="http://www.angusj.com/resourcehacker/#download" rel="noreferrer noopener">ResourceHacker</a> or the built-in icon chooser to find the proper icon index.
    </Card>
    <Card>
      <Heading tag="h3">Before Uninstalling</Heading>
      If you decide to uninstall ESM it's recommended to go to settings and run the clean up function before to prevent left-over shortcuts.
    </Card>
  </div>
</main>

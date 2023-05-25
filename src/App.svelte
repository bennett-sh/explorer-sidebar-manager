<script lang="ts">
  import ToastSystem from './lib/ToastSystem.svelte'
  import { Router, Route } from 'svelte-routing'
  import Settings from './pages/Settings.svelte'
  import { getSetting } from './utils/settings'
  import Welcome from './pages/Welcome.svelte'
  import { navigate } from 'svelte-routing'
  import Main from './pages/Main.svelte'
  import { onMount } from 'svelte'

  onMount(async () => {
    if(!await getSetting('hasBeenWelcomed', false)) {
      navigate('/welcome')
    }
  })
</script>

<Router url={window.location.pathname}>
  <Route path="/" component={Main} />
  <Route path="/settings" component={Settings} />
  <Route path="/welcome" component={Welcome} />
</Router>

<ToastSystem />

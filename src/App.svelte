<script lang="ts">
  import ToastSystem from './lib/ToastSystem.svelte'
  import { Router, Route } from 'svelte-routing'
  import Settings from './pages/Settings.svelte'
  import Main from './pages/Main.svelte'
</script>

<Router url={window.location.pathname}>
  <Route path="/" component={Main} />
  <Route path="/settings" component={Settings} />
</Router>

<ToastSystem />

<script>
  import { Router, Route } from "svelte-routing";
  import Home from '../routing/Home.svelte';

  export const url = '';
</script>

<Router url={url} > 
  <Route path="/home" component={Home} />
</Router>
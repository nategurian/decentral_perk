<script>
  import { Nav, Button } from 'svelte-chota';
  import { Router, Route, Link } from 'svelte-routing';

  import Home from '../routing/Home.svelte';

</script>

<div>
  <Router>
    <Nav>
      <div class="nav" slot="left">
        <Link class="brand" to="/">Decentral Perk</Link>
      </div>
      <div class="nav" slot="right">
        <Button>
          <a href="/">Login</a>
        </Button>
        <Button>
          <a href="/">Sign Up</a>
        </Button>
      </div>
    </Nav>  
    <Route path="/" component={Home} />
  </Router>
</div>

<style>
  .nav {
    display: inline-flex;
  }
</style>
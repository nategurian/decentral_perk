<script>
  import { Nav, Button } from 'svelte-chota';
  import { Router, Route, Link } from 'svelte-routing';

  import Home from '../routing/Home.svelte';

</script>

<div>
  <Router>
    <div class="nav-container">
      <Nav>
        <div class="nav" slot="left">
          <Link class="brand" to="/">Decentral Perk</Link>
        </div>
        <div class="nav" slot="right">
          <Button primary>
            Login
          </Button>
          <Button primary outline>
            Sign Up
          </Button>
        </div>
      </Nav> 
    </div>
    <div>
      <Route path="/" component={Home} />
    </div> 
  </Router>
</div>

<style>
  .nav-container {
    background-color: #c6c6c6;
    padding: 10px;
  }
  .nav {
    display: inline-flex;
  }
</style>
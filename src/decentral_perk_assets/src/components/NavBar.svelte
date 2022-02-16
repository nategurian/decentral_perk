<script>
  import { Nav, Button } from 'svelte-chota';
</script>

<div>
  <Nav>
    <div class="nav" slot="left">
      <a class="brand" href="/">(de)Central Perk</a>
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
</div>

<style>
  .nav {
    display: inline-flex;
  }
</style>
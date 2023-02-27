<script>
  import { Nav, Button } from 'svelte-chota';
  import { Router, Route, Link } from 'svelte-routing';
  import { AuthClient } from '@dfinity/auth-client';
  import { auth, createActor } from '../stores/auth';
  import Home from '../routing/Home.svelte';
  import { onMount } from 'svelte';

  // Variables
  /** @Type {AuthClient} */
  let client;

  onMount(async() => {
    // Create auth client for Internet Identity
    client = await AuthClient.create();
    const persistedAuth = localStorage.getItem('auth');
    if(persistedAuth) {
      const parsedPersistedAuth = JSON.parse(persistedAuth);
      parsedPersistedAuth.principal = client.getIdentity().getPrincipal();
      parsedPersistedAuth.actor = createActor({
        agentOptions: {
          identity: client.getIdentity()
        }
      })
      auth.update(() => parsedPersistedAuth);
    }
  })

  // Auth Actions
  const handleLogin = async () => {
    client.login({
      identityProvider: process.env.DFX_NETWORK === 'ic' ? 'https://identity.ic0.app/#authorize' : `http://${process.env.INTERNET_IDENTITY_CANISTER_ID}.localhost:8000/#authorize`,
      onSuccess: handleAuth,
    })
  };

  const handleAuth = async () => {
    auth.update(() => ({
      principal: client.getIdentity().getPrincipal(),
      actor: createActor({
        agentOptions: {
          identity: client.getIdentity()
        }
      })
    })); 
    // Note $uth.actor does not save properly to local storage.
    localStorage.setItem('auth', JSON.stringify($auth));
    console.log('Principal: ', $auth.principal.toString())
  };

  const handleLogout = async () => {
    console.log('Logging out...');
    localStorage.clear();
    auth.update(() => ({
      principal: '',
      actor: createActor() // created anon actor.
    }));
    localStorage.setItem('auth', JSON.stringify($auth)); 
  };
</script>

<div>
  <Router>
    <div class="nav-container">
      <Nav>
        <div class="nav" slot="left">
          <Link class="brand" to="/">deCentral Perk</Link>
        </div>
        <div class="nav" slot="right">
          {#if $auth.principal === ''}
            <Button primary on:click={handleLogin}>
              Login
            </Button>
          {:else}
            <Button dropdown={`${$auth.principal.toString().substring(0, 15)}...`} primary>
              <p class="menu-option" on:click={handleLogout}>Logout</p>
            </Button>
          {/if}
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

  .menu-option {
    cursor: pointer;
  }
</style>
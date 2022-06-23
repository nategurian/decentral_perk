<script>
  import { Nav, Button } from 'svelte-chota';
  import { Router, Route, Link } from 'svelte-routing';
  import { AuthClient } from '@dfinity/auth-client';
  import { auth, createActor } from '../stores/auth';
  import { vendorStore } from '../stores/vendorStore';
  import Home from '../routing/Home.svelte';
  import Dashboard from '../routing/VendorDashboard.svelte';
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
      if($auth.loggedIn) {
        const result = await $auth.actor.getMyStore();
        console.log('My Store: ', result);
        if(result.Ok) {
          vendorStore.set(result.Ok);
          localStorage.setItem('vendor', JSON.stringify(result.Ok));
        }
      }
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
      loggedIn: true,
      principal: client.getIdentity().getPrincipal(),
      actor: createActor({
        agentOptions: {
          identity: client.getIdentity()
        }
      })
    })); 
    // Note $uth.actor does not save properly to local storage.
    localStorage.setItem('auth', JSON.stringify($auth));
    const result = await $auth.actor.getMyStore();
    console.log('My Store: ', result);
    console.log('Principal: ', $auth.principal.toString())
    if(result.Ok) {
      vendorStore.set(result.Ok);
      localStorage.setItem('vendor', JSON.stringify(result.Ok));
    }
  };

  const handleLogout = async () => {
    console.log('Logging out...');
    localStorage.clear();
    vendorStore.set(undefined)
    auth.update(() => ({
      loggedIn: false,
      principal: '',
      actor: createActor() // created anon actor.
    }));
    const test = await $auth.actor.getMyStore();
    console.log('Test result: ', test)
    localStorage.setItem('auth', JSON.stringify($auth)); 
  };
</script>

<div>
  <Router>
    <div class="nav-container">
      <Nav>
        <div class="nav" slot="left">
          <Link class="brand" to="/">Decentral Perk</Link>
        </div>
        <div class="nav" slot="right">
          {#if !$auth.loggedIn}
            <Button primary on:click={handleLogin}>
              Login
            </Button>
            <Button primary outline>
              Sign Up
            </Button>
          {:else}
            <Button dropdown={`${$auth.principal.toString().substring(0, 15)}...`} primary>
              <p class="menu-option">My Orders</p>
              {#if $vendorStore}
                <Link class="nav-link" to="dashboard">My Store</Link>
              {/if}
              <hr />
              <p class="menu-option" on:click={handleLogout}>Logout</p>
            </Button>
          {/if}
        </div>
      </Nav> 
    </div>
    <div>
      <Route path="/" component={Home} />
      <Route path="dashboard" component={Dashboard} />
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
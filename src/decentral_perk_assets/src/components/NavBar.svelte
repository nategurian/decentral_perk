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

  const iiCanisterId = '';

  onMount(async() => {
    // Create auth client for Internet Identity
    client = await AuthClient.create();
  })

  // Auth Actions
  const handleLogin = async () => {
    client.login({
      identityProvider: process.env.DFX_NETWORK === 'ic' ? 'https://identity.ic0.app/#authorize' : `http://${process.env.INTERNET_IDENTITY_CANISTER_ID}.localhost:8000/#authorize`,
      onSucess: handleAuth
    })
  };

  const handleAuth = () => {
    auth.update(() => ({
      loggedIn: true,
      principal: client.getIdentity().getPrincipal(),
      actor: createActor({
        agentOptions: {
          identity: client.getIdentity()
        }
      })
    })) 
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
</style>
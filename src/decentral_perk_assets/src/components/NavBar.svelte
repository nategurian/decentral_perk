<script>
  import { Nav, Button } from 'svelte-chota';
  import { Router, Route, Link } from 'svelte-routing';
  import { AuthClient } from '@dfinity/auth-client';
  import { Principal } from "@dfinity/principal";
  import { auth, createActor } from '../stores/auth';
  import { host } from '../stores/store';
  import Home from '../routing/Home.svelte';
  import { onMount } from 'svelte';
  import { HttpAgent, Actor } from '@dfinity/agent';
  import { idlFactory } from '../../../declarations/decentral_perk/decentral_perk.did.js';

  // Variables
  /** @Type {AuthClient} */
  let client;
  let backendActor;

  const iiCanisterId = '';
  

  onMount(async() => {
    // Create auth client for Internet Identity
    client = await AuthClient.create();
    const persistedAuth = localStorage.getItem('auth');
    if(persistedAuth) {
      const parsedPersistedAuth = JSON.parse(persistedAuth);
      parsedPersistedAuth.principal = client.getIdentity().getPrincipal();
      auth.update(() => parsedPersistedAuth);
    }
    console.log($auth)
    if($auth.loggedIn) {
      // Fetch to see if user is vendor and retrieve store.
      const identity = client.getIdentity();
      const agent = new HttpAgent({identity, $host});

      if(process.env.DFX_NETWORK === 'local')
        agent.fetchRootKey().catch((err) => {
          console.warn('Unable to fetchrootkey')
        });

      backendActor = Actor.createActor(idlFactory, {
        agent,
        canisterId: process.env.DECENTRAL_PERK_CANISTER_ID
      });
      const result = await backendActor.getMyStore();
      //TODO: Set store to global variable
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
    })) 
    localStorage.setItem('auth', JSON.stringify($auth));

    // Get Vendor for user:
    // Fetch to see if user is vendor and retrieve store.
    const identity = client.getIdentity();
    const agent = new HttpAgent({identity, $host});

    if(process.env.DFX_NETWORK === 'local')
      agent.fetchRootKey().catch((err) => {
        console.warn('Unable to fetchrootkey')
      });

    backendActor = Actor.createActor(idlFactory, {
      agent,
      canisterId: process.env.DECENTRAL_PERK_CANISTER_ID
    });
    const result = await backendActor.getMyStore();
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
              <p>My Orders</p>
              <p>My Store</p>
              <hr />
              <p>Logout</p>
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
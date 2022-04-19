<script>
  import { Nav, Button } from 'svelte-chota';
  import { Router, Route, Link } from 'svelte-routing';
  import { AuthClient } from '@dfinity/auth-client';
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
      console.log('Fetching My Store')
      const result = await backendActor.getMyStore();
      console.log(result);
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
    console.log('My Store: ', result);
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
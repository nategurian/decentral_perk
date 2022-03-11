import { writable } from 'svelte/store';
import { Actor, HttpAgent } from '@dfinity/agent';


// Creates actor for backend canister 
export const createActor = (options) => {
  const hostOptions = {
    host: 
      process.env.DFX_NETWORK === 'ic'
        ? `https://${process.env.BACKEND_CANISTER_ID}.ic0.app`
        : 'http://localhost:8000'
  };

  if(!options) {
    options = {
      agentOptions: hostOptions
    };
  }
  else if(!options.agentOptions) {
    options.agentOptions = hostOptions,
  }
  else {
    options.agentOptions.host = hostOptions.host;
  }
  
  const agent = new HttpAgent({...options.agentOptions});

  // Need to fetch root key for local development only
  if(process.env.DFX_NETWORK === 'local') {
    agent.fetchRootKey().catch((err) => {
      console.warn('Unable to fetch root key, please ensure local replica is running');
    })
  };

  return Actor.createActor(idlFactory, {
    agent,
    canisterId: process.env.BACKEND_CANISTER_ID,
    ...options?.actorOptions
  });
};

export const auth = writable({
  loggedIn: false,
  principal: '',
  actor: createActor()
})
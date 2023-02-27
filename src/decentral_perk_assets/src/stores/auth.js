import { writable } from 'svelte/store';
import { Actor, HttpAgent } from '@dfinity/agent';
import { idlFactory } from '../../../declarations/decentral_perk/decentral_perk.did.js';


// Creates actor for backend canister 
export const createActor = (options) => {
  const hostOptions = {
    host: 
      process.env.DFX_NETWORK === 'ic'
        ? `https://${process.env.DECENTRAL_PERK_CANISTER_ID}.ic0.app`
        : 'http://localhost:8001'
  };

  if(!options) {
    options = {
      agentOptions: hostOptions
    };
  }
  else if(!options.agentOptions) {
    options.agentOptions = hostOptions;
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
    canisterId: process.env.DECENTRAL_PERK_CANISTER_ID,
    ...options?.actorOptions
  });
};

export const auth = writable({
  principal: '',
  actor: createActor()
})
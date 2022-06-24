import { writable } from 'svelte/store';

export const host = writable(
  process.env.DFX_NETWORK === 'local'
    ? 'http://localhost:8000'
    : 'ic0.app'
)

export const vendors = writable([]);
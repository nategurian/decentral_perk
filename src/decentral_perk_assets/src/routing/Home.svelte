<script>
  import { onMount } from 'svelte';
  import { vendors } from '../stores/store';
  import { vendorStore } from '../stores/vendorStore';
  import VendorCard from '../components/VendorCard/VendorCard.svelte';
 
  onMount(() => {
    // Check localStorage (for page refresh instances)
    const persistedVendor = localStorage.getItem('vendor');
    if(persistedVendor)
      vendorStore.set(JSON.parse(persistedVendor))
  });
  console.log('Vendors: ', $vendors)
</script>

<div class="page-container">
  {#if $vendors.length > 0}
    {#each $vendors as vendor}
      <VendorCard vendor={vendor}  />
    {/each}
  {/if}
</div>

<script>
  import { onMount } from 'svelte';
  import { vendorStore } from '../stores/vendorStore';
 
  onMount(() => {
    // Check localStorage (for page refresh instances)
    const persistedVendor = localStorage.getItem('vendor');
    if(persistedVendor)
      vendorStore.set(JSON.parse(persistedVendor))
  });
</script>

<div class="page-container">
  <div>
    {$vendorStore.name}
  </div>
  <div>
    {$vendorStore.description}
  </div>
  <div>
    {$vendorStore.website}
  </div>
</div>
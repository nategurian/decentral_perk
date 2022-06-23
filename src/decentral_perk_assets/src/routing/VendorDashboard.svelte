<script>
  import { onMount } from 'svelte';
  import { vendorStore } from '../stores/vendorStore';
 
  onMount(() => {

  });
</script>
<div class="page-container">
  <div>
    {$vendorStore.name}
  </div>
</div>
<script>
  import {Card} from 'svelte-chota'
  export let vendor;
</script>

<div class="vendor-card">
  <Card>
    <h4 slot="header">{vendor.name}</h4>
    {vendor.description}
    <div slot="footer" class="is-right">
      <span>&</span>
    </div>
  </Card>
</div>
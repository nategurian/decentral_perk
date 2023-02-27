<script>
  import { onMount } from 'svelte';
  import { auth } from '../stores/auth';
  import { posts } from '../stores/post';
  import PostDisplay from '../components/PostDisplay/PostDisplay.svelte';
 
  onMount(async () => {
    //console.log($auth.actor)
    posts = await $auth.actor.get_posts();
  });
</script>

<div class="page-container">
  <div class="post-options-container">

  </div>
  <div class="posts">
    {#each $posts as post}
      <PostDisplay post={post} /> 
    {/each}
  </div>
</div>

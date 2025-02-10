<script>
  import "../app.css";
  let { children } = $props();
  // import "../app.css";
  import { Navigation } from '@skeletonlabs/skeleton-svelte';
  import IconTrees from 'lucide-svelte/icons/trees';
  import IconApple from 'lucide-svelte/icons/apple';
</script>

<div class="card border-surface-100-900 grid h-[640px] w-full grid-cols-[auto_1fr] border-[1px]">
    <Navigation.Rail>
      {#snippet header()}
        <Navigation.Tile href="/" title="Menu" label="Home"></Navigation.Tile>
      {/snippet}
      {#snippet tiles()}
        <Navigation.Tile id="0" label="Drey" href="/drey">
          <IconTrees />
        </Navigation.Tile>
        <Navigation.Tile id="0" label="Canopy" href="/canopy">
          <IconApple />
        </Navigation.Tile>
        <Navigation.Tile id="0" label="Mycelium" href="/mycelium">
          <IconTrees />
        </Navigation.Tile>

        <Navigation.Tile id="1" label="Test" href="/test">
        </Navigation.Tile>
      {/snippet}
      {#snippet footer()}
        <Navigation.Tile labelExpanded="Settings" href="#" title="settings" label="Settings"></Navigation.Tile>
      {/snippet}
    </Navigation.Rail>
    <div class="flex items-center justify-center">

      {@render children()}
      <!-- <p class="opacity-20">(Content)</p> -->
    </div>
  </div>
  
  


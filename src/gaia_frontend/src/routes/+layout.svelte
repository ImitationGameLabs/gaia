<script>
  import "../app.css";
  import { Navigation } from '@skeletonlabs/skeleton-svelte';
  import IconHouse from 'lucide-svelte/icons/house';
  import IconTrees from 'lucide-svelte/icons/trees';
  import IconApple from 'lucide-svelte/icons/apple';
  import IconBug from 'lucide-svelte/icons/bug';
  import IconWaypoints from 'lucide-svelte/icons/waypoints';

  let { children } = $props();
</script>

<div>
  <header class="sticky top-0 h-20 w-full bg-red-500"></header>
  <main class="border-surface-100-900 grid h-[640px] w-full grid-cols-[auto_1fr] border-[1px]">
      <div>
        <Navigation.Rail>
          {#snippet header()}
          <!-- <Navigation.Tile href="/" title="Menu" label="Home"></Navigation.Tile> -->
          {/snippet}
          {#snippet tiles()}
            <Navigation.Tile id="0" label="Nest" href="/nest">
              <IconHouse />
            </Navigation.Tile>
            <Navigation.Tile id="1" label="Fruits" href="/fruits">
              <IconApple />
            </Navigation.Tile>
            <Navigation.Tile id="2" label="Mycelium" href="/mycelium">
              <IconWaypoints />
            </Navigation.Tile>
            <Navigation.Tile id="3" label="Forest" href="/forest">
              <IconTrees />
            </Navigation.Tile>
            <Navigation.Tile id="4" label="Test" href="/test">
              <IconBug />
            </Navigation.Tile>
          {/snippet}
          {#snippet footer()}
            <!-- <Navigation.Tile labelExpanded="Settings" href="#" title="settings" label="Settings"></Navigation.Tile> -->
          {/snippet}
        </Navigation.Rail>
      </div>
     
      <div class="min-h-screen h-fit w-full flex items-center justify-center bg-green-500">
  
        {@render children()}
        <!-- <p class="opacity-20">(Content)</p> -->
      </div>
  </main>
  <footer></footer>
</div>

  
  


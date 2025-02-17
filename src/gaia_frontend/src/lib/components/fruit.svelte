<script lang="ts">
    import type { Fruit } from "declarations/gaia/gaia.did";

    interface Props {
      fruit: Fruit;
	  }

    let { fruit }: Props = $props();
</script>

<a
  href="/#"
  class="card preset-filled-surface-100-900 border-[1px] border-surface-200-800 card-hover divide-surface-200-800 block max-w-md divide-y overflow-hidden h-full"
>
  <header>
    <!-- <img src={imgSrc} class="aspect-[21/9] w-full grayscale hue-rotate-90" alt="banner" /> -->
  </header>
  <article class="space-y-4 p-4">
    <div>
      <!-- <h2 class="h6">Haha Fruit</h2> -->
      <h3 class="h3">{fruit.title}</h3>
    </div>
    <p class="opacity-60">
      {fruit.description}
    </p>
  </article>
  <footer class="flex items-center justify-between gap-4 p-4">
    <medium class="opacity-60">Reward {fruit.rewards} GAIA</medium>
    <medium class="opacity-60">By {fruit.creator}</medium>
    <medium class="opacity-60">On {new Date(Number(fruit.created_at)).toLocaleDateString()}</medium>
  </footer>
</a>
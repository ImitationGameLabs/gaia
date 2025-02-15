<script lang="ts">
    // Fruit object
    let { fruit } = $props();


</script>

<a
  href="/#"
  class="card preset-filled-surface-100-900 border-[1px] border-surface-200-800 card-hover divide-surface-200-800 block max-w-md divide-y overflow-hidden"
>
  <header>
    <!-- <img src={imgSrc} class="aspect-[21/9] w-full grayscale hue-rotate-90" alt="banner" /> -->
  </header>
  <article class="space-y-4 p-4">
    <div>
      <h2 class="h6">Haha Fruit</h2>
      <h3 class="h3">{fruit.name}</h3>
    </div>
    <p class="opacity-60">
      Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam aspernatur provident eveniet eligendi cumque consequatur tempore sint
      nisi sapiente. Iste beatae laboriosam iure molestias cum expedita architecto itaque quae rem.
    </p>
  </article>
  <footer class="flex items-center justify-between gap-4 p-4">
    <small class="opacity-60">By Alex</small>
    <small class="opacity-60">On {new Date().toLocaleDateString()}</small>
  </footer>
</a>
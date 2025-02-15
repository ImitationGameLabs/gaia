<script lang='ts'>
    import Fruit from '$lib/components/fruit.svelte'
    import { fruits } from './fruits_testdata'

</script>

<div class="h-screen w-full grid grid-rows-[auto_1fr_auto]">
	<header class="bg-red-500 p-4">Fruits</header>
	<main class="bg-green-500 p-4 space-y-4 columns-3">
        {#each fruits as fruit (fruit.id)}
            <Fruit fruit={fruit} />
        {/each}
	</main>
	<footer class="bg-blue-500 p-4">(footer)</footer>
</div>


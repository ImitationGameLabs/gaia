<script lang='ts'>
    import Fruit from '$lib/components/fruit.svelte'
    import { fruits } from './fruits_testdata'

</script>

<div class="h-full w-full bg-green-500">
    <div class="grid grid-cols-3 p-4 space-y-4 ">
        {#each fruits as fruit (fruit.id)}
            <Fruit fruit={fruit} />
        {/each}
    </div>
</div>



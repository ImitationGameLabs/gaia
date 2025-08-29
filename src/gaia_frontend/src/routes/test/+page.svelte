<script>
    import { auth } from "$lib/auth";
    import Auth from '$lib/components/auth.svelte';
    let whoami = $state("");

    function onSubmit(event) {
        const name = event.target.name.value;
        $auth.agent.getPrincipal().then((response) => {
            whoami = response;
        });
        return false;
    }
</script>

<div class="bg-surface-100-900">
    <form action="#" onsubmit={onSubmit} class="mx-auto w-full max-w-md space-y-4">
        <label for="name">Enter your name: &nbsp;
            <input id="name" alt="Name" type="text" class="input border" />
        </label>
        
        <button type="submit" class="preset-filled-primary-500 btn">Click Me!</button>
    </form>
    <br />
    <br />
    <section id="greeting" class="">Hi, {whoami}</section>

    <Auth />
</div>

<script>
    import { auth } from "$lib/auth";
    import Auth from '$lib/components/auth.svelte';
    let whoami = $state("");

    function onSubmit(event) {
        const name = event.target.name.value;
        $auth.actor.whoami().then((response) => {
            whoami = response;
        });
        return false;
    }
</script>

<div>
    <h>Trees</h>
    <h2>hahah </h2>
    
    <form action="#" onsubmit={onSubmit}>
        <label for="name">Enter your name: &nbsp;</label>
        <input id="name" alt="Name" type="text" />
        <button type="submit">Click Me!</button>
    </form>
    <br />
    <br />
    <section id="greeting">Hi, {whoami}</section>

    <Auth />
</div>

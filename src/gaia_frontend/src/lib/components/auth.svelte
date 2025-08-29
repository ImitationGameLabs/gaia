<script lang="ts">
    import { AuthClient } from "@dfinity/auth-client";
    import { onMount } from "svelte";
    import { auth, createActors, createAgentHost, defaultAuth, defaultIdentity } from "../auth";
    import { HttpAgent } from "@icp-sdk/core/agent";
  
    let client: AuthClient;
    
    let whoami = $state(defaultIdentity.getPrincipal());
  
    onMount(async () => {
      client = await AuthClient.create();
      if (await client.isAuthenticated()) {
        handleAuth();
      }
    });
  
    function handleAuth() {
      const agent = HttpAgent.createSync({
        host: createAgentHost(),
        identity: client.getIdentity(),
      })
      
      auth.update(() => ({
        loggedIn: true,
        agent,
        actors: createActors(agent),
      }));
  
      whoami = client.getIdentity().getPrincipal();
    }
  
    function login() {
      client.login({
        identityProvider:
          process.env.DFX_NETWORK === "ic"
            ? "https://identity.ic0.app/#authorize"
            : `http://${process.env.CANISTER_ID_INTERNET_IDENTITY}.localhost:4943/#authorize`,
        onSuccess: handleAuth,
      });
    }
  
    async function logout() {
      await client.logout();

      auth.update(() => (defaultAuth));
  
      whoami = await $auth.agent.getPrincipal();
    }
</script>
  
<div class="container">
  <div>
    {#if $auth.loggedIn}
      <button onclick={logout} class="rounded-2xl preset-filled-secondary-500">Log out</button>
    {:else}
      <button onclick={login} class="rounded preset-filled-primary-500">Authenticate in with Internet Identity</button>
    {/if}
  </div>

  <div class="principal-info">
      Your principal ID is
      <code>{whoami}</code>

      {#if whoami.isAnonymous()}
        (anonymous)
      {:else if !$auth.loggedIn}
        (guest)
      {/if}
  </div>
</div>

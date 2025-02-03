
function identityProvider() {
    if (process.env.DFX_NETWORK === "ic") {
        return `https://${process.env.CANISTER_ID_INTERNET_IDENTITY}.ic0.app`;
    } else {
        return `http://${process.env.CANISTER_ID_INTERNET_IDENTITY}.localhost:4943`;
    }
}

/**
 * Replace backend actor identity with the identity from AuthClient,
 * additionally re-render to show the updated authentication status
 */
async function onIdentityUpdate() {
    Actor.agentOf(ii_integration_backend).replaceIdentity(this.authClient.getIdentity());
    this.isAuthenticated = await this.authClient.isAuthenticated();
    this.#render();
  }
  
  /**
   * Create AuthClient, this loads an existing session if available
   */
async function createAuthClient() {
    this.authClient = await AuthClient.create();
    await this.#onIdentityUpdate();
}
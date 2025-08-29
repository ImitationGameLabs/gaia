import { writable } from "svelte/store";
import { idlFactory } from "declarations/forest";
import { Actor, HttpAgent } from "@dfinity/agent";
import type { HttpAgentOptions, ActorConfig } from "@dfinity/agent";

// Creates an actor for the Backend canister
type Options = {
  agentOptions?: HttpAgentOptions,
  actorOptions?: ActorConfig,
}

export function createActor(options: Options) {
  const hostOptions = {
    host:
      process.env.DFX_NETWORK === "ic"
        ? `https://${process.env.CANISTER_ID_FOREST}.ic0.app`
        : undefined,
  };

  if (!options) {
    options = {
      agentOptions: hostOptions,
    };
  } else if (!options.agentOptions) {
    options.agentOptions = hostOptions;
  } else {
    options.agentOptions.host = hostOptions.host;
  }

  const agent = HttpAgent.createSync({ ...options.agentOptions });

  // Fetch root key for certificate validation during development
  if (process.env.DFX_NETWORK !== "ic") {
    agent.fetchRootKey().catch((err) => {
      console.warn(
        "Unable to fetch root key. Check to ensure that your local replica is running"
      );
      console.error(err);
    });
  }

  // Creates an actor with using the candid interface and the HttpAgent
  let actor = Actor.createActor(idlFactory, {
    agent,
    canisterId: process.env.CANISTER_ID_FOREST,
    ...options?.actorOptions,
  });

  return actor;
}

export const auth = writable({
  loggedIn: false,
  actor: createActor(),
});
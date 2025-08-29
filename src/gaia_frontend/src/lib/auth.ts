import { writable } from "svelte/store";
import { Ed25519KeyIdentity } from "@icp-sdk/core/identity";
import { HttpAgent } from "@icp-sdk/core/agent";

import { canisterId as forestCanisterId, createActor as createForestActor } from "declarations/forest";
import { canisterId as gaiaCanisterId, createActor as createGaiaActor } from "declarations/gaia";

export function createAgentHost(): string | undefined {
  return process.env.DFX_NETWORK === "ic"
    ? "https://icp-api.io"
    : undefined
}

export function createActors(agent: HttpAgent) {
  return {
    forest: createForestActor(forestCanisterId),
    gaia: createGaiaActor(gaiaCanisterId)
  }
}

export const defaultIdentity = Ed25519KeyIdentity.generate()

const defaultAgent = HttpAgent.createSync({
  host: createAgentHost(),
  identity: defaultIdentity,
})

export const defaultAuth = {
  loggedIn: false,
  agent: defaultAgent,
  actors: createActors(defaultAgent)
}

export const auth = writable(defaultAuth);
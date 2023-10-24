import { writable } from "svelte/store";
import type { Preferences } from "./bindings";

export const animationsOn = writable(true);
export const unceasingAnimations = writable(false);
export const animationSpeed = writable(4);
export const soundOn = writable(true);
export const volume = writable(1);

export const NullPreferences: Preferences = {
  animations_on: null,
  unceasing_animations: null,
  sound_on: null,
  volume: null,
};
import type { StoryFn, Decorator, StoryContext } from "@storybook/svelte";
import { unceasingAnimations, animationSpeed } from "$lib/preferences";

interface Preferences {
  unceasingAnimations?: boolean;
  animationSpeed?: number;
}

interface StoreArgs {
  preferences?: Preferences;
  [key: string]: any;
}

const SvelteStoresDecorator: Decorator = (
  story: StoryFn,
  context: StoryContext,
) => {
  const { args, parameters } = context;
  const { preferences } = parameters as StoreArgs;
  if (preferences?.unceasingAnimations !== undefined) {
    unceasingAnimations.set(preferences.unceasingAnimations);
  }
  if (preferences?.animationSpeed === undefined) {
    animationSpeed.set(1);
  } else {
    animationSpeed.set(preferences.animationSpeed);
  }

  return story(args, context);
};

export default SvelteStoresDecorator;

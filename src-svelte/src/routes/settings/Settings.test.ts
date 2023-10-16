import { expect, test, vi, assert } from "vitest";
import { get } from "svelte/store";
import "@testing-library/jest-dom";

import { act, render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import Settings from "./Settings.svelte";
import { soundOn } from "../../preferences";
import fs from "fs";
import yaml from "js-yaml";
import { Convert } from "$lib/sample-call";

const tauriInvokeMock = vi.fn();

vi.stubGlobal("__TAURI_INVOKE__", tauriInvokeMock);

interface ParsedCall {
  request: (string | Record<string, string>)[];
  response: Record<string, string>;
}

function parseSampleCall(sampleFile: string): ParsedCall {
  const sample_call_yaml = fs.readFileSync(sampleFile, "utf-8");
  const sample_call_json = JSON.stringify(yaml.load(sample_call_yaml));
  const rawSample = Convert.toSampleCall(sample_call_json);
  assert(rawSample.request.length === 2);
  const parsedSample: ParsedCall = {
    request: [rawSample.request[0], JSON.parse(rawSample.request[1])],
    response: JSON.parse(rawSample.response),
  };
  return parsedSample;
}

describe("Switch", () => {
  let playSwitchSoundCall: ParsedCall;
  let setSoundOnCall: ParsedCall;
  let setSoundOffCall: ParsedCall;
  let unmatchedCalls: ParsedCall[];

  beforeAll(() => {
    playSwitchSoundCall = parseSampleCall(
      "../src-tauri/api/sample-calls/play_sound-switch.yaml",
    );
    setSoundOnCall = parseSampleCall(
      "../src-tauri/api/sample-calls/set_preferences-sound-on.yaml",
    );
    setSoundOffCall = parseSampleCall(
      "../src-tauri/api/sample-calls/set_preferences-sound-off.yaml",
    );
  });

  beforeEach(() => {
    tauriInvokeMock.mockImplementation(
      (...args: (string | Record<string, string>)[]) => {
        const jsonArgs = JSON.stringify(args);
        const matchingCallIndex = unmatchedCalls.findIndex(
          (call) => JSON.stringify(call.request) === jsonArgs,
        );
        assert(
          matchingCallIndex !== -1,
          `No matching call found for ${jsonArgs}`,
        );
        const matchingCall = unmatchedCalls[matchingCallIndex].response;
        unmatchedCalls.splice(matchingCallIndex, 1);
        return Promise.resolve(matchingCall);
      },
    );
  });

  test("can toggle sound on and off while saving setting", async () => {
    render(Settings, {});
    expect(get(soundOn)).toBe(true);
    expect(tauriInvokeMock).not.toHaveBeenCalled();

    const soundSwitch = screen.getByLabelText("Sounds");
    unmatchedCalls = [setSoundOffCall];
    await act(() => userEvent.click(soundSwitch));
    expect(get(soundOn)).toBe(false);
    expect(tauriInvokeMock).toBeCalledTimes(1);
    expect(unmatchedCalls.length).toBe(0);

    unmatchedCalls = [setSoundOnCall, playSwitchSoundCall];
    await act(() => userEvent.click(soundSwitch));
    expect(get(soundOn)).toBe(true);
    expect(tauriInvokeMock).toBeCalledTimes(3);
    expect(unmatchedCalls.length).toBe(0);
  });
});

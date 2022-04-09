import { assign, Interpreter, Machine, State as XStateEvent } from "xstate";

type Context = { error: Error | undefined };

type Event = { type: "SET_ERROR"; value: Error } | { type: "RESET" };

type State = {
  states: { ok: Record<string, unknown>; error: Record<string, unknown> };
};

export const errorMachine = Machine<Context, State, Event>({
  id: "errorMachine",
  initial: "ok",
  context: {
    error: undefined,
  },
  states: {
    ok: {
      on: {
        SET_ERROR: {
          actions: assign({ error: (_, event) => event.value }),
          target: "error",
        },
      },
    },
    error: {
      on: {
        RESET: {
          actions: assign({ error: (_) => undefined }),
          target: "ok",
        },
      },
    },
  },
});

export type ErrorMachine = [
  XStateEvent<Context, Event>,
  Interpreter<Context, State, Event>["send"],
  Interpreter<Context, State, Event>
];

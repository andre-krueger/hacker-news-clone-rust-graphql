import { createContext, useContext } from "react";
import { ErrorMachine } from "./ErrorMachine";

export type Machines = {
  errorMachine: ErrorMachine;
};

export const MachinesContext = createContext<Machines | null>(null);

export const MachinesContextProvider = MachinesContext.Provider;

export const useMachine = (): Machines =>
  useContext(MachinesContext) as Machines;

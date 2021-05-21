export function checkExhaustiveness(stmt: never): void {
  throw Error(`${stmt as string} unhandled`);
}

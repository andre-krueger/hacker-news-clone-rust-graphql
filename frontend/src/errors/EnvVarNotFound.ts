class EnvVarNotFound extends Error {
  constructor(message: string) {
    super(message);
  }
}

export default EnvVarNotFound;

module.exports = {
  root: true,
  ignorePatterns: ["__generated__", "node_modules", "scripts"],
  parser: "@typescript-eslint/parser",
  env: { node: true },
  parserOptions: {
    tsconfigRootDir: __dirname,
    project: ["./tsconfig.json"],
  },
  plugins: ["@typescript-eslint", "prettier"],
  extends: [
    "eslint:recommended",
    "plugin:@typescript-eslint/recommended",
    "plugin:@typescript-eslint/recommended-requiring-type-checking",
    "plugin:react/recommended",
    "plugin:react-hooks/recommended",
    "plugin:prettier/recommended",
  ],
  rules: {
    "prettier/prettier": "error",
    "no-restricted-imports": [
      "error",
      {
        paths: [
          { name: "graphql", importNames: ["graphql"] },
          {
            name: "react-native",
            importNames: ["View", "Text"],
          },
        ],
      },
    ],
  },
};

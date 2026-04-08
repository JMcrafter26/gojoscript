# goboscript Guidelines

This repository contains the goboscript compiler (Rust) and editor extensions (TypeScript).

**Critical Note:** According to [docs/contributing.md](docs/contributing.md), **no LLM-generated code will be accepted** in this project. Copilot should primarily be used for code exploration, debugging, explanation, and understanding the architecture rather than authoring pull requests.

## Code Style
- **Rust**: Uses standard `cargo +nightly fmt`.
- **TypeScript**: Follows ESLint/Prettier setup in `editors/code/`.
- **Commits**: Follows conventional commits.

## Architecture
- **Compiler**: Rust-based CLI and library. Key phases include Lexer (`logos`), Parser (`LALRPOP`), AST transformations (Visitor pattern), and Codegen (Scratch `.sb3` generation). Also supports WASM targets for the web IDE.
- **VS Code Extension**: Located in `editors/code/`. Wraps the compiler to provide language server features. 
- **Tooling**: Python scripts in `tools/` test and validate generated Scratch code against official JSON schemas.

## Build and Test
- **Rust Compiler**: Build with `cargo build` or `cargo build --release`. (Uses nightly toolchain).
- **VS Code Extension**: Navigate to `editors/code/` and run `npm run build` or `npm run watch`.
- **Testing & Validation**: Use `tools/run.py --validate tests/*` to generate projects and validate schemas.
- For testing `.sb3` internals, use `tools/sb3.py` to extract and repatch `project.json` for debugging structural errors. See [docs/contributing.md](docs/contributing.md) for detailed commands.

## Conventions
- **Link, don't embed**: Refer to [docs/contributing.md](docs/contributing.md) for environment setup and `.sb3` validation workflows. See [docs/language/](docs/language/) for goboscript language specifications and [docs/standard-library.md](docs/standard-library.md) for standard library features.
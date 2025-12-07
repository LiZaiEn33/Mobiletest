# Testing this repo in GitHub Codespaces (iPad-friendly)

Quick steps to try Codespaces on an iPad:

- On GitHub (mobile browser or app): open this repository, press the green `Code` button, then choose `Codespaces` → `Create codespace` on `main`.
- Or open the repository in Safari and append `/codespaces/new` to the repo URL.
- Codespaces will use the `.devcontainer/devcontainer.json` image which includes Rust and runs `cargo test` after setup.

Tips for iPad usage:
- Use Safari (recommended) or a desktop-class browser; Codespaces runs in the cloud so the device just needs a capable browser.
- For better editing, use `Open in VS Code` (if you have the GitHub Codespaces app or VS Code for Web) or use the VS Code mobile app that supports server connections.
- Ensure your GitHub account has Codespaces access (some accounts require a plan or organization enablement).

Local testing (if you have Rust locally):

```bash
cargo test
```

Files added:

- `Cargo.toml` — project manifest
- `src/lib.rs` and `src/main.rs` — example code and unit test
- `.devcontainer/devcontainer.json` — Codespaces/devcontainer configuration
- `.vscode/tasks.json` — convenient tasks for build/test

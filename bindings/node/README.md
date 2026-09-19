# Node foundation

Build from the repository root with `cargo build -p cerebri-node`.
Import `plan` from `bindings/node/index.mjs`, then `await plan(request)`.
The asynchronous JavaScript wrapper starts the Rust process bridge; planning runs only in cerebri-core.

The default binary is target/debug/cerebri-node-bridge(.exe). Applications can pass
`{ binary: absolutePath }` as the second argument for their packaged binary.
This is a provisional process bridge, not a native N-API addon. Native ABI selection,
Electron packaging, cancellation and production distribution remain the integration milestone.

Verify with `node --test bindings/node/test.mjs`.
[Interface reference](../../docs/en/cpir.md)


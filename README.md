# `yap` - proof-of-concept chat program

This is an updated MVP of `yap_client` (2021) and `yap_server` (2021) in order for them to build again.

For historical reasons I've preserved the jank, excessive nesting, most compiler warnings, and poor cryptography.

**New code's quality is an approximation of the original version's.**

To quickly test everything, see `demo/`

## Updates

Modernization:
- Update dependencies
- Use `unfold` and `merge` instead of `tokio::select`
- Fix calls to obsolete APIs
- Spawn async tasks with `Handle` and `JoinSet`

Logistics:
- Create demo using containers
- Reorganize modules
- Greatly reduce imports indirection
- Use cargo workspace
- Consolidate common code to module
- Dockerfiles to prevent a repeat of build failures
- Visibility fixes

Other:
- Use `anyhow` for error handling
- Replace `structopt` with `clap`
- Minor event loop robustness fixes
- Server now sends WebSocket address to client upon login

## No AI assistance

No AI assistance has been used in this project.

## Archive

Old nonfunctional repos with untouched histories can be found in the submodules, branch `old`.

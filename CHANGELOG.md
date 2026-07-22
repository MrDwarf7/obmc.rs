# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

### Bug Fixes

- use actions/checkout@v7 directly (centralized action can only run after checkout) - ([20193e8](https://github.com/MrDwarf7/obmc.rs/commit/20193e8fd61363b3307bdafabac5467355a61be2))
- sync draft.yml with latest template, update setup-rust to checkout@v7 - ([0c769da](https://github.com/MrDwarf7/obmc.rs/commit/0c769da9aa0ef2da072705819bebd1613e4d5e79))

### Documentation

- README banners use assets/ header + icon - ([a330e3c](https://github.com/MrDwarf7/obmc.rs/commit/a330e3c65d9eb273beea706424e624a4fe4ccb65))
- align README with rust_template conventions; drop duplicate rustfmt.toml - ([3933489](https://github.com/MrDwarf7/obmc.rs/commit/393348944989479766f0e023a9c4d06837e28149))

### Features

- assets/ icons + Windows build.rs (winresource) - ([0e8f98c](https://github.com/MrDwarf7/obmc.rs/commit/0e8f98cd248f1a2390dc8abc591b26ace49ad325))
- apply rust_template conventions (workflows, configs, README) - ([94d5ffa](https://github.com/MrDwarf7/obmc.rs/commit/94d5ffa3a50b3d4356c8c41a99f11483e219ba5f))

### Miscellaneous Tasks

- publish promotes draft via gh API (drop svenstaro force path) - ([d115190](https://github.com/MrDwarf7/obmc.rs/commit/d1151908bd01d6ad22120ab798dfb28a8f3950c7))
- sync workflows to latest rust_template (publish.yml, no force-push nightly) - ([029bdd1](https://github.com/MrDwarf7/obmc.rs/commit/029bdd1dfa3508014a322e2ae5564eacc1fe3820))
- add .gitattributes, centralized checkout action - ([5bd8f5f](https://github.com/MrDwarf7/obmc.rs/commit/5bd8f5f1557edd23dd383c9eb88bbd9c5ad07c26))
- sync workflows with latest rust_template (checkout@v7 centralized) - ([251cad9](https://github.com/MrDwarf7/obmc.rs/commit/251cad92af2951e0fd7242912442e2e2efd7b8a0))
- sync workflows & setup-rust action from rust_template, enable mold - ([6e184e8](https://github.com/MrDwarf7/obmc.rs/commit/6e184e81488a2b396b380dcb672298734b528ff7))
- align with rust_template conventions (README, setup scripts, build/) - ([6e22d3f](https://github.com/MrDwarf7/obmc.rs/commit/6e22d3f688726dd33b007b6624614728eb4e8d53))
- sync .github/actions + workflows from rust_template, refresh build/install.sh for obmc - ([a25ded0](https://github.com/MrDwarf7/obmc.rs/commit/a25ded05c01b81c20292af141205b55e8527b075))
- add cliff.toml and deny.toml, fix repo URL in cliff - ([6cb642d](https://github.com/MrDwarf7/obmc.rs/commit/6cb642d89cc1a8d402a18378a17b90a94973ce2a))

### Styling

- cargo fmt - ([44d1cff](https://github.com/MrDwarf7/obmc.rs/commit/44d1cffb468dcc1095b364ed84356b05e0dcdb8e))



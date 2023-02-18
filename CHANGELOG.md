# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## v0.19.0 - 2023-02-18
#### Bug Fixes
- CD Secret - (129aecb) - Harry Bairstow
- always fetch HEAD when building container - (24af511) - Harry Bairstow
#### Features
- new dashboard (#104) - (965b554) - Harry Bairstow
#### Miscellaneous Chores
- Tidy up Echo Server, closing small issues (#105) - (869d7a1) - Harry Bairstow
- - -

## v0.18.0 - 2023-02-16
#### Bug Fixes
- tenant url (#102) - (2200ce4) - Harry Bairstow
#### Features
- Support p8 certificates (#100) - (dba344c) - Harry Bairstow
- More Metrics (#103) - (2e46761) - Harry Bairstow
- - -

## v0.17.3 - 2023-02-07
#### Bug Fixes
- lowercase string conversions into ProviderKind (#95) - (0cf66a0) - WC
- Strip out decentralized identifier prefix from client_id (#96) - (89c7b77) - WC
#### Miscellaneous Chores
- ignore `.github` from `ci` - (baa395b) - Harry Bairstow
- revert pipeline changes - (f58a2a2) - Harry Bairstow
- - -

## v0.17.2 - 2023-02-07
#### Bug Fixes
- **(hotfix)** cors not properly configured (#99) - (eecd5ee) - Derek
- - -

## v0.17.1 - 2023-02-06
#### Bug Fixes
- **(hotfix)** dedupe messages (#97) - (c12ea58) - Derek
- - -

## v0.17.0 - 2023-02-03
#### Bug Fixes
- clippy - (5510f23) - Derek
- CI workflow broken - (df9b643) - Derek
- apple push notifications not showing - (86ae566) - Derek
#### Features
- improve pipelines - (cf726a0) - Derek
- - -

## v0.16.0 - 2023-02-01
#### Features
- Contributor Guide (#93) - (2023792) - Harry Bairstow
#### Miscellaneous Chores
- update labels in `intake.yml` - (c994c32) - Harry Bairstow
- - -

## v0.15.0 - 2023-01-31
#### Features
- CORS (#88) - (18f49a4) - Harry Bairstow
- - -

## v0.14.2 - 2023-01-30
#### Bug Fixes
- broken query - (07672e5) - Harry Bairstow
- - -

## v0.14.1 - 2023-01-30
#### Bug Fixes
- duplicate health endpoint - (0312b2b) - Harry Bairstow
- missing env var - (f390f4c) - Harry Bairstow
- - -

## v0.14.0 - 2023-01-29
#### Features
- migrate `bat-cave` to `echo-server` (#86) - (1d24db7) - Harry Bairstow
- use `bat-cave` secret for tenant db - (884da1e) - Harry Bairstow
#### Miscellaneous Chores
- bump lockfile - (ccd4490) - Harry Bairstow
- - -

## v0.13.1 - 2023-01-27
#### Bug Fixes
- patch queries - (e559394) - Harry Bairstow
- - -

## v0.13.0 - 2023-01-20
#### Features
- Disable sig validation option (#83) - (1a94f36) - Harry Bairstow
- Remove Flags and Flattern Payload (#82) - (eab8def) - Harry Bairstow
#### Miscellaneous Chores
- switch from `debug` to `info` - (09207b8) - Harry Bairstow
- update otel command - (377c3b8) - Harry Bairstow
- - -

## v0.12.0 - 2023-01-13
#### Bug Fixes
- ci/cd - (9ecf44a) - Harry Bairstow
- not all errors tracked - (a82cd14) - Derek
#### Features
- Rework Logging, Metrics and Traces (#76) - (b40bfcb) - Harry Bairstow
- define alert - (caf80c2) - Derek
- Use tini as init in echo-server container (#77) - (963a291) - WC
- upgrade limits (#73) - (a0afc94) - Derek
- - -

## v0.11.5 - 2022-12-23
#### Bug Fixes
- Incorrect version used for container - (da338b0) - Harry Bairstow
- - -

## v0.11.4 - 2022-12-22
#### Bug Fixes
- ghcr `403`'s - (ea61d2f) - Harry Bairstow
- - -

## v0.11.3 - 2022-12-22
#### Bug Fixes
- ghcr auth in actions - (13b8aa1) - Harry Bairstow
- - -

## v0.11.2 - 2022-12-22
#### Bug Fixes
- publish container to ghcr - (f8110e6) - Harry Bairstow
- - -

## v0.11.1 - 2022-12-22
#### Bug Fixes
- Doesn't build containers - (f769ac6) - Harry Bairstow
- - -

## v0.11.0 - 2022-12-22
#### Bug Fixes
- **(cd)** cannot invoke validate from CD - (4943390) - Derek
- **(cd)** syntax error in workflow - (e797cc7) - Derek
- Pipelines (#60) - (01589cf) - Harry Bairstow
#### Features
- **(o11y)** add http metrics - (d4037ce) - Derek
- E2EE (#72) - (05f70a1) - Harry Bairstow
- allow to manually kick off CD - (e690245) - Derek
- add basic dashboard (#67) - (22851b8) - Derek
- Migrate to E2EE webhooks (#70) - (611f14c) - Harry Bairstow
- add validate workflow (#62) - (87870fa) - Derek
- Monitoring (#59) - (59ce744) - Harry Bairstow
- - -

## v0.10.2 - 2022-12-07
#### Bug Fixes
- Missing upserts for clients store - (a75f955) - Harry Bairstow
- - -

## v0.10.1 - 2022-12-07
#### Bug Fixes
- Query error - (4be9363) - Harry Bairstow
- Incorrect Prometheus endpoint - (45b7a26) - Harry Bairstow
- - -

## v0.10.0 - 2022-12-07
#### Features
- Metrics (#58) - (9b3781b) - Harry Bairstow
- - -

## v0.9.0 - 2022-12-07
#### Bug Fixes
- cannot register clients (#56) - (f2ff6ad) - Harry Bairstow
#### Features
- allow updating device token (#35) - (dbadfe1) - Derek
- implement integration tests (#55) - (c4e4a37) - Derek
- - -

## v0.8.0 - 2022-12-02
#### Features
- cover registration logic (#53) - (df88052) - Derek
- Improve Terraform (#46) - (3f91f09) - Harry Bairstow
- Tenant CRUD (#54) - (e0b33bb) - Harry Bairstow
#### Miscellaneous Chores
- add `cog.toml` - (87f9926) - Harry Bairstow
- Make Properties Public - (f01a7b6) - Harry Bairstow
- - -

## v0.7.0 - 2022-11-26
#### Bug Fixes
- All APNS Config required - (9e518f9) - Harry Bairstow
- tests not compiling - (b75d1db) - Derek
- fmt - (a3dbe3d) - Derek
- Upload both Cargo files from `release` action - (d2a9c68) - Harry Bairstow
- CI/CD (#45) - (a7b364e) - Harry Bairstow
#### Features
- **(ci)** use larger runners in CI (#51) - (5929f5c) - Derek
- **(tests)** implement initial functional test (#50) - (c1c3653) - Derek
- Bump Axum out of `rc` - (f1a6395) - Harry Bairstow
- refactor for functional test (#49) - (75a4cc1) - Derek
#### Miscellaneous Chores
- **(cargo)** Bump `build-info` - (9842abb) - Harry Bairstow
- **(cargo)** Remove minor versions - (7c95434) - Harry Bairstow
- **(fmt)** `rustfmt` config and run formatter - (0cc04b6) - Harry Bairstow
- - -

## v0.6.0 - 2022-11-08
#### Features
- Initial Multi-Tenant Work (#40) - (cb66818) - Harry Bairstow
- - -

## v0.5.3 - 2022-10-26
#### Bug Fixes
- Remove reference to `topic` with `clone()` - (775f09d) - Harry Bairstow
- APNS Topic (#39) - (e1fffc1) - Harry Bairstow
- - -

## v0.5.2 - 2022-10-26
#### Bug Fixes
- Check client before notification insert - (5df746b) - Harry Bairstow
#### Miscellaneous Chores
- Bump lockfile - (70fc233) - Harry Bairstow
- - -

## v0.5.1 - 2022-10-26
#### Bug Fixes
- SQL (#38) - (2bdd074) - Harry Bairstow
- - -

## v0.5.0 - 2022-10-26
#### Features
- Better Errors (#37) - (c031272) - Harry Bairstow
- Logging (#36) - (a6eeb33) - Harry Bairstow
- - -

## v0.4.0 - 2022-10-20
#### Bug Fixes
- Env name wrong - (7eb151e) - Harry Bairstow
- cd use as string instead of command - (eb34282) - Harry Bairstow
- CD use as var instead of command - (c06ef38) - Harry Bairstow
- Remove `cpu_architecture` - (5341d17) - Harry Bairstow
- Add missing job requirement - (db9ba2c) - Harry Bairstow
#### Features
- Change URL (#33) - (3a65ff3) - Harry Bairstow
#### Miscellaneous Chores
- Create missing file - (208e8e6) - Harry Bairstow
- - -

## v0.3.1 - 2022-10-20
#### Bug Fixes
- Typo in depedencies - (27e13be) - Harry Bairstow
- - -

## v0.3.0 - 2022-10-20
#### Bug Fixes
- Rename - (09a3194) - Harry Bairstow
- Use PAT - (3dcf431) - Harry Bairstow
#### Features
- `cog.toml` - (aa7436f) - Harry Bairstow
#### Miscellaneous Chores
- Remove `v` from release - (4dbf420) - Harry Bairstow
- Bump `Cargo.lock` - (baa0d17) - Harry Bairstow
- Delete changelog - (787402e) - Harry Bairstow
- Only push `*.toml` - (9e68784) - Harry Bairstow
- Merge local - (5b395b4) - Harry Bairstow
- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).
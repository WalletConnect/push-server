# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## v0.41.7 - 2024-07-10
#### Bug Fixes
- **(apns)** proper catching of `Unknown CA` error (#345) - (cee0298) - Max Kalashnikoff | maksy.eth

- - -

## v0.41.6 - 2024-06-21
#### Bug Fixes
- **(apns)** proper handling of the wrong CA in APNS certificate (#343) - (fc38254) - Max Kalashnikoff | maksy.eth
- **(clippy)** fixing updated clippy dead code warnings (#344) - (5d201d8) - Max Kalashnikoff | maksy.eth

- - -

## v0.41.5 - 2024-06-05
#### Bug Fixes
- handle FCM v1 token unregistration & forbidden & Apns(TopicDisallowed) (#342) - (2f0c70a) - Chris Smith
- concurrent client registration errors (#341) - (85b9997) - Chris Smith

- - -

## v0.41.4 - 2024-05-29
#### Bug Fixes
- increasing rate-limiting threshold (#339) - (47fa3b1) - Max Kalashnikoff

- - -

## v0.41.3 - 2024-05-21
#### Bug Fixes
- improve create client performance (#337) - (0481f50) - Chris Smith
- notification_channels syntax - (7c2fb36) - Chris Smith
- submodule checkout - (81e5f64) - Chris Smith
- TF var syntax & metric names - (8e125b1) - Chris Smith

- - -

## v0.41.2 - 2024-05-20
#### Bug Fixes
- Postgres query metrics (#335) - (4d687a7) - Chris Smith

- - -

## v0.41.1 - 2024-05-20
#### Bug Fixes
- validate step (#336) - (845eede) - Chris Smith

- - -

## v0.41.0 - 2024-05-20
#### Bug Fixes
- upgrade otel (#334) - (7086406) - Chris Smith
#### Features
- delete provider (#329) - (66b31e2) - Chris Smith

- - -

## v0.40.2 - 2024-05-17
#### Bug Fixes
- client registrations chart (#332) - (a74ad70) - Chris Smith
- rate limit 10 req/min (#331) - (1c3a860) - Chris Smith

- - -

## v0.40.1 - 2024-05-16
#### Bug Fixes
- applying rate limiting middleware to all except push endpoints (#328) - (532a9bc) - Max Kalashnikoff

- - -

## v0.40.0 - 2024-05-16
#### Bug Fixes
- push payloads (#326) - (1abce43) - Chris Smith
#### Features
- per-IP rate limiting (#327) - (70e3002) - Max Kalashnikoff
#### Miscellaneous Chores
- refactor validate JWT (#325) - (c31b519) - Chris Smith

- - -

## v0.39.0 - 2024-05-15
#### Features
- FCM v1 (#316) - (2c2f09a) - Chris Smith
#### Miscellaneous Chores
- bump a2 (#320) - (4701eb7) - Chris Smith
- bump a2 (#319) - (09512dd) - Chris Smith

- - -

## v0.38.2 - 2024-04-26
#### Bug Fixes
- upgrde to axum 0.7 (#315) - (2a493ca) - Chris Smith

- - -

## v0.38.1 - 2024-04-22
#### Bug Fixes
- separate staging and prod JWT secrets (#318) - (30b172e) - Chris Smith

- - -

## v0.38.0 - 2024-04-17
#### Features
- refactored cloud auth (#317) - (830ad97) - Chris Smith

- - -

## v0.37.6 - 2024-03-25
#### Bug Fixes
- APNs certificate expired error (#314) - (d5347fa) - Chris Smith
- cannot downgrade RDS version - (168ec33) - Chris Smith
- increasing PgPool connections to `100` (#310) - (044b51d) - Max Kalashnikoff
- downgrade log level (#312) - (c0fb3a5) - Chris Smith
#### Miscellaneous Chores
- downgrade runner (#311) - (969252a) - Chris Smith

- - -

## v0.37.5 - 2024-03-13
#### Bug Fixes
- increasing Postgres connection pool size (#309) - (3429e23) - Max Kalashnikoff

- - -

## v0.37.4 - 2024-03-06
#### Bug Fixes
- scale down (#307) - (05662e3) - Chris Smith
- reduce info logs (#306) - (124098a) - Chris Smith

- - -

## v0.37.3 - 2024-02-13
#### Bug Fixes
- **(alarms)** increasing 5xx alarm threshold from 5 to 15 (#303) - (8b25665) - Max Kalashnikoff

- - -

## v0.37.2 - 2024-02-01
#### Bug Fixes
- removing `update_tenant` function (#299) - (bb76d86) - Max Kalashnikoff
- tenants update on conflict handling (#298) - (994ec79) - Max Kalashnikoff

- - -

## v0.37.1 - 2024-01-25
#### Bug Fixes
- using client id `pg_advisory_xact_lock` lock when inserting notification (#296) - (a0793d0) - Max Kalashnikoff
- adding the pg_lock for the token (#293) - (d77fa50) - Max Kalashnikoff

- - -

## v0.37.0 - 2023-11-29
#### Bug Fixes
- log client errors (#289) - (b2acd19) - Chris Smith
#### Features
- **(o11y)** makes the alarm less noisy (#287) - (ccb9a2e) - Derek
#### Miscellaneous Chores
- **(logging)** improve logging with instrument and request-id span (#290) - (21ec1ed) - Max Kalashnikoff
- removing `/info` endpoint in a favor of `/health` (#291) - (b0d2e78) - Max Kalashnikoff

- - -

## v0.36.0 - 2023-11-21
#### Bug Fixes
- **(o11y)** registered client metric is wrong (#285) - (c442c03) - Derek
- alarm immediately (#280) - (4b34fa8) - Chris Smith
- idempotency is per-client (#275) - (1905c8b) - Chris Smith
#### Features
- **(decrypted_notify)** update message payload and pass raw message (#281) - (3990b70) - Max Kalashnikoff
- **(decrypted_notify)** adding `always_raw` for the client registration (#279) - (16a377a) - Max Kalashnikoff
#### Miscellaneous Chores
- Revert "chore: style sql (#277)" - (ea1401e) - Chris Smith
- style sql (#277) - (70dad11) - Chris Smith
- fix all warnings (#276) - (f3c5dd6) - Chris Smith

- - -

## v0.35.2 - 2023-11-08
#### Bug Fixes
- idempotency per client (#274) - (2002cba) - Chris Smith
#### Miscellaneous Chores
- remove linked issues check (#272) - (cb945d8) - Xavier Basty

- - -

## v0.35.1 - 2023-11-06
#### Bug Fixes
- adding jwt verification to the apns and fcm update handler (#261) - (2c454eb) - Max Kalashnikoff
- fixing and enabling ignored tests (#263) - (7bf555a) - Max Kalashnikoff
- remove `for` from evaluation and adding a threshold line (#260) - (a070782) - Max Kalashnikoff
- `notification` variable is encoded as a string, fix alert thresholds (#231) - (8028744) - Max Kalashnikoff
- revert to the base64 encoded p8 key (#253) - (0339004) - Max Kalashnikoff
- fixing apns keys encoding (#246) - (2e6a9fe) - Max Kalashnikoff
- spacing fix in the terraform main file (#243) - (9d37d66) - Max Kalashnikoff
- using relay public key from environment variable (#241) - (0934fd2) - Max Kalashnikoff
- use `axum-client-ip` to get the real client IP (#240) - (591effe) - Max Kalashnikoff
- add advisory locking to the client create (#239) - (46d1797) - Max Kalashnikoff
- adding `... FOR UPDATE` to lock the row while client creation (#235) - (1de1a69) - Max Kalashnikoff
#### Continuous Integration
- bump `update_rust_version` to 2.1.5 (#237) - (066bd18) - Max Kalashnikoff
#### Miscellaneous Chores
- update `utils` version (#269) - (5695fa3) - Xavier Basty
- remove Ukraine from list of OFAC blocked countries (#268) - (a8b467a) - Xavier Basty
- add Russia and Ukraine to list of OFAC blocked countries (#336) (#266) - (e7344b1) - Xavier Basty
- change no data state for 5xx monitoring (#259) - (190a1ed) - Max Kalashnikoff
- distinguish bad device token errors (#257) - (733c152) - Max Kalashnikoff
- fix tenant ID log (#255) - (3902a9c) - Chris Smith
- adding `for` and `message` for 5xx alerts (#258) - (3acf35a) - Max Kalashnikoff
- tap err (#252) - (bbdf178) - Chris Smith
- distinguish 500s from other errors (#248) - (b1135ce) - Chris Smith
- more logging (#250) - (45902c7) - Chris Smith
- adding logs to apns update (#245) - (80e3bf5) - Max Kalashnikoff
- deploy to production became optional with choice for the `image_tag` (#226) - (3b10917) - Max Kalashnikoff
#### Tests
- adding `client_create_same_id_and_token` test (#234) - (c7b14ff) - Max Kalashnikoff

- - -

## v0.35.0 - 2023-10-02
#### Features
- geo-blocking, replace `gorgon` with `utils-rs` (#227) - (40e6d51) - Xavier Basty
#### Miscellaneous Chores
- enabling alarm notifications (#219) - (00eaa3b) - Max Kalashnikoff
- fixing output variable name in the `release` step (#221) - (48a605d) - Max Kalashnikoff

- - -

## v0.34.7 - 2023-09-28
#### Bug Fixes
- don't wipe response_message (#216) - (37a5a31) - Chris Smith
- adding `id` for the updated release script (#214) - (e269a42) - Max Kalashnikoff
#### Miscellaneous Chores
- extra assert APNS response is non-error (#218) - (7bc4c8e) - Chris Smith

- - -

## v0.34.6 - 2023-09-26
#### Bug Fixes
- adding SQL migration for making `device_token` UNIQUE (#202) - (7b761b3) - Max Kalashnikoff
- updating device token deduplication SQL query in `create_client` (#213) - (d727723) - Max Kalashnikoff
- alter notifications constraint with `ON DELETE CASCADE` (#211) - (3553b2e) - Max Kalashnikoff
#### Miscellaneous Chores
- bumping version in `Cargo.lock` in release CI workflow (#200) - (2b83b51) - Max Kalashnikoff

- - -

## v0.34.5 - 2023-09-21
#### Bug Fixes
- noop - (2baf8da) - Chris Smith
#### Miscellaneous Chores
- revert 196 (#209) - (0a55521) - Chris Smith
- - -

## v0.34.4 - 2023-09-21
#### Bug Fixes
- enable functional tests in CI (#198) - (87ea075) - Max Kalashnikoff
#### Miscellaneous Chores
- log client database actions (#207) - (2180574) - Chris Smith
- fix byzantine failures on functional tests (#206) - (e0d8a39) - Max Kalashnikoff
- - -

## v0.34.3 - 2023-09-20
#### Bug Fixes
- removing of the deprecated S3 analytics bucket (#204) - (d85ee76) - Max Kalashnikoff
- duplicated push tokens (#196) - (f0820f6) - Max Kalashnikoff
- compilation errors with `functional_tests` flag (#195) - (b6c88f1) - Max Kalashnikoff
- use RELEASE_PAT (#192) - (95474d7) - Chris Smith
- - -

## v0.34.2 - 2023-09-07
#### Bug Fixes
- data only messages not showing in background (#188) - (94e225f) - Chris Smith
#### Miscellaneous Chores
- revert to info logs (#183) - (c370719) - Harry Bairstow
- - -

## v0.34.1 - 2023-08-30
#### Bug Fixes
- tidy comments - (7cd9d55) - Harry Bairstow
#### Miscellaneous Chores
- remove comment - (dda91f1) - Harry Bairstow
- move migration (#186) - (8a3c9b0) - Harry Bairstow
- - -

## v0.34.0 - 2023-08-30
#### Bug Fixes
- ci - (0beaa5c) - Harry Bairstow
- listener cert (#180) - (5dbe341) - Harry Bairstow
#### Features
- handle invalid topic for token (#182) - (89034b3) - Harry Bairstow
#### Miscellaneous Chores
- remove optional on topic (#185) - (54bdc73) - Harry Bairstow
- extra logs for FCM Errors - (6079231) - Harry Bairstow
- set content avaliable - (b7c019b) - Harry Bairstow
- amend previous - (fd369b3) - Harry Bairstow
- only log SQL statements when at trace - (e1153a9) - Harry Bairstow
- patch clippy workflow - (3e44b92) - Harry Bairstow
- - -

## v0.33.0 - 2023-08-11
#### Bug Fixes
- ensure tenant_id matches the tenant a client is registered with (#168) - (b9dd96e) - Harry Bairstow
- inherit secrets (#173) - (1441a9c) - Harry Bairstow
#### Features
- Suspend Broken Tenants & Delete Broken Clients (#177) - (a8e1aa7) - Harry Bairstow
- backup domain (#179) - (f51c7c1) - Harry Bairstow
- always include topic data for push (#175) - (7c3cd9b) - Harry Bairstow
- APNS Verification (#174) - (fac7d6a) - Harry Bairstow
- - -

## v0.32.0 - 2023-08-02
#### Bug Fixes
- incorrect tag value (#170) - (cbf55e8) - Harry Bairstow
#### Features
- Web3Inbox CORS Support (#172) - (b16b97d) - Harry Bairstow
- full/Improved pipeline (#160) - (39235fe) - Harry Bairstow
- fcm verification (#169) - (d829e18) - Harry Bairstow
#### Miscellaneous Chores
- change context value (#171) - (3680820) - Harry Bairstow
- re-enable debug logs (#167) - (219b1ec) - Harry Bairstow
- - -

## v0.31.2 - 2023-07-25
#### Bug Fixes
- catch panic (#166) - (32163ef) - Harry Bairstow
- - -

## v0.31.1 - 2023-07-20
#### Bug Fixes
- ensure all data provided (#164) - (6dc5949) - Harry Bairstow
- - -

## v0.31.0 - 2023-07-18
#### Features
- Response Analytics (#161) - (5698d83) - Harry Bairstow
- - -

## v0.30.0 - 2023-07-12
#### Features
- move analytics to proper s3 bucket (#159) - (88416fb) - Rakowskiii
- propagate tags to ECS tasks - (a21dce9) - Derek
- - -

## v0.29.0 - 2023-07-03
#### Features
- add `msg_id` to analytics exports (#156) - (0be5086) - Xavier Basty
- - -

## v0.28.2 - 2023-07-03
#### Bug Fixes
- **(cors-headers)** allow authorization and content-type headers (#155) - (0a6fbfe) - Cali
- - -

## v0.28.1 - 2023-06-29
#### Bug Fixes
- bucket prefix (#153) - (ed139ab) - Harry Bairstow
- - -

## v0.28.0 - 2023-06-27
#### Bug Fixes
- return correct error code (#148) - (baf52ac) - Harry Bairstow
#### Features
- improve default message (#151) - (42b8bbe) - Derek
- - -

## v0.27.0 - 2023-06-22
#### Bug Fixes
- **(hotfix)** build info unknown (#145) - (8c12c57) - Derek
#### Features
- use latest otel collector image (#144) - (4bd8ea7) - Derek
- - -

## v0.26.0 - 2023-06-22
#### Features
- bump parquet and fix builds (#143) - (59df9a8) - Harry Bairstow
- - -

## v0.25.1 - 2023-06-20
#### Bug Fixes
- docker images now build - (d65eb3b) - Harry Bairstow
- - -

## v0.25.0 - 2023-06-20
#### Bug Fixes
- **(o11y)** received notifications metric broken (#131) - (bb1e0f4) - Derek
- reformat file - (090ca71) - Harry Bairstow
- Ensure CI/Release actions and container building works (#136) - (5d1a864) - Harry Bairstow
- remove redundant project actions (#138) - (90a3a7e) - Xavier Basty
#### Features
- Better error responses (#139) - (c4ce93c) - Harry Bairstow
- add project issues workflow, update project id (#129) - (d0559a2) - Xavier Basty
- update Grafana notification channel - (c5e2713) - Derek
- upgrade to Grafana 9 - (8f0ac06) - Derek
- authenticate JWTs for tenant endpoints (#123) - (e94d85b) - Harry Bairstow
- - -

## v0.24.1 - 2023-04-26
#### Bug Fixes
- have `/health` and `/info` endpoints (#125) - (0fb0804) - Harry Bairstow
- - -

## v0.24.0 - 2023-04-24
#### Features
- `project_id` validation (#122) - (f4cf08d) - Harry Bairstow
- logging (#119) - (aee046c) - Harry Bairstow
- - -

## v0.23.6 - 2023-04-22
#### Bug Fixes
- incorrect `cfg` usage (#118) - (9a2d0e9) - Harry Bairstow
- - -

## v0.23.5 - 2023-04-20
#### Bug Fixes
- docker features in cook command - (c1eb66e) - Harry Bairstow
#### Miscellaneous Chores
- **(docker)** features in cook command - (dca5e72) - Harry Bairstow
- merge - (3c743ef) - Harry Bairstow
- - -

## v0.23.4 - 2023-04-20
#### Bug Fixes
- Dockerfile - (332c1c1) - Harry Bairstow
- missing inputs for environment in terraform - (f038502) - Harry Bairstow
- CD action broken with terraform cloud - (48a9a4b) - Harry Bairstow
#### Miscellaneous Chores
- revert engine version change - (86ec9cb) - Harry Bairstow
- CD action typo in name - (a210140) - Harry Bairstow
- - -

## v0.23.3 - 2023-04-20
#### Bug Fixes
- remove incorrect `cfg` - (0cfeedb) - Harry Bairstow
- - -

## v0.23.2 - 2023-04-20
#### Bug Fixes
- copy crates to allow build - (6ec1ca0) - Harry Bairstow
- - -

## v0.23.1 - 2023-04-20
#### Bug Fixes
- images don't copy crates - (92871a2) - Harry Bairstow
- - -

## v0.23.0 - 2023-04-20
#### Bug Fixes
- image suffixes - (22850b3) - Harry Bairstow
- image suffixes - (084c8ac) - Harry Bairstow
#### Features
- proper suffix for images - (c049e3c) - Harry Bairstow
- - -

## v0.22.0 - 2023-04-20
#### Bug Fixes
- invalid workflow format - (c079248) - Harry Bairstow
- invalid workflow format - (7b66fd8) - Harry Bairstow
- invalid workflow format - (ab33159) - Harry Bairstow
- Update ed25519-dalek to 2.0.0-rc.2 and resolve breaking changes (#111) - (0a5f45c) - WC
#### Features
- add terraform cloud support to actions (#117) - (964bfb7) - Harry Bairstow
- more tests & tidy existing tests (#108) - (1ae2be4) - Harry Bairstow
- migrate multitenant and analytics to features (#116) - (7f77003) - Harry Bairstow
- Monitoring (#115) - (85b8c48) - Harry Bairstow
#### Miscellaneous Chores
- format terraform - (f8bf800) - Harry Bairstow
- migrate state - (fea7edc) - Harry Bairstow
- - -

## v0.21.0 - 2023-03-17
#### Features
- Analytics (#114) - (2e7c538) - Harry Bairstow
- added tracing of JWT claims verification failure (#113) - (83ac3bb) - Rakowskiii
- - -

## v0.20.0 - 2023-03-16
#### Features
- `client_id` auth (#110) - (e085617) - Harry Bairstow
- - -

## v0.19.4 - 2023-03-01
#### Bug Fixes
- hotfix hyper-apln  (#112) - (9e5b40a) - Rakowskiii
- FCM counter name - (944fd93) - Harry Bairstow
- - -

## v0.19.3 - 2023-02-20
#### Bug Fixes
- APNS Store Logic & Tests (#107) - (4e1575b) - Harry Bairstow
- - -

## v0.19.2 - 2023-02-18
#### Bug Fixes
- cast `apns_type` - (7233111) - Harry Bairstow
- - -

## v0.19.1 - 2023-02-18
#### Bug Fixes
- APNS type not set (#106) - (87aa4f0) - Harry Bairstow
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
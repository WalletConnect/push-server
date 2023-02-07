# Release Guide

This doc explains the Echo Server pipelines and how to create a release.

> **Warning**
> All commits **Must** follow the [conventional commit](https://www.conventionalcommits.org/en/v1.0.0/) 
> specification for the pipeline to continue working, if a commit ever doesn't follow this then git 
> history may be to be re-written!

# Creating a new Release

If you decide that we need to push the latest changes from `main` to our servers and as a release for
downstream dependents, follow these steps:
- Go to the actions tab
- Select the `release` action from the sidebar
- Click run workflow

This will then run the [Release](#release) workflow which in turn triggers [CD](#cd) so that changes are
deployed to our servers

# Workflows

## Intake

Adds `S-accepted` to issues opened by the WalletConnect core team and adds them to our boards

## Validate

Runs integration tests against a specific environment - typically called by the [CD](#cd) action but
can be run manually using `workflow_dispatch`

## CI Terraform

Checks Terraform formatting and then runs plan, sends the plan as a comment on the PR so that reviewers can
more clearly see what this PR changes with the infra

## CI

Runs:
- `cargo clippy`
- `cargo +nightly fmt`
- `cargo test`

To check that all formatting is correct, clippy has be respected and that unit/integration tests are still
passing

## Release

Generates a changelog, bumps the version in the `cargo.lock` file, commits it and creates a new release.

The new release triggers [CD](#cd), and while that starts we create docker containers and publish them to:
- Internal ECR
- `ghcr.io/walletconnect/echo-server`

# CD

Deploy changes to Staging infrastructure, then runs the [Validations](#validate), if they succeed we then
deploy the same changes to Production
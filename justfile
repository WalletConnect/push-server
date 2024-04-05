lint: clippy fmt

unit: lint test test-all test-single-tenant lint-tf

devloop: unit fmt-imports

test := ""

test:
  RUST_BACKTRACE=1 cargo test --lib --bins -- {{test}}

test-all:
  RUST_BACKTRACE=1 cargo test --all-features --lib --bins -- {{test}}

test-single-tenant:
  RUST_BACKTRACE=1 cargo test --features=functional_tests -- {{test}}

clippy:
  #!/bin/bash
  set -euo pipefail

  if command -v cargo-clippy >/dev/null; then
    echo '==> Running clippy'
    cargo clippy --all-features --tests -- -D warnings
  else
    echo '==> clippy not found in PATH, skipping'
  fi

fmt:
  #!/bin/bash
  set -euo pipefail

  if command -v cargo-fmt >/dev/null; then
    echo '==> Running rustfmt'
    cargo fmt
  else
    echo '==> rustfmt not found in PATH, skipping'
  fi

  if command -v terraform -version >/dev/null; then
    echo '==> Running terraform fmt'
    terraform -chdir=terraform fmt -recursive
  else
    echo '==> terraform not found in PATH, skipping'
  fi

fmt-imports:
  #!/bin/bash
  set -euo pipefail

  if command -v cargo-fmt >/dev/null; then
    echo '==> Running rustfmt'
    cargo +nightly fmt -- --config group_imports=StdExternalCrate,imports_granularity=One
  else
    echo '==> rustfmt not found in PATH, skipping'
  fi

lint-tf: tf-validate tf-fmt tfsec tflint tfdocs

tf-fmt:
  #!/bin/bash
  set -euo pipefail

  if command -v terraform >/dev/null; then
    echo '==> Running terraform fmt'
    terraform -chdir=terraform fmt -recursive
  else
    echo '==> Terraform not found in PATH, skipping'
  fi

tf-validate:
  #!/bin/bash
  set -euo pipefail

  if command -v terraform >/dev/null; then
    echo '==> Running terraform fmt'
    terraform -chdir=terraform validate
  else
    echo '==> Terraform not found in PATH, skipping'
  fi

tfsec:
  #!/bin/bash
  set -euo pipefail

  if command -v tfsec >/dev/null; then
    echo '==> Running tfsec'
    cd terraform
    tfsec
  else
    echo '==> tfsec not found in PATH, skipping'
  fi

tflint:
  #!/bin/bash
  set -euo pipefail

  if command -v tflint >/dev/null; then
    echo '==> Running tflint'
    cd terraform; tflint
    cd ecs; tflint
    cd ../monitoring; tflint
    cd ../private_zone; tflint
    cd ../redis; tflint

  else
    echo '==> tflint not found in PATH, skipping'
  fi

tfdocs:
  #!/bin/bash
  set -euo pipefail

  if command -v terraform-docs >/dev/null; then
    echo '==> Running terraform-docs'
    terraform-docs terraform
  else
    echo '==> terraform-docs not found in PATH, skipping'
  fi

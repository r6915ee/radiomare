# Triggers dev recipe.
default: dev

# Build a specified profile.
build profile="dev":
    cargo build --profile {{ profile }}

# Build the "dev" profile.
dev: (build "dev")

# Build the "release" profile.
release: (build "release")

# Trigger all tests.
test:
    cargo test

# Trigger all tests, plus allow access to stdout.
test-out:
    cargo test -- --nocapture

# Trigger Clippy.
lint:
    cargo clippy

# Test, and then generate the documentation.
doc:
    cargo test --doc
    cargo doc --no-deps

# Trigger doc recipe, and open $HTTP_SERVER in target/doc.
view-docs: doc
    ${HTTP_SERVER} target/doc

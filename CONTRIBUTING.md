## Contributing

We want to be open and set the right expectations: We are currently not set up
to accept external contributions.

## Release Checklist

 1. Update the version in `Cargo.toml` and `simd_tokens/Cargo.toml`.
 2. Ensure GitHub CI is passing.
 3. Ensure CHANGELOG.md is up-to-date.
 4. Create a version tag on GitHub, e.g. `v0.3.0`. This will trigger CI.
 5. Approve the CI run at https://github.com/google/simd_tokens/actions.
 6. Verify that the new version is up at https://crates.io/crates/simd_tokens.

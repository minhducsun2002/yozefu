# Releasing a new version

This document explains the release process of the tool. Most of the steps are automated with github actions. The `main` branch of the repository is protected. If you want to release a new version of the tool, you must create a release branch.


1. Ensure you are on a release branch.
2. Install `cargo-release`: `cargo install cargo-release`
3. Thanks to conventional commits and `cargo-semver-checks`, you can determine the next version to release.
4. Bump the version: `cargo release  "<major | minor | patch>" --no-publish --no-confirm --execute --no-tag`
5. [Create the pull request](https://github.com/MAIF/yozefu/compare).
6. If all checks succeed, the pull request will be accepted by a reviewer.
7. When a new release is created, a github action workflow is triggered to update the changelog. The updated changelog will be available on a branch named `changelog/<version>`. A pull request is automatically created with that branch, which must be approved to be merged into `main`.
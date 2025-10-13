# Release Process

To release a new version of `boil` the following steps need to be done:

1. Make sure the local `main` branch is up-to-date and in a clean state.
2. Run the `.scripts/release_boil.sh` script. This takes care of
   - generating the changelog
   - updating the `Cargo.toml` version
   - raising a PR with the changes
3. Merge the PR.
4. Add the appropriate tag on `main` by running `git tag -s`.
5. Push the tag.

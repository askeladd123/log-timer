Usefult when adding a new feature.

# creating a new branch
- create a new branch named after a feature or bug
- consider creating a file called `before-merge.md`
  - this states when the branch is ready to merge

# merging
- read through `todo.md`; you may have done some of them
- read commented code: consider removing it
- read `before-merge.md`; delete when done
- update `--help` text
- update `README.md`
- tests: consider making tests for new feature or bug
- run tests
- run linter: `cargo clippy`
- read `git log`: squash WIP commits with for example "fix me"
- `git rebase main`
- bump version, see [below](#bump-version)
  - update `Cargo.toml`
  - add tag

# bump version
Versions are stated both in `Cargo.toml` -> `package.version` and in *git tags*, which must be updated to reflect each other. Use major-minor-patch system like `v0.3.1`:
- major: breaking api changes
- minor: backwards compatible changes
- patch: bug fix or small improvement
- no bump: commits that only affect development

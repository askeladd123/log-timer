Usefult when adding a new feature.

# creating a new branch
- create a new branch named after a feature or bug
- consider creating a file called `before-merge.md`
  - this states when the branch is ready to merge

# add
- read through `todo.md`; you may have done some of them
- read commented code: consider removing it
- read `before-merge.md`; delete when done
- update `--help` text
- update `README.md`
- tests: consider making tests for new feature or bug
- run tests
- run linter: `cargo clippy`

# commit
- commit message: prepend with **version impact**: `maj`, `min`, `pat` and `dev` see [[#bump version]]
  - example: `git commit --message 'maj: renamed command'`
- read `git log`: squash WIP commits with for example "fix me"
- `git rebase main`
- run tests again
- version bump? see [below](#bump-version)
  - update `Cargo.toml`
  - make bump commit

# bump version
Since there's no releases anywhere yet, there is no rush to bump versions. Do it when you feel like it. 

## version impact
Prefix commits with following labels. These will be used later to create a version number using `major.minor.patch`, example: `v1.2.3`.
- `maj`, major: breaking api changes
- `min`, minor: backwards compatible changes
- `pat`, patch: bug fix or small improvement
- `dev`, no bump: commits that only affect development

## steps
When you fell like it's time, do the following steps:
- update [CHANGELOG.md](./checklist.md#changelog)
- update `Cargo.toml` -> `package.version`
- update `flake.nix` -> `*.buildRustPackage.version`
- create a commit called `bump: vX.X.X` but replace `X` with version numbers

## changelog
Use `git log` to create a summary of changes in `CHANGELOG.md`. Put the newest on top of the file, with the following markdown header structure:
- "vX.X.X" example v1.2.3
  - major
  - minor
  - patch

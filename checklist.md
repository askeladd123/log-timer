Usefult when adding a new feature.

# creating a new branch
- create a new branch named after a feature or bug
- consider creating a file called `before-merge.md`
  - this states when the branch is ready to merge

# merging
- read through todos; you may have done some of them
- read commented code: consider removing it
- read `before-merge.md`; delete when done
- update `--help` text
- update `README.md`
- tests: consider making tests for new feature or bug
- run tests
- read `git log`: squash WIP commits with for example "fix me"
- `git rebase main`

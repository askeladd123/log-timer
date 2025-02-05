A list of bugs, features or other changes **todo** in the future:
- support white space separated values
- consider removing csv column `duration` as it can be derived from `datetime-start` and `datetime-stop`
  - replace *row format* with just: `time format` = `standard` | `readable`
- change config file format to 'toml'
- add version to config file to ensure compatibility
- add options 'get --first' and 'get --last' for filtering
- change from DateTime<Local> to DateTime<FixedOffset> to support time zones
- `log get total` should have a flag where you can get HH:MM instead of just minutes
- add warning when log is empty
- readme: tips on how to view data: `column`, nushell and plotting
- no subcommands: inform about duration
  - example: "Currently timing activity 'skole-diskret-matte', started 30 minutes ago at 13:07."
- handle broken config file: delete prompt maybe?
- add test for `--time` flag
- investigate bug: too many minutes
  - `log get total` gives me *2days 20hr 9 min* (4089min) of `skole-diskret-2`
  - counting gives me *20hr 8min* (1208min)
- nix build: wait for [bugfix](./todo.md#nix-flakes-src)
- nix: provide module
- nix: replace docker tests with nix

# nix flakes src
Nix flakes by default uses the whole git tree to detect changes. This means updates to `README.md` or this file requires a rebuild. When using a non-flake build, one could use `nix-gitignore.gitignoreSource` [1], but flakes apparently has a bug makes this useless [2].

Working non-flake `default.nix`:
```nix
{pkgs ? import <nixpkgs> {}}:
pkgs.rustPlatform.buildRustPackage {
  pname = "log-timer";
  version = "0.0.0";
  src =
    pkgs.nix-gitignore.gitignoreSource ''
      /*.md
      /tests/
    ''
    ./.;
  cargoLock.lockFile = ./Cargo.lock;
}
```

# links
[1]: https://nixos.org/manual/nixpkgs/stable/#sec-pkgs-nix-gitignore
[2]: https://github.com/NixOS/nix/issues/5549 "flake docs don't explain that local flakes are copied to the nix store before evaluation"

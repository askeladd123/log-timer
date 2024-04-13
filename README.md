# log timer
A command line program I made to improve my workflow. Below is the output of `log-timer help`:

```
This tool helps you keep track of time. Example usage: 
- 'log-timer start washing-dishes'
- 'log-timer stop' when you're done.
The program will add an entry with the time you washed dishes to a log file. See 'log-timer configure --help' for initial setup of the log file.

Usage: log-timer [COMMAND]

Commands:
  start       Begin timing an activity now.
  stop        Stop timing an activity, and write it to a log file. 
  abort       Stop timing an activity, and forget about it.
  configure   Use this command to for example decide where to log activities.
  get-config  A quick way to see how the program is configured. This is from a file stored somewhere on your machine.
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')
```

# build and run from source
- `git clone git@github.com:askeladd123/log-timer.git`
- `cd log-timer`
- `cargo run -- help`, `cargo run -- start washing-dishes` or any other available command

## requirements
You will need *git*, and *the rust toolchain* to compile.

# run from anywhere
To make it easier to run, add the binary to the `PATH` environment variable, or make a *shell alias*.

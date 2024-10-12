#!/usr/bin/env nu
# this script has integration tests for 'log-timer' cli

use std assert

alias log-timer = ./target/debug/log-timer

let functions = [
    [name, func];
    [start-stop-label, {
        log-timer start test
        log-timer stop
        assert (log-timer get logs | str contains test)
        }
    ],
    [overwrite-options, {
        touch ~/log-1.csv
        log-timer config set --log-file-path log-1.csv --row-formatter v1-0
        let config = log-timer config get | from json
        assert ($config.log_file_path == ('~/log-1.csv' | path expand))
        assert ($config.row_formatter == V1_0)

        log-timer config set --row-formatter v2-0
        let config = log-timer config get | from json
        assert ($config.log_file_path == ('~/log-1.csv' | path expand))
        assert ($config.row_formatter == V2_0)

        touch ~/log-2.csv
        log-timer config set --log-file-path ~/log-2.csv
        let config = log-timer config get | from json
        assert ($config.log_file_path == ('~/log-2.csv' | path expand))
        assert ($config.row_formatter == V2_0)
    }]
]

def run_tests [functions, passed, total] -> int {
    mut passed = $passed
    for pair in ($functions | enumerate) {
        let text = $"[($pair.index + 1) / ($total)] ($pair.item.name): "

        print $"(ansi yellow)($text)running(ansi reset)"
        do --ignore-program-errors $pair.item.func
        if $env.LAST_EXIT_CODE == 0 {
            print $"(ansi green)($text)success(ansi reset)"
            $passed += 1
        } else {
            print $"(ansi red)($text)failed(ansi reset)"
        }
    
        print ""
    }
    return $passed
}

def main [password] {
    if $password != "you-are-contained" {
        print "wrong password: enter 'you-are-contained' to run the script. this will mess around with logs and configs, so you should do this in a docker container"
        exit 1
    }

    let total = $functions | length
    mut passed = 0

    print $"running ($total) tests:\n"
    $passed = (run_tests $functions $passed $total)
    
    let color = if $passed == $total {ansi green} else {ansi red}
    print $"($color)testing done: ($passed) of ($total) passed(ansi reset)"
}

#!/usr/bin/env nu
# this script has integration tests for 'log-timer' cli

alias log-timer = ./target/debug/log-timer

let functions = [
    [name, func];
    [start-stop-label, {
        log-timer start test
        log-timer stop
        log-timer get logs | str contains test
        }
    ],
]

def run_tests [functions, passed, total] -> int {
    mut passed = $passed
    for pair in ($functions | enumerate) {
        let text = $"[($pair.index + 1) / ($total)] ($pair.item.name): "

        print $"(ansi yellow)($text)running(ansi reset)"
        if (do --ignore-program-errors $pair.item.func) and ($env.LAST_EXIT_CODE == 0) {
            print $"(ansi green)($text)success(ansi reset)"
            $passed += 1
        } else {
            print $"(ansi red)($text)failed(ansi reset)"
        }
    
        print ""
        return $passed
    }
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

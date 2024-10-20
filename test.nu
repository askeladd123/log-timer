# WARNING: uses nushell on commit e735bd475f53b62e30a3e4a041e21462db63ac47
# this is because it uses $err.rendered from `try catch`

# this script has integration tests for 'log-timer' cli

# TODO: find out why std assert does not work

def assert [
    condition: bool,
    message?: string,
    --error-label: record<text: string, span: record<start: int, end: int>>
] {
    if not $condition {
        error make {
            msg: ($message | default "Assertion failed."),
            label: ($error_label | default {
                text: "It is not true.",
                span: (metadata $condition).span,
            })
        }
    }
}

alias log-timer = ./target/debug/log-timer

let functions = [
    [name, func];
    [start-stop-label, {
        log-timer start test
        log-timer stop
        log-timer get logs
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
    },],
    [set-default, {
        log-timer config set-default --confirm
        let current = log-timer config get | from json
        let default = log-timer config get-default | from json
        assert ($current == $default)
    }],
]

def run_tests [functions, passed, total] -> int {
    mut passed = $passed

    for pair in ($functions | enumerate) {
        let text = $"(ansi reset) [($pair.index + 1) / ($total)] ($pair.item.name)"

        print $"(ansi yellow)running($text)" --no-newline
        try { 
            do $pair.item.func out+err> /tmp/log-timer-test-output # FIXME: this is a workaround for redirecting output to a variable, not performant
            print $"\r(ansi green)success($text)"
            $passed += 1
        } catch {
            print $"\r(ansi red)failure($text)"
            print (open /tmp/log-timer-test-output)
            print $in.rendered
        }
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

    print $"running ($total) tests:"
    $passed = (run_tests $functions $passed $total)
    
    let color = if $passed == $total {ansi green} else {ansi red}
    print $"testing done: ($color)($passed) of ($total)(ansi reset) passed"
}

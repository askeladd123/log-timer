# this script has integration tests for 'log' cli

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

def run_tests [functions, passed, total] -> int {
    mut passed = $passed

    for pair in ($functions | enumerate) {
        let text = $"(ansi reset) [($pair.index + 1) / ($total)] ($pair.item.name)"

        print $"(ansi yellow)running($text)" --no-newline
        try { 
            do $pair.item.func out+err> /tmp/log-test-output # FIXME: this is a workaround for redirecting output to a variable, not performant
            print $"\r(ansi green)success($text)"
            $passed += 1
        } catch {
            print $"\r(ansi red)failure($text)"
            print (open /tmp/log-test-output)
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
    let functions = source ./tests.nu

    let total = $functions | length
    mut passed = 0

    print $"running ($total) tests:"
    $passed = (run_tests $functions $passed $total)
    
    let color = if $passed == $total {ansi green} else {ansi red}
    print $"testing done: ($color)($passed) of ($total)(ansi reset) passed"
}

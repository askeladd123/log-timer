[
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

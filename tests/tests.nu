[
    [name, func];
    [start-stop-label, {
        log start test
        log stop
        log get logs
        assert (log get logs | str contains test)
        }
    ],
    [overwrite-options, {
        touch ~/log-1.csv
        log config set --log-file-path log-1.csv --row-formatter v1-0
        let config = log config get | from json
        assert ($config.log_file_path == ('~/log-1.csv' | path expand))
        assert ($config.row_formatter == V1_0)

        log config set --row-formatter v2-0
        let config = log config get | from json
        assert ($config.log_file_path == ('~/log-1.csv' | path expand))
        assert ($config.row_formatter == V2_0)

        touch ~/log-2.csv
        log config set --log-file-path ~/log-2.csv
        let config = log config get | from json
        assert ($config.log_file_path == ('~/log-2.csv' | path expand))
        assert ($config.row_formatter == V2_0)
    },],
    [set-default, {
        log config set-default --confirm
        let current = log config get | from json
        let default = log config get-default | from json
        assert ($current == $default)
    }],
]

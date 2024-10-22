A list of bugs, features or other changes **todo** in the future:
- support white space separated values
- consider removing csv column `duration` as it can be derived from `datetime-start` and `datetime-stop`
- change config file format to 'toml'
- add version to config file to ensure compatibility
- add options 'get --first' and 'get --last' for filtering
- change from DateTime<Local> to DateTime<FixedOffset> to support time zones
- `log-timer get total` should have a flag where you can get HH:MM instead of just minutes
- add warning when log is empty

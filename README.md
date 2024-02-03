# timeat

Convert unixtime to date string.

## Example

### HELP
```
> timeat --help
Usage: timeat.exe [OPTIONS] [UNIXTIME]...

Arguments:
  [UNIXTIME]...

Options:
  -u, --utc
  -d, --datetime-only
  -f, --format <FORMAT>
  -h, --help             Print help
  -V, --version          Print version
```


### If UNIXTIME is not specified, the current UNIXTIME is output.
```
> timeat
1706941065
```

### Converts the specified UNIXTIME to a date string.

```
> timeat 1706941065
unixtime: 1706941065 , local: 2024-02-03 15:17:45 +09:00

```

### -u Output in UTC.
```
> timeat -u 1706941065
unixtime: 1706941065 , utc: 2024-02-03 06:17:45 UTC
```

### -d Output only the date string.
```
timeat -d 1706941065
2024-02-03 15:17:45 +0900
```

### -f Specifies the date string format.
[DATETIME FORMAT](https://docs.rs/chrono/latest/chrono/format/strftime/index.html)
```
timeat -d -f "%Y/%m/%d" 1706941065
2024/02/03
```











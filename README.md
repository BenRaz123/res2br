# res2br

### Description

`res2br` is a tool that converts a resolution (like 480p) to a bitrate (like 300) and offers a switch between Mbps and Kbps.

### Installation

To install, run
```shell
% cargo install res2br
```

### ✨ new in 2.0.0! ✨ Configuration

With configuration, you can store a custom table as well you preference for using kbps with a config file. By default, `res2br` will search `~/.config/res2br/config.json` for config. However, you can change this by changing the `$RES2BR_CONFIG` environment variable or by passing in `-c <PATH>` or `--config-path <PATH>`. The json file recognizes 2 fields: `"table"` (a 'dictionary' or 'hashmap' consisting of strings and floats) as well as `"use_kbps_by_default"` (a boolean). The default configuration looks like this:

```json
{
    "table": {
        "1080p": 3.000,
        "720p": 1.500,
        "540p": 0.989,
        "360p": 0.460,
        "270p": 0.327,
        "180p": 0.193
    },
    "use_kbps_by_default": false
}
```

### Status

- [x]  Started
- [x]  Create core function
- [x]  Allow switching
- [x]  Stable release

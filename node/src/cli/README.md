# Asynchronous running

## As master
Command: `run-async-as-master`

### Stages:
1.  Call `generate-template`: generating config template with name
    `common.toml` in path, specified by `public-path`.
1.  Continue form step 2 of `run-async` command.

### Params:
Name | Type | Default | Required by
--- | ---  | --- | ---
`validators-count` | number |  | `generate-template`
`supervisor-mode` | `"simple"` or `"decentralized"` | `simple` | `generate-template`

## As slave
Command: `run-async`

### Stages:
1.  Await for `common.toml` file in path, specified in `public-path`
    by checking it existence in path, specified by `public-path`
    parameter, with delay, specified by `attempt-delay` until the number of attempts is over.
1.  Create config files by `generate-config` command in path, specified by 

### Params:
Name | Type | Required by
--- | --- | ---
`public-path` | path | `generate-config`

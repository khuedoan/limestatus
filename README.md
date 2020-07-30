# limestatus

**WORK IN PROGRESS**

Generates status bar to use with lemonbar

<!-- vim-markdown-toc GFM -->

* [How it (should) look like](#how-it-should-look-like)
* [Features](#features)
* [Usage](#usage)

<!-- vim-markdown-toc -->

## How it (should) look like

```
1 2 3 · 5 · · · 9 0                           04:20            直lmeo  10% ﬙ 6.96 GB  96% 奔---
```

## Features

- [ ] Workspaces
- [x] Datetime
- [ ] Wifi
- [ ] CPU utilization
- [ ] CPU temperature
- [ ] RAM usage
- [x] Battery
- [ ] Brightness
- [ ] Volume

## Usage

Compile from source

```sh
cargo build --release
```

Then copy the binary file to your `$PATH`, for example:

```sh
cp target/release/limestatus /usr/local/bin/
```

To launch the bar, pipe `limestatus` output to `lemonbar`, for example:

```sh
limestatus | lemonbar
```

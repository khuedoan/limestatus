# limestatus (WORK IN PROGRESS, DO NOT USE)

Generates status bar to use with lemonbar

## Table of contents

<!-- vim-markdown-toc GFM -->

* [Preview](#preview)
* [Features](#features)
* [Usage](#usage)
    * [Dependencies](#dependencies)
    * [Installation](#installation)
    * [Example command](#example-command)

<!-- vim-markdown-toc -->

## Preview

How it should look like:

```
1 2 3 · 5 · · · 9 0                           04:20            直lmeo  10% ﬙ 6.96 GB  96% 奔---
```

How it looks like now:

```
1                                             04:20                                         96%
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

### Dependencies

- `lemonbar`
- `cargo` (build)

### Installation

Compile and install without cloning this repository:

```sh
sudo cargo install --git https://github.com/khuedoan/limestatus.git --root /usr/local
```

Or if you've already cloned it:

```sh
cd limestatus
sudo cargo install --path .
```

### Example command

To launch the bar, pipe `limestatus` output to `lemonbar`, for example:

```sh
limestatus | lemonbar
```

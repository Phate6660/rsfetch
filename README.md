## rsfetch

This is a WIP rewrite of [rsfetch](https://github.com/rsfetch/rsfetch) from scratch.<br>
I've been really unhappy with the codebase now for various reasons.<br>
It's made development for rsfetch downright unpleasant at times.

Table of Contents:

- [Example Output](#example-output)
- [Features](#features)
- [Help](#help)
- [TODO](#todo)

## Example Output

`$ cargo run --features=music -- -cDdehkmsuUp portage`

```
CPU:       Intel(R) Core(TM) i5-3470 CPU @ 3.20GHz
Device:    OptiPlex 7010
Distro:    Gentoo
Editor:    /usr/bin/emacsclient
Hostname:  gentoo
Kernel:    5.4.48-ck-valley
Packages:  87 (explicit), 573 (total)
Shell:     /bin/bash
Uptime:    6d 0h 51m
User:      valley
Music:     Machine Head - Burn My Eyes (1994) - I'm Your God Now
```

## Features

Currently there are 2 features. At least one must be chosen.<br>

- `nomusic`, this will cause rsfetch to not pull in the `mpd` crate, and will cause the music function to output `N/A`.
- `music`, this will cause rsfetch to pull in the `mpd` crate, and display the music info as: 

`artist - album (date) - title`

## Help

`$ cargo run -- --help`

```
rsfetch 0.1.0
Phate6660 <https://pages.codeberg.org/Phate6660>

An info fetch tool written in Rust. Everything is off by default, enable what you want.

USAGE:
    rsfetch [FLAGS] [OPTIONS]

FLAGS:
    -c               Display the model of the CPU.
    -D               Display the name of the device.
    -d               Display the name of the distro.
    -e               Display the name of the user's editor. Must have the $EDITOR environmental variable set.
        --help       Prints help information
    -h               Display the hostname of the device.
    -k               Display the name of the kernel.
    -m               Display currently playing music. Only mpd is supported. Must be built with the music feature.
    -s               Display the name of the user's shell.
    -u               Display the uptime.
    -U               Display the name of the user.
    -V, --version    Prints version information

OPTIONS:
    -p, --packages <manager>    Display package count.
```

## TODO

- CPU info (implemented, only tested on Gentoo)
- DE/WM info
- memory info
- properly implement async
- terminal info

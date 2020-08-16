## rsfetch

This is a WIP rewrite of [rsfetch](https://github.com/rsfetch/rsfetch) from scratch.<br>
I've been really unhappy with the codebase now for various reasons.<br>
It's made development for rsfetch downright unpleasant at times.

----

A blazing fast (<20 ms) information fetching utility.<br>
According to [`hyperfine`](https://github.com/sharkdp/hyperfine) anyways.

Table of Contents:

- [Example Output](#example-output)
- [Extra Info](#extra-info)
- [Features](#features)
- [Help](#help)
- [TODO](#todo)

## Example Output

`$ cargo run --features=music -- -cDdEeghkmMstuUp portage`

```
CPU:          Intel Core i5-3470 CPU @ 3.20GHz
Device:       OptiPlex 7010
Distro:       Gentoo
Editor:       /usr/bin/emacsclient
Environment:  bspwm
GPU:          AMD/ATI Cedar Radeon HD 5000/6000/7350/8350 Series
Hostname:     gentoo
Kernel:       5.4.48-ck-valley
Memory:       15971 MB
Packages:     87 (explicit), 570 (total)
Shell:        /bin/bash
Terminal:     xterm
Uptime:       2d 9h 54m
User:         valley
Music:        System Of A Down - System Of A Down (1998) - Know
```

`$ cargo run --features=music,pretty_output -- -cDdEeghkmMstuUp portage -C 0`

```
0──────────────────────────────────────────────────────────────────────0
│ CPU         │ Intel Core i5-3470 CPU @ 3.20GHz                      │
│ Distro      │ Gentoo                                                 │
│ Device      │ OptiPlex 7010                                          │
│ Editor      │ /usr/bin/emacsclient                                   │
│ Environment │ bspwm                                                  │
│ GPU         │ AMD/ATI Cedar Radeon HD 5000/6000/7350/8350 Series     │
│ Hostname    │ gentoo                                                 │
│ Kernel      │ 5.4.48-ck-valley                                       │
│ Memory      │ 15971 MB                                               │
│ Packages    │ 87 (explicit), 570 (total)                             │
│ Shell       │ /bin/bash                                              │
│ Terminal    │ xterm                                                  │
│ Uptime      │ 2d 9h 54m                                              │
│ User        │ valley                                                 │
│ Music       │ System Of A Down - System Of A Down (1998) - Suite-Pee │
0──────────────────────────────────────────────────────────────────────0
```

## Extra info

Crate deps and binary size depending on features for rsfetch:

- `music,plain_output`: 20 crates, 3.7 MB
- `music,pretty_output`: 42 crates, 3.9 MB
- `nomusic,plain_output`: 16 crates, 3.6 MB
- `nomusic,pretty_output`: 38 crates, 3.9 MB

Crates explicitely used, and why:

- `clap`: CLI framework
- `mpd`: completely optional, used for the feature `music`
- `nixinfo`: contains all of the information gathering functions
- `prettytable-rs`: completely optional, used for the feature `pretty_output`

Tokei stats (cropped to save space): `tokei -t=rust .`

```
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 Rust                   17          413          378            0           35
===============================================================================
```

## Features

Currently there are 2 features. They are completely optional.<br>

- `music`, this will cause nixinfo to pull in the `mpd` crate, and display the music info as: 

`artist - album (date) - title`

- `pretty_output`, this will cause rsfetch to pull in the `prettytable-rs` crate, see example above for how it looks.

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
    -E               Display the name of the user's editor. Must have the $EDITOR environmental variable set.
    -e               Display the user's environment. First checks for a DE, before resorting to parsing your
                     $HOME/.xinitrc for your WM.
        --help       Prints help information
    -h               Display the hostname of the device.
    -k               Display the name of the kernel.
    -m               Display free/total memory.
    -M               Display currently playing music. Only mpd is supported. Must be built with the music feature.
    -s               Display the name of the user's shell.
    -t               Display the name of the user's terminal.
    -u               Display the uptime.
    -U               Display the name of the user.
    -V, --version    Prints version information

OPTIONS:
    -C, --corner <char>         Set the corner character.
    -p, --packages <manager>    Display package count.
    -T, --temperature <C/F>     Display CPU temp for Raspberry Pi, must have CPU field enabled.
```

## TODO

- better output (implemented)
- implement async
- information gathering functions are split off into a separate lib ([implemented](https://github.com/Phate6660/nixinfo))

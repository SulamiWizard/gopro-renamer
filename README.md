# gopro-renamer

A CLI tool for renaming chaptered GoPro video files into a more sortable format.
No AI has been used in the creation of this code. I was inspired to make this
because I have used a python script called [gopro_renamer](https://github.com/kcha/gopro_renamer)
by kcha and I wanted to practice learning the Rust programming language.

## Table of Contents

<!--toc:start-->

- [gopro-renamer](#gopro-renamer)
  - [Table of Contents](#table-of-contents)
  - [The Problem](#the-problem)
  - [The Solution](#the-solution)
  - [Usage](#usage)
  - [File Name Format](#file-name-format)
  - [Planned Features](#planned-features)
  - [Building](#building)
  <!--toc:end-->

## The Problem

GoPro names chaptered video files like this:

```
GX010056.MP4   ← chapter 01 of video 0056
GX020056.MP4   ← chapter 02 of video 0056
GX010057.MP4   ← chapter 01 of video 0057
```

The encoding type prefix (`GH`/`GX`) as well as the chapter number being before
the video number causes files to group by encoding rather than by video,
so chapters of the same video don't sort together in a file explorer.

## The Solution

Renames files to put the video number first, then chapter:

```
GX010056.MP4  →  0056_CH01.MP4
GX020056.MP4  →  0056_CH02.MP4
GX010057.MP4  →  0057_CH01.MP4
```

Now files sort by video number, with chapters in order beneath them.

## Usage

```bash
# Rename files in the current directory
gopro-renamer

# Rename files in a specific directory
gopro-renamer /path/to/videos

# Dry run — preview renames without applying them
gopro-renamer -d /path/to/videos
gopro-renamer --dry-run /path/to/videos

# Concatenate chapters together into 1 single mp4 file per video
gopro-renamer -c /path/to/videos

# Dry run — preview concatenations without applying them
gopro-renamer -d -c /path/to/videos
```

## File Name Format

Shown below is [GoPro's naming scheme for chaptered video files](https://community.gopro.com/s/article/GoPro-Camera-File-Naming-Convention?language=en_US)

`GHzzxxxx.MP4`

| Part   | Meaning           |
| ------ | ----------------- |
| `GH`   | AVC encoding      |
| `GX`   | HEVC encoding     |
| `zz`   | Chapter number    |
| `xxxx` | Video/clip number |

## Planned Features

- TUI/GUI
- Better formatted output with colors

## Building

```bash
cargo build --release
```

Requires Rust and Cargo.

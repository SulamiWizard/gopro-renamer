# gopro-renamer

A CLI tool for renaming chaptered GoPro video files into a more sortable format.

## The Problem

GoPro names chaptered video files like this:

```
GX010056.MP4   ← chapter 01 of video 0056
GX020056.MP4   ← chapter 02 of video 0056
GX010057.MP4   ← chapter 01 of video 0057
```

The encoding type prefix (`GH`/`GX`) causes files to group by encoding rather than by video, so chapters of the same video don't sort together in a file explorer.

## The Solution

Renames files to put the video number first, then chapter:

```
GX010056.MP4  →  GX_0056_CH01.MP4
GX020056.MP4  →  GX_0056_CH02.MP4
GX010057.MP4  →  GX_0057_CH01.MP4
```

Now files sort by video number, with chapters in order beneath them.

## Usage

```bash
# Rename files in the current directory
gopro-renamer

# Rename files in a specific directory
gopro-renamer /path/to/videos

# Dry run — preview renames without applying them
gopro-renamer -d
gopro-renamer --dry-run
```

## File Name Format

| Part   | Meaning           |
| ------ | ----------------- |
| `GH`   | AVC encoding      |
| `GX`   | HEVC encoding     |
| `zz`   | Chapter number    |
| `xxxx` | Video/clip number |

## Building

```bash
cargo build --release
```

Requires Rust and Cargo.

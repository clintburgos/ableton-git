# Ableton-Git

This is a wrapper for `git` that ensures the best settings for version controlling Ableton Live projects.
* Audio files are managed by `git-lfs`
* `.als` files are unzipped for git operations to enable diffs, merges, and all that good stuff-- at least, depending on how much you're willing to experiment with the project files ;)

The motivation for this project is to leverage the power and reliability of `git` for online music collaboration.

## Installation

### Prerequisites
You must have `git` and `git-lfs` installed.

### Binary
Download from the [releases page](https://github.com/clintburgos/ableton-git/releases). Place it in your `bin` directory.

### Source
Clone this repo and run `cargo build --release`. The compiled binary will appear in `target/release`.

## Usage
```
# From the root of your Ableton Live project:
$ ableton-git init

# Or to check out someone else's work:
$ ableton-git clone ...

# Use `ableton-git` as you would normally use `git`!
```

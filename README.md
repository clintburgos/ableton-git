# Ableton-Git [![Build Status](https://travis-ci.org/clintburgos/ableton-git.svg?branch=master)](https://travis-ci.org/clintburgos/ableton-git) ![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/github/clintburgos/ableton-git?branch=master&svg=true)

This is a wrapper for `git` that ensures the best settings when working with Ableton Live projects.
* Audio files are managed by `git-lfs`
* `.als` files are unzipped to XML, so some conflicts can be resolved by looking at the `diff`.

The motivation for this project is to leverage the power and reliability of `git` for online music collaboration.

## Installation

### Prerequisites
You must have `git` and `git-lfs` installed.

### Binary
Download the executable for your platform from the [releases page](https://github.com/clintburgos/ableton-git/releases). Place it in your `bin` directory.

### Source
Clone this repo and run `cargo build --release`. The compiled binary will appear in `target/release`.

## Usage
``` sh
# From the root of your Ableton Live project:
$ ableton-git init

# Or to check out someone else's work:
$ ableton-git clone ...

# Use `ableton-git` as you would normally use `git`!
```

I recommend hosting Ableton projects on [GitLab](https://gitlab.com/) because you get 10 GB per repo for free 🙂

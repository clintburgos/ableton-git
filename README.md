# Ableton-Git

This is a wrapper for `git` that ensures the best settings for version controlling Ableton Live projects.
* Audio files are managed by `git-lfs`
* `.als` files are unzipped for git operations to enable diffs, merges, and all that good stuff-- at least, depending on how much you're willing to experiment with the project files ;)

At the very least this was built to provide a relatively easy way to version control in a more reliable manner than Splice.

## Installation
1. Install `git`
1. Install `git-lfs`
1. `curl https://raw.githubusercontent.com/clintburgos/ableton-git/master/ableton-git -o /bin/ableton-git`

# Rustea Git Repository Restoration

## Background

I came across this library while learning about the Rust programming language
and tinkering on side-projects. I've always enjoyed applications with a nice
looking terminal UI and have been using it to build some of my own.

The library has been very useful and the design approachable. While all the
versions of the library can still be downloaded from crates.io, the original
GitHub repository owned by `lazops` has unfortunately been deleted. This is my
attempt to restore as much history of the original repository as possible.

I don't intend to continue development in this specific repository. Instead I
am restoring it to keep as a reference and archive of the original work from
`lazops`. I do have ideas of things I'd like to add or change, but will do that
in a separate fork from this repository where it can continue as a new Rust
crate.

## References

- [Introducing Rustea](https://www.reddit.com/r/rust/comments/u5vef7/introducing_rustea_an_easytouse_tui_crate_for/)
  - The original post introducing Rustea on Reddit. It links to the now deleted
    repository at `https://github.com/lazops/rustea`:
- [Rustea on crates.io](https://crates.io/crates/rustea)
- [tonyb983/rustea fork](https://github.com/tonyb983/rustea)
  - This is the only fork I could find of the original repository. Its last
    commit was `8b7222f8fa38799c77f6c1f960be37670e6efe16` which was version
    `0.1.3` of Rustea.

I have a bit of a nostalgia for terminal based UIs. I grew up with them and
some of my earliest computer networking experiences were on text based BBS
systems. Wanting to learn more about the Rust programming language, I decided to

## Restoration Process

### Step 1: Forking a base

Starting with the fork from
[tonyb983/rustea](https://github.com/tonyb983/rustea) provided real revision
history from `lazops`, the original author, from versions `0.1.0` to `0.1.3`. I
cloned this repository as my base.

### Step 2: Tagging versions

The forked repository, and I assume the original as well, did not have any tags
defined. I decided to add these in the best I could using the `git blame`
command on `Cargo.toml`, looking for revisions where the `version` attribute
was incremented. On the first revision I saw a change of the `version`
attribute, I recorded a tag in git. I did this for versions `0.1.0 - 0.1.3`.

### Step 3: Download version history

While I could not recreate the real revision history of the original
repository, I at least had versions hosted in crates.io which I could treat as
a tagged snapshot of the codebase. A tar file of the crate can be downloaded
using the following URL scheme which I did for versions `0.1.4 - 0.1.6`:

`https://crates.io/api/v1/crates/rustea/{version}/download`

### Step 4: Estimate the commit datetime

Again, the exact dates of the commits for versions beyond `0.1.3` are not
known, but can be estimated by looking at the metadata of the library uploaded
to crates.io. The following URL scheme will return information on a crate and
among that data is a `created_at` field. This value was retrieved and recorded
for versions `0.1.4 - 0.1.6`.

`https://crates.io/api/v1/crates/rustea/{version}`

### Step 5: Patch the version history

With the historical source code and a rough estimate on when that code was
committed, I began to patch in versions `0.1.4`, `0.1.5`, and `0.1.6` one at a
time. I used the following environment variables to identify Laz as the
original author and set the commit date to the approximate dates found in Step
#4. Each commit was then tagged with the respective version number.

```
GIT_AUTHOR_NAME=Laz
GIT_AUTHOR_EMAIL=lazh@fastmail.com
GIT_AUTHOR_DATE={date retrieved from Step #4}
GIT_COMMITTER_DATE=$GIT_AUTHOR_DATE
```

To see the tags for each version and their references, use the following:

```
git show-ref --tags
```

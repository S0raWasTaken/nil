# [&nbsp;&nbsp;&nbsp;]
A little wrapper for xbps

Heavily inspired on VPM and Paru

### Usage
```sh
# Searches and prompts a package for install
nil <package_name>
```

### Installing:
Depends on:
- `bash`
- `fzf`
- `rust`            &nbsp;&nbsp;(build dependency)
- `cargo`                       (build dependency)
- `git` &nbsp;&nbsp;&nbsp;&nbsp;(build dependency)

```sh
git clone https://github.com/S0raWasTaken/nil --depth 1
cd nil

# installs the nil-query binary
cargo install --path .

chmod +x nil

# Make sure to add this directory to your PATH, or move it somewhere else
mkdir -p ~/.local/bin

cp nil ~/.local/bin

# done!
```

### Extra info
#### doas
This wrapper uses `sudo` internally and only when doing the install.

In case you use `doas` instead of `sudo`:
1. um. okay, I guess?
2. you probably already suffer enough that you already linked `/bin/doas` to `/bin/sudo`
3. `doas ln -s /usr/bin/doas /usr/bin/sudo`, but make sure you don't have it already.

#### Do I really need nil-query?
Not really, but it kinda does some sorting and prettifying of `xbps-query`'s output.
If you REALLY don't want it, just change this line on the [nil](./nil) script:

```diff
- done < <(nil-query $package | fzf -m --ansi)
+ done < <(xbps-query $package | fzf -m)
```

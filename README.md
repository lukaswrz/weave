# Weave

A fast, simple hard link manager.
Weave serves a similar purpose to [GNU Stow](https://www.gnu.org/software/stow/), but using hard links rather than symlinks.

## Usage

By default, Weave uses the `weave.toml` in the current working directory as its configuration file.
This can be overridden with the `--config` option.

A typical `weave.toml` might look like this:

```toml
from = "my-dotfiles"
to = "~"
```

The `my-dotfiles` directory should follow this structure: `my-dotfiles/<choice>/path/to/file`.
For example, if you intend to configure a program like [Helix](https://helix-editor.com/), you could have this path to the Helix configuration file: `my-dotfiles/helix/.config/helix/config.toml`.
By default, Weave will simply link all directories in `my-dotfiles`, but if you want to just link your Helix configuration files, you can just run `weave helix`.
You don't need to split these up, but it turns out to be useful in some scenarios.

### Example

This is what it looks like when I run Weave on my [dotfiles repository](https://hack.moontide.ink/pingfisher/dotfiles):

```
$ weave --verbose --force
 INFO weave: weaving from home to /home/me
 INFO weave: choices are ["nix", "fish", "helix", "qutebrowser", "ssh", "git", "bash", "mpv", "bottom"]
 INFO weave: /home/me/.config/nix/nix.conf already links to home/nix/.config/nix/nix.conf
 INFO weave: /home/me/.config/fish/config.fish already links to home/fish/.config/fish/config.fish
 INFO weave: /home/me/.config/helix/languages.toml already links to home/helix/.config/helix/languages.toml
 INFO weave: /home/me/.config/helix/themes/revontuli_transparent.toml already links to home/helix/.config/helix/themes/revontuli_transparent.toml
 INFO weave: /home/me/.config/helix/themes/revontuli_night_transparent.toml already links to home/helix/.config/helix/themes/revontuli_night_transparent.toml
 INFO weave: /home/me/.config/helix/themes/revontuli_night.toml already links to home/helix/.config/helix/themes/revontuli_night.toml
 INFO weave: /home/me/.config/helix/themes/github_dark_high_contrast_transparent.toml already links to home/helix/.config/helix/themes/github_dark_high_contrast_transparent.toml
 INFO weave: /home/me/.config/helix/themes/revontuli.toml already links to home/helix/.config/helix/themes/revontuli.toml
 INFO weave: /home/me/.config/helix/config.toml already links to home/helix/.config/helix/config.toml
 INFO weave: /home/me/.config/qutebrowser/config.py already links to home/qutebrowser/.config/qutebrowser/config.py
 INFO weave: /home/me/.ssh/config already links to home/ssh/.ssh/config
 INFO weave: /home/me/.config/git/config already links to home/git/.config/git/config
 INFO weave: /home/me/.config/git/ignore already links to home/git/.config/git/ignore
 INFO weave: /home/me/.bashrc already links to home/bash/.bashrc
 INFO weave: /home/me/.config/mpv/input.conf already links to home/mpv/.config/mpv/input.conf
 INFO weave: linked /home/me/.config/mpv/mpv.conf to home/mpv/.config/mpv/mpv.conf
 INFO weave: linked /home/me/.config/bottom/bottom.toml to home/bottom/.config/bottom/bottom.toml
 INFO weave: 2 linked, 0 overwritten, 15 skipped
```

Run `weave --help` for more information.

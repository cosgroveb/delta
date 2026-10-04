# TUI mode

`delta --tui` is an interactive git client and pager whose diffs are rendered by delta. It is based on [gitu](https://github.com/altsem/gitu) by altsem, a git client inspired by Magit, and is embedded via a [fork](https://github.com/dandavison/gitu). It is experimental. `--gitu` is an alias of `--tui`.

```sh
delta --tui                            # status view of the current repository
delta --tui show <rev>                 # one commit
delta --tui blame <file>               # blame of a file
delta --tui rebase <upstream>          # edit an interactive rebase's instruction list
git diff | delta --tui --pager         # browse a patch
```

Diffs are rendered by the running delta binary, configured as usual by your `[delta]` git config. The TUI maps each rendered row back to its line in the patch, so side-by-side, line numbers etc. do not prevent staging individual lines.

## Trying it out

```sh
GIT_PAGER='delta --tui --pager' git log
```

Move to a commit with `j` / `k` and press `enter` to open it; `q` goes back.

```sh
GIT_PAGER='delta --tui --pager' git diff
```

`shift+tab` folds every file, and again unfolds them; `tab` folds the file or hunk under the cursor. `S` toggles side-by-side and `L` line numbers. `?` lists the commands for the row under the cursor, and `q` quits. To use the TUI as git's pager all the time, see Git config below.

## Side-by-side, line numbers and +/- markers

`S` toggles side-by-side, `L` line numbers, and `+` keeps the `+`/`-` at the start of each line, for copying a diff as a patch (`plus-minus-markers`, which is delta's `keep-plus-minus-markers`). This adds to your delta config for the session; nothing is written to disk.

`-` lists these and any other delta features you configure (see TUI config below); `enter` turns the selected one on or off. In the list, typing a key makes it the selected feature's key, and typing the feature's own key removes it; the TUI saves the keys in its config file. Or set them there (`~/.config/gitu/config.toml`):

```toml
[general]
# These are the defaults. Keys set here replace all of them. A key that is
# already bound is refused.
diff_renderer.feature_keys = { side-by-side = "S", line-numbers = "L", plus-minus-markers = "+" }
```

## Git config

```gitconfig
[core]
    pager = delta
[pager]
    diff = delta --tui --pager
    show = delta --tui --pager
    log = delta --tui --pager
[sequence]
    editor = delta --tui sequence-editor
```

With this, `git diff`, `git show`, `git log` and `git rebase -i` open in the TUI. The TUI finds the git command that started it and runs it again itself, in the same directory, after each change you make. git aliases are recognised.

With `--quit-if-one-screen` (as less's), output that fits on the screen is printed and the TUI exits; that also leaves nothing to stage from, so it suits `log` better than `diff`.

## Log and commit formats

`git log` and `git show` typed at the command line use git's `format.pretty`. The `git log` and `git show` that the TUI runs itself (`l l`, `enter` on a commit) take their arguments from `log_args` and `show_args` in the TUI config. A commit is one cursor stop only if its first row matches delta's `commit-regex`, so a format of your own needs a `commit-regex` that matches it. For example, in git config:

```gitconfig
[delta]
    commit-regex = ^(▸|commit )
[pretty]
    commit-format = tformat:%n%n▸ %h %C(blue)%an %C(blue)%ar%C(auto)%d%C(reset)%n%n    %C(green)%s%C(auto)
    commit-format-full = tformat:%n%n▸ %h %C(blue)%an %C(blue)%ar%C(auto)%d%C(reset)%n%n%w(0,4,4)%C(green)%s%C(reset)%n%+b
```

and in the TUI config:

```toml
[general]
log_args = ["--stat", "--date=relative", "--pretty=commit-format"]
show_args = ["--pretty=commit-format-full"]
```

## TUI config

Optional, at `~/.config/gitu/config.toml` (or `--config <file>`). The diff renderer is always delta, whatever this file says. Values are the defaults except where marked as an example. See the [default config](https://github.com/dandavison/gitu/blob/main/src/default_config.toml) for every setting and binding.

### delta features

```toml
[general]
# Features offered by `-`. `*` and `?` match the `[delta "name"]` features in
# your git config. Example (the default omits "my-*"):
diff_renderer.features = ["side-by-side", "line-numbers", "plus-minus-markers", "my-*"]
# Top-level keys that toggle features. Keys set here replace all of these.
diff_renderer.feature_keys = { side-by-side = "S", line-numbers = "L", plus-minus-markers = "+" }
```

### Other settings

```toml
[general]
# Arguments to the `git log` run by `l l` and `l o`, also used for the commits
# in the rebase instruction list. See "Log and commit formats" above.
log_args = []
# Arguments to the `git show` run by enter on a commit and `delta --tui show`.
show_args = []
# Files left out of diff and log views, as git pathspecs. Example:
hide = ["*.pb.go", "vendor"]
# What heads each file: delta's file header ("delta"), or the file's
# `git diff --stat` row, counted by the TUI ("stat").
file_header = "delta"
# Open a patch of more than one file with its files folded, if it does not fit
# on one screen.
start_folded = false
# Let the cursor stop on unchanged lines.
visit_context_lines = true
# Open the status view on the first unstaged file, or else the first staged one.
status_cursor_on_changes = true
# false: show only the branch name at the top of the status view.
verbose_branch_status = true
# Status view sections that start folded: "branch_status", "untracked",
# "recent_commits". Example:
collapsed_sections = ["branch_status"]
# Commits in the status view's Recent commits; 0 removes the section.
recent_commits_limit = 10
# Stashes in the status view.
stash_list_limit = 10
# Ask before discarding: "line" (always), "hunk", "file" or "never".
confirm_discard = "line"
confirm_quit.enabled = false
# Always show the key help at the bottom of the screen.
always_show_help.enabled = false
# Refresh the status view when files change.
refresh_on_file_change.enabled = true
mouse_support = false
mouse_scroll_lines = 3
# Read at most this much of git's output; the rest is dropped.
max_input_bytes = 2_000_000

[bindings]
# Each binding is a list of keys; the binding names are in the table under
# Keybindings below. Examples:
root.quit = ["q", "esc", "left"]
rebase_todo.rebase_todo_drop = ["d", "ctrl+k"]

[style]
# Colors are a name ("blue"), a hex value ("#0086b3") or a 256-color index.
# Examples:
cursor = { symbol = "▌", fg = "blue" }
selection_line = { mods = "BOLD" }
section_header = { fg = "yellow" }
rebase_todo.drop = { fg = "red", mods = "BOLD" }
```

## Usage

`?` lists the commands available for the row under the cursor.

### Moving

- `j` / `k` move down / up; `ctrl+j` / `ctrl+k` move one row.
- `alt+j` / `alt+k` move to the next / previous file, hunk or commit; `alt+h` moves to the file or commit containing the cursor.
- `space` / `backspace` move a page, `ctrl+d` / `ctrl+u` half a page, `g g` / `G` to the top / bottom.
- `/` searches (a regex over the text on screen); `n` / `N` go to the next / previous match. A match inside a fold is unfolded.
- `enter` on a commit opens it as `git show`; `q` goes back. `g s` opens the status view from any view.

### Folding

- `tab` folds or unfolds the file, hunk or commit under the cursor.
- `shift+tab` folds every file; pressing it again unfolds everything. In `git log -p` the second press folds every commit and the third unfolds everything.
- `git diff --stat` opens with every file folded to its stat row; `tab` unfolds a file to its diff.
- `file_header = "stat"` heads every file with its stat row, folded or not. After `shift+tab` unfolds everything, files are headed by delta's file headers.
- `start_folded = true` opens a large patch of several files folded.

### Staging

- `s` stages, `u` unstages and `K` discards the file, hunk or line under the cursor.
- `a` applies and `v` reverses it, for example to undo part of a commit shown by `git show`.
- `shift+down` / `shift+up` select several lines in one hunk; the next command acts on all of them.
- The commands offered depend on the git command: `git diff` can stage; `git diff --cached` can unstage; `git diff <rev>`, `git show` and `git log -p` can apply and reverse.
- In `git log -p`, each commit's diff is under the commit and can be folded, applied and reversed.

### Changing the git command

These re-run the git command and redraw the view.

- `:` edits the command, for example to add `--cached`, `-M` or `-S <text>`, or to change `git log` to `git log -p`. Only read-only git commands are accepted.
- `d` on a file header hides that file.
- `&` sets which files to show, as git pathspecs: `src !*_test.go` shows `src` without its tests.
- `U` sets the number of context lines; `W` shows the whole function; an empty answer restores the default.

### Commits

- `l l` shows the log of the current branch and `l o` of another; both use `log_args`.
- On a commit: `enter` opens it, `y` copies its hash, `r i` starts an interactive rebase from it, `V V` reverts it and `A A` cherry-picks it.
- `c f` and `c F` (fixup, instant fixup) open the log to choose the commit to fix up.
- `B` on a diff line opens the blame of its file; `enter` on a blame line opens that commit at that line.
- `Y` lists branches and tags; `enter` on one opens it as `git show`.

### Interactive rebase

With `sequence.editor` set as above, `git rebase -i` opens its instruction list in the TUI. `r i` and `delta --tui rebase <upstream>` open it directly; `r i` with no commit under the cursor opens the log to choose one. Commits are listed newest first, as in the log.

| key | action | binding |
|---|---|---|
| `p` | pick | `rebase_todo.rebase_todo_pick` |
| `r` | reword | `rebase_todo.rebase_todo_reword` |
| `e` | edit | `rebase_todo.rebase_todo_edit` |
| `s` | squash | `rebase_todo.rebase_todo_squash` |
| `f` | fixup | `rebase_todo.rebase_todo_fixup` |
| `d` | drop | `rebase_todo.rebase_todo_drop` |
| `alt+j` / `alt+k` | move the commit down / up | `rebase_todo.rebase_todo_move_down` / `rebase_todo.rebase_todo_move_up` |
| `enter` | start the rebase | `rebase_todo.rebase_todo_start` |
| `q` / `esc` | leave; as git's sequence editor, call the rebase off | `rebase_todo.quit` |

Movement and folding in the list are bound separately from the main view, as `rebase_todo.move_down` etc.

### Prompts

Prompts take readline keys (`ctrl+a`, `ctrl+e`, `alt+b`, `alt+f`, `ctrl+w`, `ctrl+k`, `ctrl+u`, `ctrl+y`, `ctrl+t`). `up` / `down` step through the history of `:` and `&`, kept per worktree.

### Other

- `ctrl+z` suspends the TUI; `fg` resumes it.
- `--debug` shows the output of the commands the TUI runs.

## Keybindings

`h` or `?` shows the bindings available for the row under the cursor. Each binding is rebound under `[bindings]` in the TUI config by the name in the last column, e.g. `root.hide_file = ["ctrl+k"]`. The keys inside each menu are listed in the [default config](https://github.com/dandavison/gitu/blob/main/src/default_config.toml).

| key | action | binding |
|---|---|---|
| `j` / `k` | move down / up | `root.move_down` / `root.move_up` |
| `ctrl+j` / `ctrl+k` | move down / up a row | `root.move_down_line` / `root.move_up_line` |
| `shift+down` / `shift+up` | extend the line selection | `root.extend_selection_down` / `root.extend_selection_up` |
| `alt+j` / `alt+k` | next / previous section | `root.move_next_section` / `root.move_prev_section` |
| `alt+h` | parent section | `root.move_parent_section` |
| `space` / `backspace` | page down / up | `root.full_page_down` / `root.full_page_up` |
| `ctrl+d` / `ctrl+u` | half page down / up | `root.half_page_down` / `root.half_page_up` |
| `g g` / `G` | top / bottom | `root.move_top` / `root.move_bottom` |
| `ctrl+e` / `ctrl+y` | scroll down / up | `root.scroll_view_down` / `root.scroll_view_up` |
| `tab` | fold | `root.toggle_section` |
| `shift+tab` | fold all | `root.toggle_all_sections` |
| `enter` / `right` | show | `root.show` |
| `s` `u` `K` | stage, unstage, discard | `root.stage`, `root.unstage`, `root.discard` |
| `a` `v` | apply, reverse | `root.apply`, `root.reverse` |
| `:` | edit the git command | `root.edit_git_command` |
| `d` | hide the file under the cursor | `root.hide_file` |
| `&` | set which files to show | `root.file_patterns` |
| `U` | set the diff context | `root.diff_context` |
| `-` | delta features | `root.renderer_features` |
| `S` `L` `+` | side-by-side, line numbers, +/- markers | `general.diff_renderer.feature_keys` |
| `/` `n` `N` | search, next, previous | `root.search`, `root.search_next`, `root.search_previous` |
| `y` | copy hash | `root.copy_hash` |
| `Y` | show refs | `root.show_refs` |
| `B` | blame | `root.blame` |
| `g s` | status view | `root.status` |
| `g r` | refresh | `root.refresh` |
| `ctrl+z` | suspend | `root.suspend` |
| `h` `?` | help | `root.help_menu` |
| `q` | quit | `root.quit` |
| `b` | branch menu | `root.branch_menu` |
| `c` | commit menu | `root.commit_menu` |
| `f` | fetch menu | `root.fetch_menu` |
| `F` | pull menu | `root.pull_menu` |
| `P` | push menu | `root.push_menu` |
| `l` | log menu | `root.log_menu` |
| `m` | merge menu | `root.merge_menu` |
| `M` | remote menu | `root.remote_menu` |
| `r` | rebase menu | `root.rebase_menu` |
| `X` | reset menu | `root.reset_menu` |
| `V` | revert menu | `root.revert_menu` |
| `A` | cherry-pick menu | `root.cherry_pick_menu` |
| `z` | stash menu | `root.stash_menu` |

## Limitations

- `-` cannot turn off a feature that your delta config turns on.
- delta does not mark merge conflict rows or `\ No newline at end of file` rows, so the TUI does not show them.
- Output beyond `max_input_bytes` is dropped; a row at the end says so.
- A shell alias (`!…`) is not recognised as the git command it runs.

# MINimal DOTfiles manager

Mindot is a simple and minimal dotfiles manager that copies the dotfiles in the current directory to share them.

TODO: screenshot help

Mindot works by saving the paths to the dotfiles in `~/.config/mindot/files.json` and uses `backup` (to copy them from the home to the directory you specify) or `restore` (to copy them from the current directory to the home).

## Example
My use case is: 
- add the dotfiles I want to backup using `mindot add file1 file2`
- use `mindot backup <path>` to copy them elsewhere and later push them to my github [repo](https://github.com/0xfederama/dotfiles)
- then, when I need to use them on another machine, I simply clone my repo there and use `mindot restore` to copy them back.

## Disclaimer
Yes, the files are duplicated, but that's good in some cases, for example when you want to modify them only for sharing.

Please keep in mind that I made this project just to learn Rust, so something may be off or it could be useless to you. Anyway, feel free to reach me if you have code reviews or improvement for the program.

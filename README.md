# Move files

A simple CLI utility to move files that match regex into specific folder and
after that clean up empty directories.


## Motivation

Using standard `mv` command is sometimes inconvenient, especially when file
names contain escape characters or spaces. This is why I decided to create this
utility.


## Usage

```sh
move_files <REGEX> <DESTINATION>
```

For e.g.

```sh
move_files "\.mp3" "Singles" # Moves all mp3 files to Singles folder recursively
```

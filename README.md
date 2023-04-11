# Vizia File Dialog

A simple file dialog for the Vizia framework. At this point consider it a POC, it has no stable API (in fact it does not really have an API at all).

Big thanks to @geom3trik and all the great people of Vizia community.

## Usage

```shell
cargo run
```

## Design

The UI is inspired by the `vscode` file selector (electron/gtk?), with clickable path segments. Initially the current path is selected, and its content displayed as a list of entries (files/folders). Clicking on a folder extends the path. Clicking on a path segment selects the path (from root until the selected path segment). Clicking on a folder in the current dir extends the path from the current dir (potentially replacing path segments). Nothing fancy but seems quite intuitive.

There is a editable text button at the top, currently unused will show the selected file with full path eventually...

- Navigation:
  - Tab:ing of the path segments. Enter to select segment.
- Sorting:
  - Sorting entries by name, type, modification date.

## TODO

Known issues:

- Handling of symlinks. As of now symlinks are indicated by a trailing `@` (and a different color). Symlinks cannot be followed as of now. (Handling of symlinks is a bit OS dependent, no clear picture what to do/what we want here).

- If window is shrunk to much the path segments might go out of view. gtk3 file dialog puts `<` `>` buttons for scrolling the list, not sure if we want that or a scroll bar or something else.

- Attempting to traverse into a folder for which you don't have permissions to results in a panic.

- Sorting by size is based on the actual size of the file including folders and symlinks. Not sure what we want to do here.

There are a lot of possibilities:

- There is a editable text button at the top, currently unused. We can have it showing the currently selected file, and editing this manually could set the path segments and selected file if a match is found. Not sure if we want this, or some other behavior.

- Filtering:
  - Allowing to filter name by reg-exp (e.g.)
- Selection:
  - Inc/Dec among current dir entries. Enter to select file/folder.
- File Type:
  - Currently we don't show file type. Its not part of the metadata directly, so question is where to get this info from. Perhaps the system environment but that will be system dependent. One can see on the permissions if the file is executable and show that somehow.
- API and wrapping
  - Widget as either a modal dialog and/or embedded widget? Not sure what we want here.

## License

Following the [MIT](https://github.com/vizia/vizia/blob/main/LICENSE) license of Vizia.

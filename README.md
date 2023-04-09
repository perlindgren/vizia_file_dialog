# Vizia File Dialog

A simple file dialog for the Vizia framework. At this point consider it a POC, it has no stable API (in fact it does not really have an API at all).

## Usage

```shell
cargo run
```

## Design

The UI is inspired by the `vscode` file selector, with clickable path segments. Initially the current path is selected, and its content displayed as a list of entries (files/folders). Clicking on a folder extends the path. Clicking on a path segment selects the path (from root until the selected path segment). Clicking on a folder in the current dir extends the path from the current dir (potentially replacing path segments). Nothing fancy but seems quite intuitive.

## TODO

There are a lot of possibilities:
- Path info:
  - Highlighting the current set path somehow.
- File info:
  - Highlighting the current selected file.
  - Distinguishing file type by color.
  - Showing file size.
- Filtering:
  - Sorting entries by type, date, etc.
  - Allowing to filter by reg-exp (e.g.)
- Navigation:
  - Tab:ing of the path segments. Enter to select segment.
  - Inc/Dec among current dir entries. Enter to select file/folder.

- API and wrapping
  - Widget as a modal dialog perhaps?


## License

As free as can be...
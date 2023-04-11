# Vizia File Dialog

A simple file dialog for the Vizia framework. At this point consider it a POC, it has no stable API (in fact it does not really have an API at all).

Big thanks to @geom3trik and all the great people of Vizia community.

## Usage

```shell
cargo run
```

## Design

The UI is inspired by the `vscode` file selector, with clickable path segments. Initially the current path is selected, and its content displayed as a list of entries (files/folders). Clicking on a folder extends the path. Clicking on a path segment selects the path (from root until the selected path segment). Clicking on a folder in the current dir extends the path from the current dir (potentially replacing path segments). Nothing fancy but seems quite intuitive.

There is a editable text button at the top, currently unused will show the selected file with full path eventually...

- Navigation:
  - Tab:ing of the path segments. Enter to select segment.

## TODO

There are a lot of possibilities:

- Filtering:
  - Sorting entries by type, date, etc.
  - Allowing to filter by reg-exp (e.g.)
- Selection:

  - Inc/Dec among current dir entries. Enter to select file/folder.

- API and wrapping
  - Widget as a modal dialog perhaps?

## License

Following the [MIT](https://github.com/vizia/vizia/blob/main/LICENSE) license of Vizia.

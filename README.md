<div align="center">
    <img src=".github/logo.svg" alt="logo" height="128" />
    <h4>nyaa.si TUI</h4>
    <p>
        Browse nyaa.si from your terminal
        <br>
        A fast, keyboard-driven TUI for searching English-translated anime torrents on <a href="https://nyaa.si">nyaa.si</a>
        <br>
        Built in Rust with <a href="https://ratatui.rs">ratatui</a>, powered by the nyaa RSS feed. No scraping, no browser
    </p>
</div>

<p align="center">
    <img src=".github/demo.gif" alt="Demo" width="90%" />
</p>

## Zen mode

<p align="center">
    <img src=".github/screenshot-zen.png?v=1795000000" alt="Demo Zen" width="90%" />
</p>

## Features

- Search English-translated anime, starting on the latest trusted releases
- Cycle uploader presets or type any uploader name
- Filter by trusted only, no remakes, or everything
- Sort by date, seeders or size
- Results as a table: title, size, date, seeders, leechers
- Trusted rows in green, remakes in red
- Titles shortened to the show, episode and quality
- Details popup: comments, peers, trackers, magnet and download link
- Copy the magnet link to the clipboard (`wl-copy`, `xclip`, `pbcopy`, `clip`, or `termux-clipboard-set` on Termux)
- Download the `.torrent` file into your XDG download dir (or `~/Downloads`)
- Prebuilt binaries for Linux, macOS and Windows

## Installation

### Prebuilt binaries

Grab an archive from [Releases](https://github.com/metaory/nyaa-tui/releases/latest)

| Platform     | Target                                                                                                                                |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------- |
| Linux x86_64 | [x86_64-unknown-linux-musl](https://github.com/metaory/nyaa-tui/releases/latest/download/nyaa-tui-x86_64-unknown-linux-musl.tar.gz)   |
| Linux arm64  | [aarch64-unknown-linux-musl](https://github.com/metaory/nyaa-tui/releases/latest/download/nyaa-tui-aarch64-unknown-linux-musl.tar.gz) |
| macOS Intel  | [x86_64-apple-darwin](https://github.com/metaory/nyaa-tui/releases/latest/download/nyaa-tui-x86_64-apple-darwin.tar.gz)               |
| macOS Apple  | [aarch64-apple-darwin](https://github.com/metaory/nyaa-tui/releases/latest/download/nyaa-tui-aarch64-apple-darwin.tar.gz)             |
| Windows      | [x86_64-pc-windows-msvc](https://github.com/metaory/nyaa-tui/releases/latest/download/nyaa-tui-x86_64-pc-windows-msvc.zip)            |

```sh
target=x86_64-unknown-linux-musl
curl -fsSL "https://github.com/metaory/nyaa-tui/releases/latest/download/nyaa-tui-$target.tar.gz" \
  | tar xz -C ~/.local/bin nyaa-tui
```

Windows archives are `.zip`

### From source

Requires a Rust toolchain (edition 2024)

```sh
cargo install --git https://github.com/metaory/nyaa-tui
```

## Usage

```sh
nyaa-tui
```

| Key                       | Action                  |
| ------------------------- | ----------------------- |
| `j` / `k`, `↓` / `↑`      | down / up               |
| `g` / `G`, `Home` / `End` | top / bottom            |
| `Enter`                   | details / submit search |
| `c`                       | copy magnet             |
| `d`                       | download `.torrent`     |
| `r`                       | reload                  |
| `/`                       | search                  |
| `u`                       | cycle uploader          |
| `U`                       | type uploader           |
| `f`                       | cycle filter            |
| `s`                       | cycle sort              |
| `z`                       | zen (title and seed)    |
| `Esc`                     | cancel / close / quit   |
| `q`                       | quit                    |

## Development

```sh
cargo run                    # run the TUI
cargo test                   # unit tests, feed parsing uses fixtures/rss.xml
cargo clippy --all-targets   # clippy all + pedantic
```

---

## License

[MIT](LICENSE)

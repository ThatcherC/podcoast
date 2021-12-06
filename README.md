

## Resources:

- Rust [rss](https://docs.rs/rss/latest/rss/index.html) crate

- Rust rss crafte [ITunes extension](https://docs.rs/rss/latest/rss/extension/itunes/index.html)
  - Examples [here](https://docs.rs/rss/latest/rss/extension/itunes/struct.ITunesChannelExtension.html)
- [dir2cast](https://github.com/ben-xo/dir2cast), a PHP directory-to-podcast server

- Apple Podcast requirements

- [Google Podcast requirements](https://support.google.com/podcast-publishers/answer/9889544?hl=en)


## Requirements

Google:

- Channel:
  - One or more `<item>`s - one per episode
  - `<link>`
  - `<title>`
  - `<image>` or `<itunes:image>`

- Items (Episodes):
  - `<enclosure>`
  - `<title>`




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

## Directories

Input:
```
inputs/
    channel.yaml
    ep1/
        episode1.mp3
        episode.yaml
    second-episode/
        episode2.mp3
        episode.yaml
```

Output:
```
output-dir/
    rss/
        podcast.rss
    img/
        channel-image.png
        episode1-picture.jpg
    audio/
        episode1.mp3
        episode2.mp3
```
which should be hosted .... TODO ...

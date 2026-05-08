# Odysee.com channel content downloader

Downloads all content from a channel given an odysee.com url.

## Usage

```
Usage: odysee-dl-cli [OPTIONS] <URL>

Download all content from an Odysee channel. <URL> should be the URL of the channel,
e.g. https://odysee.com/@channel:1

Options:
    -h, --help          Print help information
    -d, --dir <DIR>     Output directory (default: .)
    -r, --resume        Resume an interrupted download
```

# Build

Find current Windows build in Releases or build from source with `cargo build --release`.

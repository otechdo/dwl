<p align="center">
<a href="https://github.com/otechdo/dwl" title="source code" target ="_blank">  <img src="https://raw.githubusercontent.com/otechdo/dwl/master/icons/dwl.svg" alt="dwl" style="zoom: 50%;" width="250" /></a>
</p>

## I
Spotify premium is required

YouTube premium is required

## II

Install libnotify, pip, ffmpeg, on your system.

## III

```bash
sudo -H pip install --upgrade youtube-dl spotdl
```

## IV

```bash
spotdl --download-ffmpeg
```

## V

```bash
spotdl --generate-config
```

## Config 

The config file is located ~/.spotdl/config.json under linux.

```json
{
  "client_id": "5f573c9620494bae87890c0f08a60293",
  "client_secret": "212476d9b0f3472eaa762d90b19b0ba8",
  "auth_token": null,
  "user_auth": false,
  "headless": false,
  "cache_path": "/Users/username/.spotdl/.spotipy",
  "no_cache": false,
  "max_retries": 3,
  "audio_providers": [
    "youtube-music"
  ],
  "lyrics_providers": [
    "genius",
    "azlyrics",
    "musixmatch"
  ],
  "playlist_numbering": false,
  "scan_for_songs": false,
  "m3u": null,
  "output": "{artists} - {title}.{output-ext}",
  "overwrite": "skip",
  "search_query": null,
  "ffmpeg": "ffmpeg",
  "bitrate": null,
  "ffmpeg_args": null,
  "format": "mp3",
  "save_file": null,
  "filter_results": true,
  "threads": 4,
  "cookie_file": null,
  "restrict": false,
  "print_errors": false,
  "sponsor_block": false,
  "preload": false,
  "archive": null,
  "load_config": true,
  "log_level": "INFO",
  "simple_tui": false,
  "fetch_albums": false,
  "id3_separator": "/",
  "ytm_data": false,
  "add_unavailable": false,
  "web_use_output_dir": false,
  "port": 8800,
  "host": "localhost",
  "keep_alive": false,
  "allowed_origins": null,
  "keep_sessions": false
}
```

## Installation

### From Crates.io
```bash
cargo install spot_dwl
```

### From GitHub

```bash
install -m 644 desktop/dwl.desktop /usr/share/applications
install -m 755 target/release/dwl /usr/bin
install -m 644 desktop/dwl.desktop /usr/share/applications
install -m 644 icons/dwl.svg /usr/share/icons
```

## Usage

```bash
dwl
```

## Song destinations

All songs will be downloaded in ~/Records directory

~/Records/cookies.txt must be exists for download with youtube cookies.

[Extension](https://chromewebstore.google.com/detail/get-cookiestxt-locally/cclelndahbckbenkjhflpdbgdldlbecc)


<p align="center">
<a href="https://github.com/otechdo/dwl" title="source code" target ="_blank">  <img src="https://raw.githubusercontent.com/otechdo/dwl/master/icons/dwl.svg" alt="dwl" style="zoom: 50%;" width="250" /></a>
</p>

## I

Spotify premium is required

YouTube premium is required

Linux is required

## II

Install python3, pipx, ffmpeg, on your system.

## III

```bash
pipx install spotdl 
```

## IV

```bash
export PATH="$HOME/.local/bin:$HOME/.cargo/bin:$PATH"
```

```shell
set -x PATH "$HOME/.local/bin:$HOME/.cargo/bin:$PATH"
```

## V

```bash
spotdl --download-ffmpeg
```

## Installation

### From Crates.io

```bash
cargo install dwl
```

### From GitHub

```bash
git clone https://github.com/otechdo/dwl.git && cd dwl && cargo build --release
install -m 755 target/release/dwl /usr/bin
install -m 644 desktop/dwl.desktop /usr/share/applications
install -m 644 icons/dwl.svg /usr/share/icons
```

## Usage

```bash
dwl
```

>  Show dwl errors

```bash
watch -n 1 tail -n 5 /tmp/dwl-stderr
```

> Show dwl ouptut

```bash
watch -n 1 tail -n 5 /tmp/dwl-stdout
```

## Config

[Extension](https://chromewebstore.google.com/detail/get-cookiestxt-locally/cclelndahbckbenkjhflpdbgdldlbecc)

```json
{
    "client_id": "",
    "client_secret": "",
    "auth_token": null,
    "user_auth": false,
    "headless": true,
    "cache_path": "/home/otechdo/.spotdl/.spotipy",
    "no_cache": false,
    "max_retries": 3,
    "use_cache_file": false,
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
    "output": "/home/otechdo/Music/{artist}/{album}/{title}.{output-ext}",
    "overwrite": "skip",
    "search_query": null,
    "ffmpeg": "ffmpeg",
    "bitrate": null,
    "ffmpeg_args": null,
    "format": "opus",
    "save_file": null,
    "filter_results": true,
    "album_type": null,
    "threads": 4,
    "cookie_file": "/home/otechdo/.cookies.txt",
    "restrict": null,
    "print_errors": false,
    "sponsor_block": false,
    "preload": true,
    "archive": null,
    "load_config": true,
    "log_level": "INFO",
    "simple_tui": false,
    "fetch_albums": false,
    "id3_separator": "/",
    "ytm_data": false,
    "add_unavailable": false,
    "generate_lrc": false,
    "force_update_metadata": false,
    "only_verified_results": false,
    "sync_without_deleting": false,
    "max_filename_length": null,
    "yt_dlp_args": null,
    "detect_formats": null,
    "save_errors": null,
    "ignore_albums": null,
    "proxy": null,
    "skip_explicit": false,
    "log_format": null,
    "redownload": false,
    "skip_album_art": false,
    "create_skip_file": false,
    "respect_skip_file": false,
    "web_use_output_dir": false,
    "port": 8800,
    "host": "localhost",
    "keep_alive": false,
    "allowed_origins": null,
    "keep_sessions": false
}
```

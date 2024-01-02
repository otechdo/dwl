# spot_dwl

A music downloader from youtube using spotify 

## I
Spotify premium is required

## II

Install libnotify, pip, ffmpeg, on your system.

## III

```bash
sudo pip3 install spotdl 
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

```bash
cargo install spot_dwl
```

## Usage

```bash
spot_dwl "song-url" "song url" ...
```
 
All song url start with https://open.spotify.com
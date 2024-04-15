<p align="center">
<a href="https://github.com/otechdo/dwl" title="source code" target ="_blank">  <img src="https://raw.githubusercontent.com/otechdo/dwl/master/icons/dwl.svg" alt="dwl" style="zoom: 50%;" width="250" /></a>
</p>

## I

Spotify premium is required

YouTube premium is required

Linux is required

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

## Installation

### From Crates.io

```bash
cargo install spot_dwl
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


## Song destinations

All songs will be downloaded in ~/Records directory

~/Records/cookies.txt must be exists for download with youtube cookies.

[Extension](https://chromewebstore.google.com/detail/get-cookiestxt-locally/cclelndahbckbenkjhflpdbgdldlbecc)


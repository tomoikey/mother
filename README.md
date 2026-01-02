# MOTHER Style Dialog Generator

A CLI tool to generate MOTHER 2 (EarthBound) style text dialog animations.

MOTHER2 (EarthBound) 風のテキストダイアログアニメーションを生成する CLI ツールです。

## Features
* Generate **GIF** animations.
* Generate **MP4** videos with sound.
* Generate **PNG** sequences.

## Requirements / 必須要件

To generate **MP4** files, **FFmpeg** must be installed and available in your system's PATH.

MP4ファイルを生成するには、システムに **FFmpeg** がインストールされており、パスが通っている必要があります。

* **macOS:** `brew install ffmpeg`
* **Windows:** `scoop install ffmpeg` or download from official site.
* **Linux:** `sudo apt install ffmpeg`

## Installation / インストール

```bash
$ chmod +x ./install.sh && ./install.sh
```

## Usage / 使い方

```bash
Usage: mother [OPTIONS] --text <TEXT>

Options:
  -t, --text <TEXT>      Text to display
  -o, --output <OUTPUT>  Output file [default: ./output.gif]
  -s, --speed <SPEED>    Speed of the gif [default: 8]
      --silent           Process silently
  -h, --help             Print help
```

## Examples / 使用例
### 1. Basic GIF generation (Default)

```bash
$ mother -t "Hello, World!"
# Generates ./output.gif
```

### 2. Generate MP4 with Sound

```bash
$ mother --text "PK Fire!" --output fire.mp4
```

## Credits & Legal / 権利表記・免責事項

### Disclaimer / 免責
This software is an unofficial fan work. It is not affiliated with, endorsed by, or connected to Nintendo Co., Ltd., HAL Laboratory, Inc., or APE Inc. MOTHER (EarthBound) is a registered trademark of Nintendo.

本ソフトウェアは非公式のファンメイド作品です。任天堂株式会社、株式会社ハル研究所、株式会社エイプ、および関連企業とは一切関係ありません。「MOTHER」は任天堂の登録商標です。

### Assets / 使用素材
This software uses the following assets. All rights belong to their respective owners.

本ソフトウェアでは以下の素材を使用しています。各素材の権利はそれぞれの制作者に帰属します。

#### Font / フォント
* **Name:**
  * MOTHER PIXEL2.ttf
* **Author/Source:**
  * https://savacanpixels.web.fc2.com/mother_pixels.html

#### Audio / 音声
* **SE:**
    * text_blip.wav
* **Source:**
    * Sourced via Starmen.net (Original sound from MOTHER 2 / EarthBound)
    * https://starmen.net/mother2/soundfx/new/007%20Text%20blip.wav

### License / ソフトウェアのライセンス
The source code of this tool is released under the [MIT License](LICENSE).

本ツールのソースコードは [MIT License](LICENSE) の下で公開されています。
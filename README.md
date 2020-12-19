# Ruffle 日本語対応デモ

## これはなに
[Ruffle](https://github.com/ruffle-rs/ruffle) の日本語関連の不具合を直してみたものです。  
【2020/12/19追記】このリポジトリで対応していた本家Ruffleの不具合がだいたい直ったようなので、フォント（noto-sans-definefont3.bin）だけ持っていって本家をビルドすれば日本語が表示できます。  
noto-sans-definefont3.binの作成には、とりぽっぽ🚂💨さん(twitter: @toripoppo0307)に多大なるご協力をいただきました。
 
## FAQ
 - Q.本家にPR投げないの
   - A.このリポジトリには本家の開発方針的に取り入れられない暫定的な対応も含まれています。  
       また、私の時間と英語力と技術力がなさすぎて年内に投げられそうにないので、  
       とりあえず公開してRuffleでの非英語コンテンツへの対応が現実的なことを示して  
       日本国内にもFlashコンテンツの存続にRuffleという選択肢があることを広めることが先決と考えて公開しました。  

 - Q.本家からの変更内容は？
   - A. ~~本家のIssue#1183の修正（マルチバイト文字が含まれる場合にパニックが発生していた）~~→本家で修正されたのでそちらをマージして削除  
       ~~マルチバイト文字の改行に対応（改行位置が適切ではないことがある）~~
       同梱のデバイスフォントをNoto-Sans-CJK-JPに差し替え(#1099で本家の開発者様が触れられている暫定対応)
       ~~SWFムービーから別のSWFムービーを呼び出した際にデバイスフォントが正常に読み込まれない問題の修正~~→本家で修正されたのでそちらをマージして削除  
       本家の不具合がほとんど直ったようです！

以下、本家のREADMEです。

<p align="center">
 <a href="https://ruffle.rs"><img src="https://ruffle.rs/assets/logo.png" alt="Ruffle"></a>
</p>
<p align="center">
 <a href="https://github.com/ruffle-rs/ruffle/actions">
  <img src="https://img.shields.io/github/workflow/status/ruffle-rs/ruffle/Test%20Rust?label=rust%20build" alt="Rust Build Status" />
  <img src="https://img.shields.io/github/workflow/status/ruffle-rs/ruffle/Test%20Web?label=web%20build" alt="Web Build Status" />
 </a>
  <a href="https://discord.gg/J8hgCQN">
      <img src="https://img.shields.io/discord/610531541889581066" alt="Ruffle Discord">
  </a>
  <br>
  <strong><a href="https://ruffle.rs">website</a> | <a href="https://ruffle.rs/demo">demo</a> | <a href="https://github.com/ruffle-rs/ruffle/releases">nightly builds</a> | <a href="https://github.com/ruffle-rs/ruffle/wiki">wiki</a></strong>
</p>

# Ruffle

Ruffle is an Adobe Flash Player emulator written in the Rust programming language. Ruffle targets both the desktop and the web using WebAssembly.

## Project status

Ruffle is in the proof-of-concept stage and can currently run early Flash animations and games. Basic ActionScript 1.0/2.0 support is in place and improving; ActionScript 3.0 support is forthcoming. For more info, read the [project roadmap](https://github.com/ruffle-rs/ruffle/wiki/Roadmap).

## Nightly builds

[Nightly builds](https://github.com/ruffle-rs/ruffle/releases) of Ruffle are available for desktop and web platforms including the browser extension.

Windows desktop builds require the x64 [2015-2019 Visual C++ Redistributable](https://support.microsoft.com/en-us/help/2977003/the-latest-supported-visual-c-downloads).

## Building from source

[Follow the official guide](https://www.rust-lang.org/tools/install) to install Rust for your platform.

### Desktop

- `cargo run --package=ruffle_desktop -- test.swf`

### Web or Extension

Follow [the instructions in the web directory](web/README.md#building-from-source) for building
either the web or browser extension version of Ruffle.

### Scanner

If you have a collection of "real world" SWFs to test against, the scanner may be used to benchmark
ruffle's parsing capabilities. Provided with a folder and an output filename, it will attempt to read
all of the flash files and report on the success of such a task.

- `cargo run --package=ruffle_scanner -- folder/with/swfs/ results.csv`

### Exporter

If you have a swf and would like to capture an image of it, you may use the exporter tool.
This currently requires hardware acceleration, but can be run headless (with no window).

- `cargo run --package=exporter -- path/to/file.swf`
- `cargo run --package=exporter -- path/to/file.swf path/to/screenshots --frames 5`

## Structure

- `core` contains the core emulator and common code
- `desktop` contains the desktop client (uses `wgpu-rs`)
- [`web`](web) contains the web client and browser extension (uses `wasm-bindgen`)
- `scanner` contains a utility to bulk parse swf files
- `exporter` contains a utility to generate PNG screenshots of a swf file

## Sponsors

You can support the development of Ruffle via [GitHub Sponsors](https://github.com/sponsors/ruffle-rs). Your sponsorship will help to ensure the accessibility of Flash content for the future. Thank you!

Sincere thanks to the diamond level sponsors of Ruffle:

<p align="center">
  <a href="https://www.newgrounds.com">
    <img src="https://ruffle.rs/assets/sponsors/newgrounds.png" alt="Newgrounds.com">
  </a>
  <a href="https://www.cpmstar.com">
    <img src="https://ruffle.rs/assets/sponsors/cpmstar.png" alt="CPMStar">
  </a>
  <a href="https://deepnight.net">
    <img src="https://ruffle.rs/assets/sponsors/deepnight.png" alt="Sébastien Bénard">
  </a>
  <a href="https://www.crazygames.com">
    <img src="https://ruffle.rs/assets/sponsors/crazygames.png" alt="Crazy Games">
  </a>
  <a href="https://www.coolmathgames.com">
    <img src="https://ruffle.rs/assets/sponsors/coolmathgames.png" alt="Cool Math Games">
  </a>
  <a href="https://www.nytimes.com/">
    <img src="https://ruffle.rs/assets/sponsors/nyt.png" alt="The New York Times">
  </a>
  <a href="https://www.armorgames.com/">
    <img src="https://ruffle.rs/assets/sponsors/armorgames.png" alt="Armor Games">
  </a>
</p>

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE_APACHE](LICENSE_APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE_MIT](LICENSE_MIT) or http://opensource.org/licenses/MIT)
  at your option.

### Contribution

Ruffle welcomes contribution from everyone. See CONTRIBUTING.md for help getting started.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.

The entire Ruffle community, including the chat room and GitHub project, is expected to abide by the [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct) that the Rust project itself follows.

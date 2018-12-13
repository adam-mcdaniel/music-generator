# music-generator
A tool to generate a lead sheet in a specified key given a certain number of choruses and verses.

## Notes

Right now it can only produce one chorus and one verse, in the process of adding more.

## Install and Run

For *nix
```bash
cd ~/Desktop
curl https://sh.rustup.rs -sSf | sh
git clone https://github.com/adam-mcdaniel/music-generator
cd music-generator
cargo build --release
./target/release/music_generator
```
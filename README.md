# No seriously, who's live?

A command line tool for finding out if a certain Twitch streamer is live.

# Usage

Clone this repository:
```bash
git clone https://github.com/huttongrabiel/whos_live.git
```

Navigate to the location of the directory:
```bash
cd path/to/cloned/repo
```

Run it:
```bash
cargo run <streamer_username> # ex. cargo run summit1g
```

Personally, I have this added to my path so that I can just call whos_live from
anywhere. To do so:
```bash
cd path/to/cloned/repo
cargo build --release
sudo cp ./target/release/whos_live /usr/bin/local
```

## fcount

A command line tool to count the number of files in a directory.
You might wonder what is the need of this tool when we can do either of the following

```bash
ls -1 | wc -l
gdu --inodes
du --inodes
```

Well the answer is simple:

- This was a good project for me to learn rust.
- The commands are intuitive for counting the number of files. Sure we can create alias, but these 
  commands are not exclusively meant for counting.
- Better output formatting.

## Installation

### Option 1: Binary Download

You can download or copy the URL of the pre-compiled binary for Linux or macOS from the [Releases Page](https://github.com/NavneetKanna/count/releases).

```bash
# Create the directory if it doesn't exist
mkdir -p ~/.local/bin

# Download and rename (Replace <url> with the link from Releases)
wget -O ~/.local/bin/fcount <url_to_binary>

# Make it executable
chmod +x ~/.local/bin/fcount
```

`Note: Ensure ~/.local/bin is in your $PATH.`

### Option 2: Build from Source

```bash
git clone https://github.com/NavneetKanna/fcount
cd fcount
RUSTFLAGS="-C opt-level=3" cargo install
```

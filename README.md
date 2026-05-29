# RSRUN

## About 

Rsrun is a simple cli tool that watches your current directory for any changes or file saves and runs specific commands found in its configuration.

Primarily tested with Rust, but it should work with most common programming languages.

## Installation 

```bash
cargo install rsrun

```

## Getting Started 

First, create the configuration file in the root of your cargo project.

```bash
touch rsrun.toml
```

### Configuration Example

```toml
commands = [["cargo clean", "cargo run"]]

ignore = ["/target/"]

```

### Configuration Details

- **commands**  
    A list of commands which will run in sequence.

- **ignore**  
  A list of paths or directories that should be ignored by the file watcher.

After creating the configuration file run:

```bash
rsrun
```

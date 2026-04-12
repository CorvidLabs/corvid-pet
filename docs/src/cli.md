# CLI Tool

corvid-pet ships a command-line binary for interacting with your pet directly from the terminal.

## Installation

```bash
cargo install corvid-pet --features cli
```

## Commands

### `show` (default)

Display the pet's ASCII art:

```bash
corvid-pet
corvid-pet show
corvid-pet show --name Pip
```

### `feed`

Feed the pet (requires saved state with simulation):

```bash
corvid-pet feed
```

### `play`

Play with the pet:

```bash
corvid-pet play
```

### `status`

Show pet status and stats:

```bash
corvid-pet status
```

### `sim`

Run an interactive simulation session:

```bash
corvid-pet sim
```

### `health`

Show repo health or record events:

```bash
# Show health status
corvid-pet health --state .corvid-pet.json

# Show as JSON
corvid-pet health --state .corvid-pet.json --json
```

### `react`

Record a CI/CD event:

```bash
corvid-pet react success --state .corvid-pet.json --name Corvin
corvid-pet react failure --state .corvid-pet.json --context "Tests failed on main"
```

### `comment`

Generate a markdown PR comment:

```bash
corvid-pet comment success --state .corvid-pet.json --name Corvin --context "All checks passed"
```

### `badge`

Generate a badge line for README embedding:

```bash
corvid-pet badge --state .corvid-pet.json --name Corvin
```

## Global Flags

| Flag | Description |
|------|-------------|
| `--name <NAME>` | Pet name (default: Corvin) |
| `--no-color` | Disable ANSI colors |
| `--color <COLOR>` | Body color |
| `--bubble-color <COLOR>` | Thought bubble color |
| `--state <PATH>` | Path to health state JSON file |

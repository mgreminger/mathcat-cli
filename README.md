# MathCAT CLI

A cross-platform, standalone command-line interface for the MathCAT engine. It converts MathML into spoken text strings for accessible document generation.

---

## Installation & Usage

MathCAT requires two things to run: this CLI binary, and a `Rules` directory containing the speech logic and braille dictionaries.

### 1. Using Pre-compiled Binaries (End Users)

If you just want to run the tool without compiling any Rust code:

1. Download the latest compiled executable for your operating system from the Releases page.
2. Download the `MathCAT-Rules.zip` asset from the exact same release and extract it. 
3. Place the `Rules` folder next to your executable (or specify its location via flags).

Run the CLI by piping MathML into standard input:

```bash
echo "<math><mi>x</mi><mo>+</mo><mi>y</mi></math>" | ./mathcat-cli
```

### 2. Building from Source

If you have Rust installed and want to compile the binary yourself:

```bash
# 1. Clone the repository
git clone [https://github.com/mgreminger/mathcat-cli.git](https://github.com/mgreminger/mathcat-cli.git)
cd mathcat-cli

# 2. Build the production binary
cargo build --release

# 3. Locate your compiled binary in target/release/mathcat-cli
```
*Note: You will still need to provide the `Rules` directory for the executable to function (see Configuration below).*

---

## Configuration: The `Rules` Directory

MathCAT relies on external YAML and configuration files. You must provide the path to these rules using one of three fallback methods, checked in this exact order:

**1. Command-Line Flag**
```bash
echo "<math><mi>x</mi></math>" | ./mathcat-cli --rules-dir /path/to/MathCAT/Rules
```

**2. Environment Variable**  
*(Highly recommended for server-side generation, Electron apps, or Pandoc Lua filters)*
```bash
export MATHCAT_RULES_DIR="/path/to/MathCAT/Rules"
echo "<math><mi>x</mi></math>" | ./mathcat-cli
```

**3. Executable-Relative Path**  
If no flag or environment variable is set, the CLI automatically looks for a directory named `Rules` residing in the exact same folder as the running `mathcat-cli` binary.


## Command Line Options

You can customize how the math is spoken using the `--style` and `--verbosity` flags. If omitted, the CLI defaults to `ClearSpeak` and `Medium`, which is generally preferred for engineering and educational content.

* `--language` or `--lang`: The locale code for the spoken text (e.g., en, es, de, vi). The specified language folder must exist within your Rules/Languages directory. Defaults to en.
* `--style`: Options are `ClearSpeak` (default) or `SimpleSpeak`.
* `--verbosity`: Options are `Terse`, `Medium` (default), or `Verbose`.

**Example:**
```bash
echo "<math><mfrac><mn>1</mn><mi>x</mi></mfrac></math>" | ./mathcat-cli --style SimpleSpeak --verbosity Verbose
```


---

## Development & Testing

This project leverages standard Cargo tooling. To set up a local development environment:

```bash
git clone [https://github.com/mgreminger/mathcat-cli.git](https://github.com/mgreminger/mathcat-cli.git)
cd mathcat-cli
```

### Setting up the local `Rules` directory
Because the MathCAT engine expects rules that exactly match its crate version, this project uses a custom Cargo alias to automatically extract the precise Rules/ folder directly from the published crates.io dependencies.

Run this command once to extract the Rules/ folder to your local root directory for testing:

```bash
cargo get-rules
```

### Running Tests
Once your `./Rules` directory is in place, you can run the integration test suite:

```bash
cargo test
```
The test suite spawns the CLI binary locally and asserts standard input/output behavior.

---

## License

This project is licensed under the [MIT License](LICENSE).
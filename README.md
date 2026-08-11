# envcat 🐱🔒

> **Modern Terminal Environment & Config File Inspector, Secret Masker, Diff Engine, Validator, and TUI**

`envcat` is an ultra-fast, security-first CLI utility and TUI application built in Rust to inspect, mask secrets, validate, diff, and convert environment variables and configuration files across multiple formats (`.env`, `JSON`, `YAML`, `TOML`, `INI`, and process system environment).

---

## 🌟 Key Features

- 🔍 **Multi-Format Parser Engine**: Seamlessly parses `.env`, `YAML`, `TOML`, `JSON`, `INI`, `.properties`, and live system process environment variables with format auto-detection.
- 🔒 **Intelligent Secret Masking**:
  - Automatically identifies secrets using keyword heuristics (`SECRET`, `PASSWORD`, `API_KEY`, `TOKEN`, `CREDENTIAL`, etc.).
  - Scans for high Shannon entropy strings (randomized hashes/tokens).
  - Built-in regex matchers for JWT tokens, AWS access keys, GitHub tokens, Slack tokens, and PEM private keys.
  - Automatically scrubs embedded credentials in database connection URIs (e.g., `postgres://user:password@host/db` $\rightarrow$ `postgres://user:******@host/db`).
- ⚡ **Interactive TUI**: Real-time terminal UI powered by `ratatui` featuring fuzzy search/filtering, detail inspection, live entropy calculation, and secret toggles.
- 🔀 **Environment Diff Engine**: Compare two environment files or system process environment against a configuration file to highlight missing, added, or changed values.
- 🛠️ **Config Validator & Linter**: Lint files for syntax errors, duplicate keys, empty values, and validate against a reference `.env.example` schema.
- 📄 **Example Generator**: Auto-generate scrubbed `.env.example` files from any environment file with intelligent type placeholders.
- 📤 **Export & Conversion**: Export parsed configurations directly to Dotenv, Shell `export KEY=VAL`, JSON, or YAML formats.
- 🐚 **Shell Completion**: Native completion script generator for Bash, Zsh, Fish, and PowerShell.

---

## 🚀 Installation & Building

### Prerequisites
- Rust compiler (`1.75+`) & `cargo`

```bash
git clone https://github.com/your-org/envcat.git
cd envcat
cargo build --release
```

The compiled binary will be available at `./target/release/envcat`.

---

## 📖 Usage & Commands

### 1. File Inspection (Default Panel View)
Inspect any `.env`, `.json`, `.yaml`, `.toml`, or `.ini` file with automatic secret masking:

```bash
envcat .env
envcat config.yaml
```

### 2. Formatted ASCII/ANSI Table View
Display configuration in a structured, color-coded table:

```bash
envcat --table .env
```

### 3. Interactive TUI Mode
Launch the interactive Terminal User Interface:

```bash
envcat -i .env
# or
envcat --interactive
```

- **`↑ / ↓`** or **`j / k`**: Navigate variables
- **`/`** or **`f`**: Live fuzzy search / filter
- **`Space`**: Toggle secret masking ON/OFF
- **`q`** or **`Esc`**: Exit TUI

### 4. Inspect System Environment
Inspect your active shell / process environment variables:

```bash
envcat --system
```

### 5. Environment Diff Engine (`diff`)
Compare two environment files or compare a file against the system environment:

```bash
# Compare two files
envcat diff production.env staging.env

# Show only changed / missing variables
envcat diff production.env staging.env --changes-only

# Compare local .env against active system environment
envcat diff .env --system
```

### 6. Config Validation & Linting (`check`)
Lint configuration for errors and check against a required schema template:

```bash
# Lint a single file
envcat check .env

# Validate .env against required keys in .env.example
envcat check .env --example .env.example
```

### 7. Generate `.env.example` (`generate-example`)
Auto-generate a clean `.env.example` with scrubbed values and type placeholders:

```bash
envcat generate-example .env > .env.example
```

### 8. Export & Conversion (`--export`)
Convert configurations between different file formats:

```bash
envcat .env --export json
envcat config.yaml --export dotenv
envcat .env --export shell
```

### 9. Custom Secret Patterns & Filtering
Filter by key prefix or supply a custom secret detection regex:

```bash
# Filter keys starting with APP_
envcat .env --prefix APP_

# Filter by pattern search
envcat .env --filter PORT

# Custom secret detection regex
envcat .env --custom-secret "(INTERNAL_|PRIVATE_)"
```

---

## 📊 Secret Detection Mechanisms

| Detector | Pattern / Criteria | Masking Action |
| :--- | :--- | :--- |
| **Keyword Heuristics** | `secret`, `password`, `token`, `auth`, `api_key`, `credential`, etc. | `yo******re` |
| **Database URI Credentials** | `protocol://user:pass@host:port/db` | `postgres://user:******@host/db` |
| **JWT Tokens** | `eyJ[A-Za-z0-9_-]+\.eyJ...` | `eyJhbGci...[MASKED]` |
| **AWS Access Keys** | `^(AKIA\|ASIA)[0-9A-Z]{16}$` | `AKIA...[MASKED]` |
| **GitHub Tokens** | `^(ghp\|gho\|ghu\|ghs\|ghr)_[a-zA-Z0-9]+$` | `ghp_...[MASKED]` |
| **Slack Tokens** | `^xox[baprs]-[0-9a-zA-Z]+` | `xoxb...[MASKED]` |
| **Entropy Analysis** | Shannon Entropy $> 4.5$ bits/char (len $\ge 20$) | `ab******yz` |

---

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

Includes unit tests, integration tests for all format parsers, secret scrubber algorithms, diff engines, and linter rules.

---

## 📄 License

MIT License. Designed with ❤️ by Antigravity AI.

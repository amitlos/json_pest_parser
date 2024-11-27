# json_pest_parser

**json_pest_parser** is a Rust-based command-line tool that parses JSON files using the **pest** library. This parser converts JSON content into a structured format and can serialize the content back to JSON. It provides clear error messages for invalid inputs, making it a reliable tool for JSON validation and manipulation.

## Overview

This project demonstrates a simple yet robust JSON parser built with **pest**, a parsing expression grammar library for Rust. The parser supports all standard JSON features, including objects, arrays, strings, numbers, booleans, and null values. It also provides a command-line interface for processing JSON files.

---
## Technical Details

1. **Parsing Process**:
   - The JSON grammar is defined in `grammar.pest` and adheres to the JSON specification.
   - The `parse_json_str` function validates and parses the JSON string into a structured `JsonContent` enum.

2. **Serialization**:
   - The `json_content_to_Str` function converts the parsed `JsonContent` back into a JSON-formatted string.

3. **Error Handling**:
   - Handles invalid JSON formats and empty files with descriptive error messages.

## Usage Instructions

To use this tool, clone the repository and build the project using `cargo build`.

After that, you can use next commands.

### Basic Commands

1. Parse a JSON file:
   ```bash
   cargo run -- <file-path>
   ```
2. To see usage of the program:
   ```bash
   cargo run -- --help
   ```
3. To see the credits:
   ```bash
   cargo run -- --credits
   ```

## Credits

- **Author**: Solovei Tymofii
- **GitHub**: [@amitlos](https://github.com/amitlos)
- **Version**: 1.0

# Rust CSV Reader CLI Tool

Welcome to the Rust CSV Reader CLI Tool! This project is a simple yet effective command-line tool built in Rust for reading CSV files. It utilizes the csv crate to parse and display the contents of a CSV file.

## How to Use

To use the tool, follow these steps:

1. Make sure you have Rust installed on your machine.
2. Clone this repository.
3. Open a terminal in the project directory.
4. Run the following command:

```bash
cargo run
```

### Dependencies

This project relies on the csv crate for handling CSV parsing tasks.

### Usage

- `Reader::from_path` initializes a reader for parsing the CSV content.

### Reading Process

- The tool iterates over the CSV records, handling each row's result, and printing it out.
- In case of an error, it gracefully reports the issue.

## Observations

- The tool prints each row of the CSV file, providing insights into its contents.
- Any parsing errors are displayed to help identify and address potential issues.

Feel free to explore and modify this Rust CSV Reader Tool. Happy parsing!

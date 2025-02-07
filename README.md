# Folder Declutter

Folder Declutter is a command-line tool written in Rust that helps users manage files by deleting those older than a specified number of days.

## How It Works

1. **CLI Arguments:** The tool accepts two parameters: the folder path and the file age threshold (in days).
2. **Cutoff Calculation:** It calculates a cutoff time using the current system time minus the provided number of days.
3. **File Deletion:** It recursively traverses the specified folder, comparing each file's last modified time to the cutoff, and deletes files that are older.

## Usage

```bash
folder-declutter --folder ~/Downloads --older-than-days 7
```

## License

This project is licensed under the [Apache License 2.0 License](LICENSE).

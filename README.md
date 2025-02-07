# Folder Declutter

Folder Declutter is a command-line utility written in Rust that helps you free up disk space by automatically deleting files that haven't been modified in a specified number of days.

> **Supported Operating Systems:**
> Linux and macOS
> *Windows support will be available in version 2.0*

## How It Works

1. **Parameters:** The tool accepts two parameters—the target folder path and the file age threshold (in days).
2. **Cutoff Calculation:** It computes a cutoff time by subtracting the provided number of days from the current system time.
3. **File Deletion:** The tool then recursively scans the folder and deletes files whose last modified time is older than the cutoff.

## Getting Started

### Download and Installation

1. Download the latest version for your operating system from our [releases page](https://github.com/ricardodantas/folder-declutter/releases).

### Scheduling with Cron

To automate the cleanup, add the following entry to your crontab:

```crontab
0 0 * * * folder-declutter --folder ~/Downloads --older-than-days 7
```

This cron setup will run Folder Declutter daily, removing files in the specified folder that are older than seven days.

## Upcoming Features

- **Service Mode:** Run the tool as a background service, eliminating the need for manual cron configuration.
- **GUI Enhancements:** Add support for a system tray/menu bar interface for an improved user experience.

## License

This project is licensed under the [Apache License 2.0](LICENSE).

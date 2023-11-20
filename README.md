# Time Management CLI

**Time Management CLI (tmc)** is a small cli tool for tracking times spent on projects. A project can be started, stopped and a report can be shown for various duration formats. It writes to a plain json file - perfect to store it in e.g. git.

## Usage

```bash
# Start the tracking of a project
tmc start -p my_project

# How is it active?
tmc show-current -p my_project

# Stop a project
tmc stop -p my_project

# After starting and stopping a project a few times, you maybe want a report of how long it got tracked in sum:
tmc report -p my_project -d all-time

# Show help with
tmc --help
```

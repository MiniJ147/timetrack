# TimeTrack
Simple task and work session manager built into your terminal.  
Founded off simplicity and practicality, this project is mean to be lightweight and to the point (no accounts, no over the top UI, etc...).  
 
*(placeholder for gifs when finished)*

**future ideas**  
- (optional) free cloud storage [optional because it would require an account]
- easy customizability 

**open to pull request and feature request =)**

## installation 

`go install github.com/MiniJ147/timetrack@latest`  

## User Guide
### User Commands
- `session, s [arg]`            - Configure sessions 
- `task, t <task-name> <arg>`   - Configure task
- `list, ls [<args>]`           - List functions

### Args
**Session**  
**usage**: `timetrack session <arg>`
- `-s, --start`             - starts a new session or resumes paused session
- `-e, --end`               - ends current session 
- `-p, --pause`             - pauses current session
- `-v, --view`              - prints status of session (time, task, etc...)
- `-m, --message <message>` - adds note to session
- `-t, --time`              - returns current time of session
- `{default}`               - calls --view

**task**    
**usage**: `timetrack task <task-name> <[flag]+arg>`
- `-i, --id`                - search via id instead of name (flag)
- `-n, --new`               - create new task
- `-r, --remove`            - deletes task 
- `-a, --active`            - active task in session (if task is already active it will deactivate it)
- `-c, --complete`          - mark task as complete (if task is already completed it will be marked as incomplete)
- `-e, --edit`              - allows you to edit information about task
- `-v, --view`              - view information about task {default value if no args are present}
- `-m, --message <message>` - adds note to task with session timestamp

**list**  
**usage**: `timetrack list [<args>] [<values>]`
- `-t, --task`                  - searches for task
- `-s, --session`               - searches for sessions
- `-v, --verbose`               - explains all information instead of surface level
- `-l, --limit <N>0>`           - limits result from (1-N)
- `-r, --range <lower,upper>`   - gives back a range between (start,end)
- `-R, --reverse`               - gives result back in reverse order
-  `{default}`                  - shows both task and sessions without a limit or range



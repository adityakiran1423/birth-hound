# birth-hound
<!-- Line break -->
A UNIX daemon written in Rust to keep track of all the birthdays your pea sized brain can't seem to remember

## To Note

* I am not a Rustacean by any means (read the LICENSE)
* Currently, birth-hound works as a standalone program and not a systemd service (am currently learning and working on daemonizing it)
* It checks against a csv file called birth-hound.csv for dates and then matches accordingly via File IO
* Commands like `add`, `delete`, and `edit` will be implemented eventually to edit both dates and names

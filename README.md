# paste
pastebin service inspired by ix.io and 0x0.st
<!-- the service is hosted at paste.jbox.dev -->
[![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](https://github.com/nop/paste/compare)

# Usage
Help information can be found by `GET`ting the server root (`\`) or by reading `/.paste.1`

# Storage
The goal of the service is to store pastes in an external database for a time based on their size.
Currently, pastes are stored in-memory and therefore exist for the duration of the server's run-time.
There is currently no paste expiry implemented, but you can track this feature at <https://github.com/nop/paste/issues/3>.
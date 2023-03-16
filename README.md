# UniMRCP Plug-In prototype
Note: The work in progress.

Assumption: build environment is RPM-based distro with UniMRCP packages `unimrcp-client`, `unimrcp-client-devel`, `unimrcp-server`, `unimrcp-server-devel` already installed.
Development environment is Fedora.

## Current state of work
### Bug at virtual function `stream_read`
In the scenario "synth" server calls `stream_read` infinitely.
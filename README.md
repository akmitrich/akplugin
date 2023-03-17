# UniMRCP Plug-In prototype
Note: The work in progress.

Assumption: build environment is RPM-based distro with UniMRCP packages `unimrcp-client`, `unimrcp-client-devel`, `unimrcp-server`, `unimrcp-server-devel` already installed.
Development environment is Fedora.

# Build instructions (Fedora)
There must be Rust in the environment: <https://www.rust-lang.org/learn/get-started>.

Build the plugin with:
```bash
cargo build --release
```

Then copy `libakplugin.so` from `target/release` to `unimrcp/plugin` (usually you need sudo privilege).

Edit config of the uniMRCP-server, file `unimrcp/conf/unimrcpserver.xml`. Add line:

`<engine id="Ak-Synth" name="libakplugin" enable="true"/>` 

Disable other synth-plugins like this: 

`<engine id="Demo-Synth-1" name="demosynth" enable="false"/>`.

On start the server loads the plugin, you may see in logs something like this
```bash
[INFO]   Load Plugin [Ak-Synth] [/opt/unimrcp/plugin/libakplugin.so]
```

## Current state of work
~~### Bug at virtual function `stream_read`~~
~~In the scenario "synth" server calls `stream_read` infinitely.~~

### Plugin works correctly
But does not do any actual synthesize. Client gets empty file.
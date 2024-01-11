# Corte

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

Program to control battery charge threshold for ASUS laptops using Linux and tries to replicate on Linux the functionality that [Asus Battery Health Charging](https://www.asus.com/ar/support/FAQ/1032726/) implements in Windows.

## Installation
Clone the repo and run:

```
just build && just install
```

This will:
- The build the program and move the binary to `/usr/bin/corte`
- Copy a [tmpfile](https://www.freedesktop.org/software/systemd/man/latest/tmpfiles.d.html) to `/usr/lib/tmpfiles.d/corte.conf`
- Copy a systemd service to `/usr/lib/systemd/user/corte.service` and start it.

## Uninstallation
Just run:

```
just uninstall
```
This will remove the binary, the tmpfile.d config file and the systemd service. 

## Usage
### Daemon
If you aren't using the service, you must run: 

```
corte daemon
```

### Clients
To modify the battery charge threshold, you have two options `tui` and `gui`.


#### TUI
```
corte tui
```
This opens a tui menu with the options to modify the battery limit.

#### GUI
```
corte gui
```

This is not implemented, but in the future will open a gui app that will allow you to select the limit. 

## TODO
- [ ] Create a man page.
- [ ] Implement the gui, probably with [Iced](https://github.com/iced-rs/iced).

> [!WARNING]
This software is provided without any warranty. USE AT YOUR OWN RISK.

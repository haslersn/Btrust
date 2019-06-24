# btrust

btrust automatically sets the trust attribute for every paired Bluetooth device.

This is done by continually checking if there are any devices that are _paired and not trusted_.
It seems weird to do it like this, so let me know if you know about a better way.
I'd be happy if another solution made btrust obsolete.

## Precompiled Binaries

See [Releases](https://github.com/haslersn/btrust/releases).
Currently, I only have a precompiled binary for aarch64.

## Building from Source

With [Nix](https://nixos.org/nix):

```sh
$ git clone https://github.com/haslersn/btrust
$ cd btrust
$ nix build
```

If you don't have Nix, you need to install the following dependencies.

* [cargo](https://crates.io/install)
* dbus-glib (For Ubuntu and Debian, the package is called `dbus-glib`)

Then:

```sh
$ git clone https://github.com/haslersn/btrust
$ cd btrust
$ cargo build
```

Afterwards you can take the binary and put it whereever you like, or use
[cargo install](https://doc.rust-lang.org/cargo/commands/cargo-install.html#cargo_install_description).

## Cross Compiling

Theretically, cross compiling (with [Nix](https://nixos.org/nix)) should work as follows.

```sh
$ git clone https://github.com/haslersn/btrust
$ cd btrust
$ nix build pkgsCross.aarch64-multiplatform.btrust \
    -f '<nixpkgs>' \
    --arg overlays '[ (_: super: { btrust = super.callPackage ./. {}; }) ]'
```

However, I get the following error:

```
builder for '/nix/store/6h1s8a08nwzhxzprddlf7dynzb8mrfvg-perl5.28.1-TermReadKey-2.37-aarch64-unknown-linux-gnu.drv' failed with exit code 2; last 10 log lines:
   #   define WIDEST_UTYPE U64
   
  rm -f blib/arch/auto/Term/ReadKey/ReadKey.so
  aarch64-unknown-linux-gnu-gcc  -shared  ReadKey.o  -o blib/arch/auto/Term/ReadKey/ReadKey.so  \
        \
    
  chmod 755 blib/arch/auto/Term/ReadKey/ReadKey.so
  "/nix/store/mmkw5mk739wqwx391l6ldkl19nw6g7c9-perl-5.28.1-aarch64-unknown-linux-gnu/bin/perl" "-Iblib/arch" "-Iblib/lib" ReadKey_pm.PL ReadKey.pm
  /nix/store/cinw572b38aln37glr0zb8lxwrgaffl4-bash-4.4-p23/bin/bash: /nix/store/mmkw5mk739wqwx391l6ldkl19nw6g7c9-perl-5.28.1-aarch64-unknown-linux-gnu/bin/perl: cannot execute binary file: Exec format error
  make: *** [Makefile:534: ReadKey.pm] Error 126
cannot build derivation '/nix/store/rqrl0a69fcfzz9p263n13rd1sq3x1fdx-git-2.19.2-aarch64-unknown-linux-gnu.drv': 1 dependencies couldn't be built
cannot build derivation '/nix/store/q0irm3xlw5nan00m81xgicnc7ll236w2-btrust-unstable-aarch64-unknown-linux-gnu.drv': 1 dependencies couldn't be built
[141 built (1 failed), 275 copied (1150.5 MiB), 949.2 MiB DL]
error: build of '/nix/store/q0irm3xlw5nan00m81xgicnc7ll236w2-btrust-unstable-aarch64-unknown-linux-gnu.drv' failed
```

You can try to replace `aarch64-multiplatform` by
[any platform supported by nixpkgs](https://github.com/NixOS/nixpkgs/blob/master/lib/systems/platforms.nix).

## Configuring and Running

**btrust needs to be run as a user that's in the group `bluetooth`.**

When run, btrust looks for a file called `btrust.toml` inside the working directory
(i.e. the directory from which btrust has been run).
`btrust.toml` is a TOML file of the following form:

```toml
adapter_alias = "My Bluetooth Adapter"
```

This allows you configure the name of the Bluetooth adapter, visible to other Bluetooth devices.
**If you don't want btrust to touch it, then just leave `btrust.toml` empty.**

## Running as a systemd Service

The following systemd configuration assumes that:

* your btrust executable is at `/usr/local/bin/btrust`,
* your configuration file at `/etc/btrust.toml`,
* the user `btrust-user` is in the group `bluetooth`.

```
[Unit]
Description=Btrust
After=bluetooth.service
Wants=bluetooth.service

[Service]
ExecStart=/usr/local/bin/btrust
WorkingDirectory=/etc
User=btrust-user
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

## Contributing

You can contribute to btrust by creating an issue or a pull request.

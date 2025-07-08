# Guest Users Module for Linux

[Installation](#installation) | [Configuration](#configuration) | [Dev Setup](#development-setup) | [Architecture](#architecture)

This project offers a guest user support for Linux devices using the [PAM framework](https://github.com/linux-pam/linux-pam) as well as the [GNU nss framework](https://www.gnu.org/software/libc/manual/html_node/Name-Service-Switch.html).

It offers a username (specified via `guest_username_new_user`, default `guest`), which creates a new user on the fly for a guest account. Per guest login a new user account will be created in order to achieve separation of guest users. These accounts will be hidden from most GUI parts of the system, but will exist as long as the guest-users package is installed in order to prevent uid/gid re-use.

To log in, just click on the Guest user entry on the login screen or use `guest` as username (configurable via `guest_username_new_user`):

![Gnome login screen with guest-users installed](docs/login_screenshot.png)

## Installation

This project is currently tested with Ubuntu 24.04 (noble), 22.04 (jammy) and Debian Bookworm. Both `amd64` and `arm64` architectures are supported.

### From repo

Depending on your distribution, you can add an apt repository like this:

Ubuntu 24.04 (noble):

```bash
echo "deb [trusted=yes] https://mixxplorer.pages.rechenknecht.net/guest-users/packages ubuntu-noble stable" > /etc/apt/sources.list.d/guest-users.list
```

Ubuntu 22.04 (jammy):

```bash
echo "deb [trusted=yes] https://mixxplorer.pages.rechenknecht.net/guest-users/packages ubuntu-jammy stable" > /etc/apt/sources.list.d/guest-users.list
```

Debian 12 (Bookworm)

```bash
echo "deb [trusted=yes] https://mixxplorer.pages.rechenknecht.net/guest-users/packages debian-bookworm stable" > /etc/apt/sources.list.d/guest-users.list
```

Afterwards, you can install the corresponding packages:

```bash
apt-get update
apt-get install --install-recommends guest-users
# If you want to have a GUI warning message when a guest user gets logged in
apt-get install guest-users-guest-warning
# If you have snapd running on your machine and your guest users should be able to use it, use
apt-get install guest-users-snap-tricks
```

### From dev

You can get the latest Debian packages from the pipeline (`build-rs` step). You just have to install all packages.

Check out the building section for more information.

## Configuration

Once installed, you can configure the module by placing a [toml](https://toml.io) file to `/etc/guest-users/settings.toml`.

You can set the following configuration options:

| Option | Default value | Description |
|:------:|:-------------:|:-----------:|
| `guest_username_new_user` | `guest` | The username, which can be used to create a new guest user during login |
| `guest_username_prefix` | `guest` | A prefix all guest usernames are prepended with |
| `guest_username_human_readable_prefix` | `Guest` | A prefix all human readable guest usernames are prepended with |
| `guest_group_name_prefix` | `guest` | A prefix all guest group names are prepended with |
| `enable_guest_common_group` | `false` | Whether a common group for all guest users is enabled |
| `guest_common_group_name` | `guest-users` | A group name for the common group, which all guest users are member of |
| `guest_common_group_gid` | `31001` | A gid of the group name of the common group, which all guest users are member of |
| `home_base_path` | `/home/guest-users` | Base path for guest home directories. If it is outside `/home`, snap will not work with default (our) configuration. |
| `home_skel` | `/etc/skel` | Skeleton home directory being copied to every new guest user |
| `guest_shell` | `/bin/bash` | Shell, which will be used for all guest users |
| `public_database_path` | `/etc/guest-users/public.db` | Database path for guest users (sqlite) |
| `uid_minimum` | `31010` | Minimum UID for guest users (make sure these IDs are and will be really available) |
| `uid_maximum` | `31999` | Maximum UID for guest users (make sure these IDs are and will be really available) |
| `gid_minimum` | `31010` | Minimum GID for individual default groups of guest users (make sure these IDs are and will be really available) |
| `gid_maximum` | `31999` | Maximum GID for individual default groups of guest users (make sure these IDs are and will be really available) |
| `guest_user_warning_app_name` | `Guest User` | App name shown in notifications starting with Gnome 46 |
| `guest_user_warning_title` | `You are using a guest account` | Title of warning message guest users are shown after logging in |
| `guest_user_warning_body` | `All data will be deleted on logout. Make sure to store your data on a safe location apart from this device.` | Body of warning message guest users are shown after logging in |
|`enable_ghost_user` | `true` | Whether to enable a ghost user which will be shown e.g. on login screens |
| `ghost_user_gecos_username` | `Guest` | The name the user will be shown on login screen |
| `ghost_user_uid` | `31000` | UID for ghost user (make sure this ID is and will be available) |
| `ghost_user_gid` | `31000` | GID for ghost user (make sure this ID is and will be available) |

When you change ghost user related settings, it is necessary to either reboot the machine or alternatively run `guest-users-sync-accountsservice` manually.

## Useful tips

Guest user session do have the `IS_GUEST_USER` env set to `true` in order to enable a guest user detection for e.g. sessions scripts.

### Limitations

Currently, only single-seat systems (like traditional notebooks, desktop PCs) are supported. See [Architecture](#architecture) for more details.

If guest users can write files beyond the scope of the local system, please take proper measures so other guest users (on other systems) might only see what they should be able to see as different guest users might share the same user id and group id.

## Development setup

For building the project, it is required to have some development dependencies. On Debian/Ubuntu you can install them via apt:

```bash
apt-get install libsqlite3-dev dpkg dpkg-dev liblzma-dev libclang-dev libpam-dev libnss3-dev
```

Before you can start developing, it is necessary to initialize the database. Therefore, it is required to install the diesel CLI:

```bash
cargo install diesel_cli --no-default-features --features "sqlite"
```

### Database setup

To initialize the database, just run

```bash
diesel migration run --database-url=guest_users.db --config-file guest-users-lib/diesel.toml --migration-dir guest-users-lib/migrations
```

### Building

This project makes use of the cargo package manager. To build, just execute `cargo build` at the top level directory after initializing the database via `diesel migration run` (see above).

To build all the Debian packages, you have to install the `cargo-deb` tool:

```bash
cargo install cargo-deb
```

Then, you can make use of this one-liner to build all Debian packages:

```bash
cargo deb -p guest-users-pam && cargo deb -p guest-users-nss && cargo deb -p guest-users-lib && cargo deb -p guest-users-sync-accountsservice && cargo deb -p guest-users-guest-warning && cargo deb -p guest-users-cleanup-daemon
```

The resulting `.deb` packages are then built in `target/debian` and can be installed with `dpkg`.

## Architecture

This section describes a few architectural decisions taken during development of this package. This section shouldn't be necessary to read for using the package, but might be helpful before touching the code.

### Guest users lifetime

#### Guest user creation

Guest users are created on the fly on login of new guest users. Therefore, after installation no "real" guest user exists.
The user, which is shown is the `guest` user, specified via `guest_username_new_user`. It is a user, only consisting of a user name (no home directory etc.).

This `guest_username_new_user` username will be matched by the PAM guest-users module and if the name matches, it will create a new user.
PAM does support replacing the user name during login. And that is what the the PAM guest-users module does. It replaces the `guest_username_new_user` user name with the newly generated guest user (with a different name).

With this approach, it is possible to isolate different guest sessions from each other by providing different users to them.

Therefore, a typical login might look like this:

* User logs in via ghost user (by default `guest`)
* PAM module gets invoked
  * Creates new `guest-$id` user
  * replaces the username `guest` with `guest-$id`
  * PAM module returns
* User gets logged in

Using this approach, even a re-login is possible by using the `guest-$id` user specifically. However, the PAM module denies such login if no session is open for the user and/or the system got restarted.

#### Guest user re-login

For various occasions (like a locked session), a guest user must be able to re-authenticate. As a guest user does not have any password, the authentication is performed without any credential. This might impose an issue on thin client / multi-seat systems, which is the reason why this package currently supports only one-seat systems.

A re-login is permitted as long as the guest user session is active and the system did not get rebooted in after user creation.

#### Guest user removal

Currently, guest users will only be disabled but not removed. Guest users might have created some resources with their user ID. To reduce the risk implied by user id or group id re-using, this package does not release any assigned ids.

For specific use cases it might make sense to release ids at some point. E.g. if you reset your systems on a regular basis, you might just delete the database, which also releases all claimed IDs.

The user home directories will be removed by the [guest-users-cleanup-daemon]('cleanup-daemon') once users do not have any processes left.

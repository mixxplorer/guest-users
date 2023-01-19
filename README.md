# Guest Users Module for Linux

This project offers a guest user support for Linux devices using the [PAM framework](https://github.com/linux-pam/linux-pam) as well as the [GNU nss framework](https://www.gnu.org/software/libc/manual/html_node/Name-Service-Switch.html).

It offers a username (specified via `GUEST_USERNAME_NEW_USER`), which creates a new user on the fly for a guest account. Per guest login a new user account will be created in order to achieve separation of guest users.

## Installation

This project is currently tested with Ubuntu 22.04 (jammy) only.

### From repo

Just add a new repository like

```bash
echo "deb [trusted=yes]  https://mixxplorer.pages.rechenknecht.net/guest-users/packages/release/main /" > /etc/apt/sources.list.d/guest-users.list
```

and then you can install the corresponding packages:

```bash
apt-get update
apt-get install guest-users
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
| `guest_group_name_prefix` | `guest` | A prefix all guest group names are prepended with |
| `home_base_path` | `/tmp/guest-users-home` | Base path for guest home directories |
| `guest_shell` | `/bin/bash` | Shell, which will be used for all guest users |
| `public_database_path` | `/etc/guest-users/public.db` | Database path for guest users (sqlite) |
| `uid_minimum` | `31000` | Minimum UID for guest users |
| `uid_maximum` | `31999` | Maximum UID for guest users |
| `gid_minimum` | `31000` | Minimum GID for individual default groups of guest users |
| `gid_maximum` | `31999` | Maximum GID for individual default groups guest users |

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

This project makes use of the cargo package manager. To build, just execute `cargo build` at the top level directory.

To build all the Debian packages, you can make use of this one-liner:

```bash
cargo deb -p guest-users-pam && cargo deb -p guest-users-nss && cargo deb -p guest-users-lib
```

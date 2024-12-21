# Snap Tricks

This module is necessary as snap does not support home directories outside `/home` [out of the box](https://snapcraft.io/docs/home-outside-home).

In our tests, utilizing the [corresponding option](https://snapcraft.io/docs/system-options#heading--homedirs) only works as long the user home directory is really a sub directory of `/home`.

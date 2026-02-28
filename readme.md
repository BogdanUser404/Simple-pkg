## Simple PKG

Simple PKG is a unified command-line frontend for managing packages from **Pacman**, **AUR** (via `yay`), and **Flatpak** repositories.

### Usage
simple-pkg --install <package> Install a package
simple-pkg --remove <package> Remove a package
simple-pkg --update Update all packages from all sources
simple-pkg --add-flatpak-repo <name> <url> Add a Flatpak repository

text

### Examples
```
simple-pkg --install firefox
simple-pkg --update
simple-pkg --add-flatpak-repo flathub https://flathub.org/repo/flathub.flatpakrepo 
```

### Dependencies

- `pacman` (core package manager)
- `yay`(for AUR support)
- `flatpak` (for Flatpak support)

### Installation
```bash
cargo build --release
sudo cp target/release/simple-pkg /bin/simple-pkg
```
### License

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

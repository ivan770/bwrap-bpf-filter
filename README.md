# bwrap-bpf-filter

A simple tool to generate a BPF syscall filter for Bubblewrap.

## Usage

`bwrap-bpf-filter x86-64 output.bpf`

## License

The syscall filter is adapted from Flatpak, a software licensed under LGPL-2.1-or-later.
See [`COPYING`](https://github.com/flatpak/flatpak/blob/main/COPYING) for more information about the licensing terms of Flatpak.

This program is free software; you can redistribute it and/or
modify it under the terms of the GNU Lesser General Public
License as published by the Free Software Foundation; either
version 2.1 of the License, or (at your option) any later version.
This library is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
Lesser General Public License for more details.
You should have received a copy of the GNU Lesser General Public
License along with this library. If not, see <http://www.gnu.org/licenses/>.
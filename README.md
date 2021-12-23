# <img width="5%" src="logo.svg" alt="Proton Caller logo"> Proton-Caller
Run any Windows program through [Valve's Proton](https://github.com/ValveSoftware/Proton).

[Usage](https://github.com/caverym/Proton-Caller#usage)

Please create an issue if you want added features or have an issue.

[FAQ](https://github.com/caverym/Proton-Caller/wiki/FAQ)

## Problem Reporting:
Please create an issue on the [Github](https://github.com/caverym/Proton-Caller) page which lists: system, kernel version, game, shell, and if it is or isn't a Steam game – provide how you had installed it and where it is installed. Additionally provide screenshots of the shell. Try many methods to get it to work and describe what you did in your issue.

### Warning: if you are not using a release, use a release.

## Usage:

Defaults to the latest version of Proton.
```
proton-call -r foo.exe
```

Defaults to the latest version of Proton, all extra arguments passed to the executable.
```
proton-call -r foo.exe --goes --to program
```

`--goes --to program` are passed to the proton / the program

Uses specified version of Proton, any extra arguments will be passed to the executable.
```
proton-call -p 5.13 -r foo.exe
```

Uses custom version of Proton, give the past to directory, not the Proton executable itself.
```
proton-call -c '/path/to/Proton version' -r foo.exe
```

## Config:
Configuration files are extremely simple: `~/.config/proton.conf`  
Set your own path to `data` (any empty directory), `steam`, (the directory steam is installed in), and optionally `common` (steam's common directory).
```
data = "/home/avery/Documents/Proton/env/"
steam = "/home/avery/.steam/steam/"

# optional
common = "/home/avery/.steam/steam/steamapps/common/"
```

## Install:

### Arch Linux:
[Proton-caller](https://aur.archlinux.org/packages/proton-caller) is available as a [package in the AUR](https://aur.archlinux.org/packages/proton-caller).

### Debian-based Distributions:
#### Based on Ubuntu 21.04 or 21.10:
```
sudo add-apt-repository ppa:benthetechguy/proton-caller
sudo apt install proton-caller
```

#### Other:
A `.deb` file is available for download at the [releases](https://github.com/caverym/proton-caller/releases) page.

### RPM-based Distributions:
A `.rpm` file is available for download at the [releases](https://github.com/caverym/proton-caller/releases) page. There is also a [Copr](https://developer.fedoraproject.org/deployment/copr/about.html) repository available for Fedora 34+ users:
```
sudo dnf copr enable benthetechguy/proton-caller
sudo dnf install proton-caller
```

### Other Linux:
An x86_64 Linux binary is available for download at the [releases](https://github.com/caverym/proton-caller/releases) page.

### Compile from source:
```
git clone https://github.com/caverym/proton-caller.git
cd proton-caller
cargo b --release --locked
sudo install -Dm 755 target/release/proton-call /usr/bin/proton-call 
```

## Space Engine example:
   Make a .desktop launcher. [example file](Space%20Engine.desktop)
   
```
[Desktop Entry]
Type=Application
Name=Space Engine
Comment=Space Engine
Exec=proton-call --run SpaceEngine.exe
Path=/home/avery/Documents/games/SpaceEngine/system
Terminal=false
StartupNotify=false
```

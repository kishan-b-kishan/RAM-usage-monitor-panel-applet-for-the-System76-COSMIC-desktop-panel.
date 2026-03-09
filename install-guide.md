# Installation and Setup Guide

This guide explains how to build and install the **RAM Monitor COSMIC panel applet**.
---

# 1. Install Required Dependencies
 terminal: Bash-
 
sudo apt install git pkg-config libinput-dev libudev-dev libdbus-1-dev \
libpipewire-0.3-dev libspa-0.2-dev clang libclang-dev

Why these dependencies are required

These libraries are required because COSMIC applets interact directly with system services.

Dependency	Purpose
git:	Used to clone the COSMIC source repository
pkg-config:	Allows Rust crates to locate system libraries
libinput-dev:	Input device handling used by the COSMIC compositor
libudev-dev:	Device management library used by many system components
libdbus-1-dev:	Enables communication with system services through D-Bus
libpipewire-0.3-dev:	Required for multimedia and system monitoring features
libspa-0.2-dev:	PipeWire plugin API used internally by COSMIC
clang / libclang-dev:	Required by Rust bindgen to generate bindings to system libraries

Without these dependencies, the Rust build process will fail because the required system libraries cannot be linked.

# 2. Clone the COSMIC Applets Workspace
terminal: Bash-

git clone https://github.com/pop-os/cosmic-applets
cd cosmic-applets

Why this is required

COSMIC applets are developed as a Rust workspace.
The workspace contains multiple applets such as network, battery, bluetooth, time,workspaces.
By adding our RAM applet to this workspace, we ensure it builds using the same environment and dependencies as official COSMIC applets.

# 3. Add the RAM Applet Directory

Place the ram-applet folder inside the cosmic-applets directory.

Your directory structure should now look like:
<img width="171" height="784" alt="image" src="https://github.com/user-attachments/assets/db502b25-c283-4096-9ce5-be604e8eccaa" />

# 4. Update the Workspace Cargo.toml

Inside the root of the repository there is a file:
cosmic-applets/Cargo.toml

This is the workspace Cargo.toml which tells Cargo which projects belong to the workspace.

Open it and add the RAM applet to the members section.

Example:

members = [
    "cosmic-app-list",
    "cosmic-app-list/cosmic-app-list-config",
    "cosmic-applets",
    "cosmic-applet-audio",
    "cosmic-applet-battery",
    "cosmic-applet-bluetooth",
    "cosmic-applet-minimize",
    "cosmic-applet-network",
    "cosmic-applet-notifications",
    "cosmic-applet-power",
    "cosmic-applet-status-area",
    "cosmic-applet-tiling",
    "cosmic-applet-time",
    "cosmic-applet-workspaces",
    "cosmic-panel-button",
    "cosmic-applet-input-sources",
    "cosmic-applet-a11y",
    "cosmic-applets-config",
    "ram-applet",
    ]
    
Why this step is necessary

Cargo workspaces only compile packages listed in the members section.
If the RAM applet is not added here, Cargo will not detect it as part of the workspace and the build command will fail.

# 5. Verify the RAM Applet Cargo.toml

Inside the ram-applet directory there is another file: ram-applet/Cargo.toml
This file defines the applet package itself.

Ensure the name written within the package is:
name = "cosmic-applet-ram"

Why the package name matters:
COSMIC expects panel applets to follow a naming convention:
cosmic-applet-*
Using this naming format ensures the applet integrates properly with the COSMIC panel system.

# 6. Build the Applet

Run the following command from inside the cosmic-applets directory:
Terminal: Bash-
cargo build -p cosmic-applet-ram --release

Why we build using the workspace
Building through the workspace ensures the RAM applet:
uses the same dependency versions
links correctly with libcosmic
compiles using the same configuration as official applets

The compiled binary will appear in- target/release/cosmic-applet-ram

# 7. Install the Binary

Copy the compiled binary to a system executable directory:
Terminal: Bash-
sudo cp target/release/cosmic-applet-ram /usr/bin/
sudo chmod +x /usr/bin/cosmic-applet-ram

Why /usr/bin

The /usr/bin directory contains system-wide executable programs.
Placing the binary here allows COSMIC to launch the applet using the command: cosmic-applet-ram

# 8. Register the Applet with COSMIC

Create a desktop entry:
Terminal: Bash-
sudo nano /usr/share/applications/cosmic-applet-ram.desktop

Add the following content:

[Desktop Entry]
Name=RAM Monitor
Comment=Shows RAM usage in the COSMIC panel
Exec=cosmic-applet-ram
Type=Application
Categories=COSMIC;Applet;
NoDisplay=true
X-CosmicApplet=true

Why this file is required
COSMIC does not automatically detect binaries as panel applets.
Instead, it scans desktop entry files located in:
/usr/share/applications

Each applet registers itself using a desktop entry with:
X-CosmicApplet=true

This flag tells the COSMIC panel that the application should appear in the Add Applet menu.

# 9. Restart the COSMIC Panel

Restart the panel so it reloads available applets:
Terminal: Bash- 
pkill cosmic-panel
cosmic-panel &

But Honestly I would prefer to restart the system to ensure everything restarts and nothing fails in the process.

# 10. Add the Applet to the Panel

Open:
Settings → Desktop → Panel → Add Applet
Search for:
RAM Monitor
Add it to the panel.

# Verification

You can confirm the applet works by running:
Terminal: Bash-
cosmic-applet-ram

This will launch the applet window independently.
When launched by the COSMIC panel it will appear directly inside the panel instead.

# Troubleshooting

If the applet does not appear:
Check the binary exists:
Terminal: Bash-
which cosmic-applet-ram
 
 or
 
Verify the desktop entry:
Terminal: Bash-
cat /usr/share/applications/cosmic-applet-ram.desktop

Restart the COSMIC panel again.

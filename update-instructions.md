With the updated windows.rs another small chnage required is to add a dependancy in Cargo.toml file of the ram-applet folder.
To Add dependency:

Open your ram-applet/Cargo.toml
Add:
[dependencies]
sysinfo = "0.30"

After updating and Cargo.toml and replacing the windows.rs file-
1 Terminal:cargo build -p cosmic-applet-ram --release
To make an updated binary file.

2 Terminal:pkill -f cosmic-applet-ram
This will stop the already existing and running panel.

3 Terminal:sudo cp ~/cosmic-applets/target/release/cosmic-applet-ram /usr/bin/

4 Restart Panel either by restarting the system or by 
Terminal:pkill cosmic-panel
         cosmic-panel &

You can successfully see the new updated applet up and running.

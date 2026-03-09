# RAM-usage-monitor-panel-applet-for-the-System76-COSMIC-desktop-panel.
---
A lightweight RAM usage monitor panel applet for the **System76 COSMIC Desktop Environment**.
This applet displays real-time memory usage directly in the COSMIC panel, allowing users to quickly monitor system memory without opening additional tools.
The applet is written in **Rust** and integrates with the **libcosmic** toolkit used by the COSMIC desktop.
---

## Features

- Displays live RAM usage in the COSMIC panel
- Lightweight and minimal
- Native integration with COSMIC panel
- Built using Rust and libcosmic
- Simple installation process
- Open source and customizable

Example output:

<img width="1920" height="48" alt="image" src="https://github.com/user-attachments/assets/8e19e987-42fa-45ca-ae94-2d1b51ad509d" />

## How it Works

The applet runs as a small COSMIC panel component that periodically reads system memory usage from the Linux system and updates the panel display.

It integrates with COSMIC using the **CosmicApplet framework**, which allows applications to register themselves as panel widgets.

---

## Requirements

- COSMIC Desktop Environment
- Rust toolchain
- Cargo
- libcosmic dependencies
- Linux system (tested on Pop!_OS 24.04 LTS x86_64 COSMIC)

---

## Installation

Detailed installation steps are provided in:
install-guide.md
---

## Customization

Since the project is written in Rust, you can easily modify:

- refresh interval
- RAM display format
- add percentage display
- add color indicators
- add CPU usage monitoring
- add GPU monitoring

---

## Project Structure


<img width="175" height="240" alt="image" src="https://github.com/user-attachments/assets/ea22eeed-faf3-482f-b317-1bd22cfa1d92" />



---

## Future Improvements

Possible enhancements:

- RAM percentage display
- color-coded usage indicators
- CPU usage monitor
- temperature monitoring
- system monitor dropdown menu
- graphical usage bars

---

## Contributing

Contributions are welcome.

If you would like to improve the applet or add new system monitoring features, feel free to submit a pull request.

---

## License

GPL-3.0 (same as COSMIC components)

---

## Credits

Built using:

- System76 COSMIC Desktop
- libcosmic toolkit
- Rust programming language

Inspired by existing COSMIC panel applets.

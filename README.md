# waybar-modules
A collection of my custom waybar modules


## Mullvad
The "Mullvad" module is a custom Waybar module for monitoring and toggling the status of the Mullvad VPN service. 

Depends on [mullvad-vpn](https://aur.archlinux.org/packages/mullvad-vpn) 

### Functionality
The "Mullvad" module offers the following functionality:

- Display the current Mullvad VPN status (connected or disconnected).
- Toggle the connection status of Mullvad from Waybar.
- Provide a tooltip with information about the Mullvad connection.

### Usage
To use the "Mullvad" module, follow these steps:

1. Build the project.

   ```bash
   cargo build --release
   ```

2. Execute from your waybar.

    Example:

    ```{jsonc}
    "custom/mullvad": {
        "exec": "path_to_your_mullvad_binary",
        "return-type": "json"
        "format": "{icon}",
        "format-icons": {
            "on": "󰌆",
            "off": "󰌊"
        },        "return-type": "json",
        "tooltip": true
        "on-click": "path_to_your_mullvad_binary -t"
        "on-click-right: "mullvad-vpn"
    }
    ```

    Please replace "path_to_your_mullvad_binary" with the path to the built Mullvad module binary.


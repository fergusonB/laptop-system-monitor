

# Laptop System Monitor
A system monitor for laptop users.

![Linux, macOS, Windows](https://shields.io/badge/platform-windows%20%7C%20macos%20%7C%20linux-lightgrey)



## Development
### Overview
* This project is built with [Tauri](https://tauri.studio/docs/getting-started/prerequisites)
* Cross-platform system data is collected with the rust crate [sysinfo](https://docs.rs/sysinfo/latest/sysinfo/index.html). `/src-tauri/src/get_data.rs`
* The UI is built with [svelte](https://svelte.dev/).  `/src/App.svelte`
### Commands
After installing the requirements at the links above:
* `yarn install` - Install dependencies
* `yarn tauri dev` - Starts the development server.
* `yarn tauri build` - Builds the application.
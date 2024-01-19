# StoryTree: Native

This crate includes logic for creating windows and making sys calls. It has an API that maps to the appropriate platforms
libraries. The base crate will give access to creating a windows, themed title bars, light and dark mode background colors,
window/system events, I/O events. On top of this basic modals and selector APIs are provided. The idea is that the base
crate serves to provide a base window for other libraries such as those that use graphics APIs to render to the windows.

Additionally, to begin with this crate will provide an API for systray utilities and other useful features including full
application generation using native system API calls.

## Features:

### Create a Window
  - [x] Windows
  - [ ] Linux
  - [ ] Macos

### Window Controls
- [ ] Fullscreen
- [ ] Minimize
- [ ] Maximize
- [ ] Restore

### Modals and Selectors
- [ ] Dialog
  - [x] Window
  - [ ] Linux 
  - [ ] Apple 
- [ ] File Selector
  - [x] Window
  - [ ] Linux
  - [ ] Apple
- [ ] Color Picker 
  - [x] Window
  - [ ] Linux
  - [ ] Apple
- [ ] Font Picker 
  - [x] Window
  - [ ] Linux
  - [ ] Apple

### Menu: (File, Edit, etc...)
  - [ ] Windows
    - Part of caption
  - [ ] Linux
    - Part of caption
  - [ ] Macos
    - Part of top bar/status bar

### Systray Menu creation and customization
  - [ ] Window
    - [so:ref](https://stackoverflow.com/questions/68474486/creating-system-tray-right-click-menu-c)
  - [ ] Linux
  - [ ] Apple

### Events
  - [ ] Keyboard
    - [ ] Window
    - [ ] Linux
    - [ ] Apple
  - [ ] Mouse
    - [ ] Window
    - [ ] Linux
    - [ ] Apple
  - [ ] Actions
    - [ ] Window
    - [ ] Linux
    - [ ] Apple

### Ideas
- Custom title bar
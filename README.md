# Transparent Overlay

## Overview

A Tauri Application displaying a transparent / fullscreen Webpage from a link provided by the User.

## Install

This application is only available on Windows

### Development

<br>
Install dev dependancies :

```sh
npm install
```

<br>
Build the debug version :

```sh
npm run tauri:dev
```

### Production

<br>
Install prod dependancies :

```sh
npm install --omit=dev
```

<br>
Build the release bundle (win installer) :

```sh
npm run tauri:build
```

## Usage

- Run the .exe to open the configuration window. A URL can be specified to open this website through a transparent / click-through overlay.

- The .exe can be run from the command line, passing the URL as an argument :

```bat
& "./Transparent Overlay.exe" https://example.com
```

- The application runs in background and is accessible from the System Tray, where the webpage can be reloaded or exited.

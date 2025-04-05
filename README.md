# E-Z Uploader (Linux Port)

# THIS IS STILL IN DEVELOPMENT, YOU WILL ENCOUNTER BUGS. REFER TO THE TROUBLESHOOTING SECTION.

Make sure your screenshot app (I use flameshot) saves its screenshots in ~/screenshots/


## Reqs

- pnpm
- node v20.10.0
- rust 1.57

## Dev

```
pnpm install
pnpm tauri dev
```

## Build

```
pnpm install
pnpm tauri build
```

# Setup
When you first open the app, it will require some configuration. Go to the settings page and input your API key, click save, and click the button that mentions finishing initialization. You should be good to go.


# Troubleshooting

## The app doesn't seem to be uploading, how come?
Common fixed to this which I found effective were to restart the app, turn your wifi on and on, and try logging out and back in.

## It's not autostarting!
Create a file in ~/.config/autostart labeled ```E-Z Uploader.desktop``` and paste the following contents in, editing the file to accomodate for your file path:
```
[Desktop Entry]
Type=Application
Version=1.0
Name=E-Z Uploader
Comment=E-Z Uploader startup script
Exec=INSERT PATH TO FILE
StartupNotify=true
Terminal=false
```

## It uploads the image, but doesn't send a notification!
Cause TBD - open a PR if you know.

# oliver
![Logo](https://r2.e-z.host/2082d908-7c65-4fc3-b02a-5f50f9141543/lbo1x6wn.png)

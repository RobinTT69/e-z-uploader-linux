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


# Simple Troubleshooting

## The app doesn't seem to be responding, how come?
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
# Complex Troubleshooting

## It fails to upload!
This can be caused due to one of 3 things:
1. Your api key is wrong - in which case go to settings and put in the right one
2. the upload url is wrong - in which case go to settings and reset the url
3. The application attempts to upload the image before it has finished writing to disk, resulting in 400 and 422 error codes. In which case:

a. **Clone the Repository and Install Dependencies**: Clone the repository and ensure all dependencies are installed.

b. **Modify the Delay in the Code**:
   - Navigate to `src-tauri/src/listeners.rs`.
   - Locate the line immediately following the comment regarding the delay.
   - Increase the delay from 700ms to a higher value.

c. **Rebuild the Application**: After making the changes, rebuild the application to apply the updates.


# oliver
![Logo](https://r2.e-z.host/2082d908-7c65-4fc3-b02a-5f50f9141543/lbo1x6wn.png)

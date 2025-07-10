# Common Key Extractor for all platforms!
A common key extractor for all platforms! (Windows, Linux and macOS)

## Platforms that has been tested

- [x] macOS (macOS Sequoia 15.5 (Host System))
- [x] Windows (Windows 10 (VM))
- [x] Linux (Ubuntu 25.04 (VM))

Further testing will need to be done!

## Usage

- Windows
    - For Windows, you just need to double click the executable and drag and drop your OTP from Explorer or enter manually the path to it.
- macOS and Linux
    - For macOS, in Terminal, in the same directory as your executable, you must execute the following command: <br>
    `chmod +x commonkey_extractor && ./commonkey_extractor`
    Then, you must enter the path to your OTP. You can also drag and drop it from Finder or your File Explorer into the Terminal, it will be a path directly to it.

## Minimum system requirements

This tool has been tested on a Windows 10 22H2 Virtual Machine (VM), Linux Ubuntu 25.04 VM and macOS Sequoia 15.5 (Host) as said in the **Platforms that has been tested** section.

The minimum system requirements are:

- Windows 8.1 or newer
    - **Will work** on Windows mods that are based on Windows 10 or 11 (exemple: Reunion 7)
- macOS 10.14 Mojave
- Any Linux distro version that is still relatively new (newer than 2018)

## Contributing
To Contribute, fork the repo and make a branch using the following scheme:

`<username>/<feature you wanna add>`

Then, make a pull request and I'll see the changes you made.

## To-do list

- [x] Make it functional and usable
- [x] Add a Terminal User Interface (TUI)
- [x] Make a Windows release
- [ ] Perhaps make a GUI too
- [ ] Add more features

## Issues

Any issues? Report them to the Issues tab. Make sure to write your Operating System version and I will test it under a clean fresh install of it. <br> I will try to replicate it. **Make sure to say every detail for best assistance**

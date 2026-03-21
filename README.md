# Wii U Common Key Extractor
A universal tool to extract the Common Key from your `otp.bin` file.

## Features
- **GUI Mode (Default):** A clean, easy-to-use interface for selecting files and viewing the key.
- **Interactive TUI:** A classic terminal interface for those who prefer the command line.
- **Direct CLI:** Pass a file path as an argument for instant results.

## Usage

### Windows
- **GUI:** Just double-click `commonkey_extractor.exe`.
- **Terminal:** Run `.\commonkey_extractor.exe --tui` for interactive mode or `.\commonkey_extractor.exe <path_to_otp>` for direct extraction.

### macOS and Linux
- **GUI:** Navigate to the folder where the executable is located and render it executable by typing this in your terminal window : `chmod +x commonkey_extractor`
  then, you should be able to double click the file.
- **Terminal:** Navigate to the folder where the executable is located and render it executable by typing this in your terminal window: `chmod +x commonkey_extractor`
  then, run `./commonkey_extractor --tui` for interactive mode or `./commonkey_extractor <path_to_otp>` for direct extraction.

## Platforms Tested
- [x] macOS (Sequoia 15.5) x64
- [x] Windows 10 x64
- [x] Linux (Ubuntu 25.04) x64

***Note that the ARM based builds has __NOT__ been tested.***

## Contributing
To Contribute, fork the repo and make a branch using the following scheme:

`<username>/<feature you wanna add>`

Then, make a pull request and I'll see the changes you made.

## To-do list

- [x] Make it functional and usable
- [x] Add a Terminal User Interface (TUI)
- [x] Make a Windows release
- [x] Perhaps make a GUI too
- [ ] Add more features


## Star History

<a href="https://www.star-history.com/?repos=acer51-doctom%2Fcommonkey_extractor&type=date&legend=bottom-right">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/image?repos=acer51-doctom/commonkey_extractor&type=date&theme=dark&legend=bottom-right" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/image?repos=acer51-doctom/commonkey_extractor&type=date&legend=bottom-right" />
   <img alt="Star History Chart" src="https://api.star-history.com/image?repos=acer51-doctom/commonkey_extractor&type=date&legend=bottom-right" />
 </picture>
</a>


## Issues

Any issues? Report them to the Issues tab. Make sure to write your Operating System version and I will test it under a clean fresh install of it. I will try to replicate it. **Make sure to say every detail for best assistance.** <br> Here's a template of what you should do

```
Title : <Insert a small description>

Contents:

CPU Architecture: <ARM or x64>

OS: <Put your OS>

In-depth description:
```

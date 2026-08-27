# layshift
A small Wayland clipboard tool for converting text between keyboard layouts.

![preview](preview.gif)

## How it works
It read the current text from clipboard then map it to the target layout and writes the result back to the clipboard.

```
text -copy/cut-> clipboard -wl-paste-> layshift -wl-copy-> clipboard -paste-> result
```

## Usage
Using layshift is simple, just run the following command to change the text layout:

```
layshift <source_layout> <target_layout>
```

- For faster and easier use, it is recommended to bind layshift to a keyboard shortcut.

### Layouts
Layouts are identified using:

```
<language>:<variant>
```

- If your layout is not supported, just add it as a json file😁️.

```json
{
    "normal": ["`", "1", ...],
    "shift": ["~", "!", ...]
}
```

## Installation
### Install from source code
Clone the repository or download the latest release then Go to the project directory and run this `just` command:

```
just install
```

- You might need `Rust compiler` and `just` installed.
- This method may require superuser privileges.

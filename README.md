
<div align="center">
    <img src="./images/ferris.png" width=250 />
</div>

<div align="center">
    <h1>Image optimization CLI</h1>
</div>

<div align="center">
    <strong>Optimize your images to webp with a simple CLI</strong>
</div>

## Installation

```bash
cargo install pixels-cli
```

## Usage

```bash
pixels-cli --input <input> --rename <filename> --quality <quality>
```

## Options

- `--input` or `-i`: The input file or directory
- `--rename` or `-r`: The new filename, only new filename without extension

- `--quality` or `-q`: The quality of the image, from 1 to 80 (default 80), can be higher until 100, but it's not recommended for small images because the file size will be too big.

## Optional

If the name **pixels-cli** is too long for you, you can create an alias for the command, in this case for bash:

```bash
echo "alias your-alias=pixels-cli" >> ~/.bashrc
```
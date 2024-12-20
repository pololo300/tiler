# Tiler

`tiler` is a Rust tool inspired by [svgtiler](https://github.com/edemaine/svgtiler). It converts a text file into a PDF, mapping characters to colored Tetris pieces as defined in a configuration file (`config.jsonc`). Optional features include rendering a board frame or grid.

## Options:

- `<input_file>`: Required text file with a character grid.
- `c`, `--config <FILE>`: Custom config file (default: config.jsonc).
- `f`, `--frame`: Render a board frame.
- `g`, `--grid`: Render a grid overlay.
- `d`, `--debug`: Enable debug logging (multiple levels).

## Config File

The `config.jsonc` file loads the following colors and parametres:

```jsonc
{
  "cell_size": 30,
  "border_width": 5,
  "separator_width": 2,
  "frame": false,
  "grid": false,
  "colors": {
    "A": "none",
    "B": "hsl(240,75%,50%)", // blue
    "b": "hsl(240,75%,50%)", // blue
    "S": "hsl(240,75%,50%)", // blue
    "s": "hsl(240,75%,50%)", // blue
    "E": "hsl(240,75%,50%)", // blue
    "e": "hsl(240,75%,50%)", // blue
    "R": "hsl(0  ,75%,50%)", // red
    "r": "hsl(0  ,75%,50%)", // red
    "Y": "hsl(60 ,75%,50%)", // yellow
    "y": "hsl(60 ,75%,50%)", // yellow
    "o": "hsl(40 ,75%,50%)", // organge
    "O": "hsl(40 ,75%,50%)", // organge
    "c": "hsl(180,75%,50%)", // cyan
    "C": "hsl(180,75%,50%)", // cyan
    "P": "hsl(280,75%,50%)", // purple
    "p": "hsl(280,75%,50%)", // purple
    "G": "hsl(120,75%,50%)", // green
    "g": "hsl(120,75%,50%)", // green
    "i": "#778899", // gray
    "I": "#778899", // gray
    "d": "#2d3133", // dark gray
    "D": "#2d3133", // dark gray
    "w": "#ffffff", // white
    "W": "#ffffff", // whte
    " ": "white",
    "X": "none",
  },
}
```

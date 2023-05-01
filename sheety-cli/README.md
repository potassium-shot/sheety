# Sheety

Sheety is a sprite-sheet manipulation program written in Rust.

## Usage

There are 4 commands in sheety.
 - cat: allows concatenating, merging several sprite sheets together
 - slc: allows slicing a sprite sheet to extract juste one or more consecutive images from it
 - del: allows removing one or more consecutive images from a sprite sheet
 - rev: allows reversing the order of images of a sprite sheet

### The `cat` command

`sheety cat -S 128x96 -i image1.png -i image2.png -i image3.png`

This will concatenate image1.png, image2.png and image3.png, considering a sprite size of 128 by 96 pixels.
If you wish define the sprite size individually per image, do this instead:

`sheety cat -i image1.png -s 128x96 -i image2.png -s 128x96 ...`

You can also define the number of cells in the sheet, using a `-`:

`sheety cat -i image.png -s 3-4`

Or, you can include single images with `single`

`sheety cat -i single_image.png -s single`


You can define a custom distribution of sprites with the `-d` option. `-d "columns <num>"` and `-d "lines <num>"` will set the amount of columns, and lines, to `<num>`, respectively. `-d "packed columns/lines"` packs the sprites together, favoritising column/line length, respectively. The default, if not specified, is `-d "packed columns"`.

### The `slc` and `del` commands

`sheety slc 4-8 -i image.png -s 4-6 -d "packed lines"`

This will only keep images 4 through 8 (8 excluded) of image.png, distributing the result with `"packed lines"`.

`sheety del 4-8 -i image.png -s 4-6 -d "packed lines"`

This will do the same, but delete images 4 through 8 instead of keeping only them.

`sheety del 7- -i image.png -s 4-6`

If no end point is specified, images up to the end will be affected.

### The `rev` command

`sheety rev -i image.png -s 100x100`

This will reverse the order of sprites in image.png.

## Todo:
- [x] API
	- [x] reordering of sprites
	- [x] sprite-sheet concatenation
	- [x] sprite import
	- [x] sprite-sheet import
	- [x] individual sprites export
	- [x] sprite-sheet export
	- [x] documentation
- [x] CLI
- [x] publish on crates.io
- [ ] GUI (maybe)

# Contributing

## Grab submodules

```
git submodule init
git submodule update
```

## Submitting images through the CLI

Once you have the project cloned (with submodules), use `cargo run --bin ferret_image_cli` to begin submitting images.

For example, here's the command to download an image from a URL for submissions:

```sh
cargo run --bin ferret_image_cli -- create <url>
```

To ensure that your image is under 300kb, go to the directory changed, and run `du -h <file>`. Please make sure to [Optimize your images](#image-optimization).

Once you have made your contribution, make a commit, and inside the `ferret_images` directory, if you're working as a submodule, run `git push origin HEAD:<branch>`. Once you're done, make a pull reqest!

## Image optimization

Install [jpegoptim](https://github.com/tjko/jpegoptim), and if you (most likely) want to resize, [imagemagick](https://imagemagick.org/index.php):

```sh
sudo apt install jpegoptim imagemagick
```

Then, use it to optimize images:

```sh
jpegoptim --dest image.jpg --overwrite --size 300 -strip--all
```

### Still too big

If your files are too big, downsize them, and run the previous command again.

(How to resize in CLI:)

```sh
# you can get the file size using the "file" command:
file image.jpg

# then convert it -- replace <width> and <height> with your preferred width and height. make sure it maintains aspect ratio!
convert image.jpg -resize <width>x<height> image.jpg
```

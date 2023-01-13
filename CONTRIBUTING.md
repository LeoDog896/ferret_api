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
cargo run --bin ferret_image_cli -- create --source <url>
```

To ensure that your image is under 300kb, go to the directory changed, and run `du -h <file>`. If it's not, you can [Optimize your images](#auto-image-optimization).

## Auto-image optimization

If you want control over your optimization, install [jpegoptim](https://github.com/tjko/jpegoptim):

```sh
sudo apt install jpegoptim
```
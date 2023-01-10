# ferret_api ![made with love <3](https://img.shields.io/badge/made%20with-love-ff69b4?style=flat-square) 

a little API for fetching the snoodliest of ferrets (all ferrets are welcome!)

Want to add images instead? Use the [`ferret_images`](https://github.com/LeoDog896/ferret_images) repository instead for contribution. Image contribution guidelines are available there, or with the `ferret_image` cli.

## Module Layout

there are five modules: `ferret_api`, `ferret_server`, `ferret_image`, `ferret_image_cli`, and `ferret_cli`:

### External modules:
* `ferret_api` (TODO) is a rust library for interacting with the ferret RESTful API
* `ferret_cli` (TODO) is a command line interface for interacting with the ferret RESTful API

### Internal modules:
* `ferret_server` (TODO) exposes a RESTful API for retrieving ferrets given different search parameters
* `ferret_image` (Unstable) is the shared library for generating and maintainig the structure for ferret images
* `ferret_image_cli` (Partial) is a command line interface for generating and verifying ferret images. The verification functionality is used for the CI pipeline in order to verify new PRs.
* `www` (TODO) the website deployed to github pages.

## Image structure

`/images/<uuid>` is the directory structure for a ferret image. The directory contains the following files:

* `image.json` is the metadata for the ferret image
* `image.png` is the image of the ferret

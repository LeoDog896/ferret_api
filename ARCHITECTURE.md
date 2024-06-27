# Architecture

Overview of the architecture of the `ferret_api` project.

## Module Layout

### External modules

* `ferret_api` (TODO) is a rust library for interacting with the ferret RESTful API
* `ferret_cli` (TODO) is a command line interface for interacting with the ferret RESTful API

### Internal modules

* `ferret_server` (TODO) exposes a RESTful API for retrieving ferrets given different search parameters
* `ferret_image` (Unstable) is the shared library for generating and maintaining the structure for ferret images
* `ferret_image_cli` (Unstable) is a command line interface for generating and verifying ferret images. The verification functionality is used for the CI pipeline in order to verify new PRs.
* `www` (Partial) the website deployed to GitHub pages.

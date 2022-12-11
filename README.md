# ferret_api

a lil api for collecting the noodelest of ferrets (all ferrets are welcome!)



## module layout

there are five modules: `ferret_api`, `ferret_server`, `ferret_image`, `ferret_image_cli`, and `ferret_cli`:

### External modules (TODO):
* `ferret_api` is a rust library for interacting with the ferret RESTful API
* `ferret_cli` is a command line interface for interacting with the ferret RESTful API

### internal modules:
* `ferret_server` exposes a RESTful API for retrieving ferrets given different search parameters
* `ferret_image` is the shared library for generating and maintainig the structure for ferret images
* `ferret_image_cli` is a command line interface for generating and verifying ferret images. The verification functionality is used for the CI pipeline in order to verify new PRs.
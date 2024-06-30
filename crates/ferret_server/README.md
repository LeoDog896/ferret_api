# ferret_server

Restful API for retrieving ferrets given different search parameters

## Documentation

> **Note**: You can not, with the API, programatically submit ferret photos.
> To submit them, please go to https://github.com/LeoDog896/ferret_images.

Root for the following queries will be in `/v1/`:

`image/uuid/<uuid>`: search a ferret's image by UUID
`image/random`: gets a random ferret's image.

`data/uuid/<uuid>`: search a ferret's image by UUID
`data/random`: gets a random ferret's image.


`list`: get the list of every single ferret UUID
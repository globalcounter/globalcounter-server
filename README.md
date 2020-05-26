# globalcounter-server

The backend for the [Global Counter](https://github.com/globalcounter/GlobalCounter) `Android` app.

## Requirements

It's using [tusk](https://github.com/rliebz/tusk) to run the automated scripts. Install it from the following:

```sh
brew install rliebz/tusk/tusk
```

## Development

Setup the project by installing all the required dev tools:

```sh
tusk setup
```

Start the dev server:

```sh
tusk dev
```

Start the prod server:

```sh
tusk start
```

Deploy the server:

```sh
tusk deploy
```

Please refer to `tusk.yml` file for more commands.
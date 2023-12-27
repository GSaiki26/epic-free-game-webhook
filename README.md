# EpicGames Free Game Webhook
The EpicGames Free Game Webhook is a Rust application that checks every time it runs if some new games has been released on Epic Games Store.
It's webhook request has been based on ``Discord`` embed messages.

## Usage
In order to used it, you may need to check:

### Environment Variables
The template to the environment variables can be found in `.env.example`. Where:
* CATALOG_URL: The URL to Epic's backend catalog, where the application'll extract every information.
* WEBHOOK_URL: The webhook's URL. The project uses Discord's webhook.

### Running
To run the program, you may want to use Docker:
```sh
# Simple docker
docker build -t gsaiki26/epic-freegame-webhook .;
docker run --rm --env-file=.env --name gsaiki26/epic-freegame-webhook gsaiki26/epic-freegame-webhook;

# Docker-compose
docker-compose up --build -f ./docker-compose.yaml;

```

# Epic Free Game Webhook
[![Audit](https://github.com/GSaiki26/epic-free-game-webhook/actions/workflows/audit.yaml/badge.svg)](https://github.com/GSaiki26/epic-free-game-webhook/actions/workflows/audit.yaml) [![Docker build](https://github.com/GSaiki26/epic-free-game-webhook/actions/workflows/docker-build.yaml/badge.svg)](https://github.com/GSaiki26/epic-free-game-webhook/actions/workflows/docker-build.yaml) [![Docker Hub](https://github.com/GSaiki26/epic-free-game-webhook/actions/workflows/docker-push.yaml/badge.svg)](https://github.com/GSaiki26/epic-free-game-webhook/actions/workflows/docker-push.yaml) [![Linter](https://github.com/GSaiki26/epic-free-game-webhook/actions/workflows/linter.yaml/badge.svg)](https://github.com/GSaiki26/epic-free-game-webhook/actions/workflows/linter.yaml)

The Epic Free Game Webhook is a Rust application that checks every time it runs if some new games has been released on Epic Games Store.


## Webhooks supports
You can simply implement a webhook support using the webhook trait defined in `./src/webhooks/webhook.rs`.

The table above lists all the current supported webhooks. The `ID` field is used in the DATAFILE to define the type of the webhook.
<table>
    <tr>
        <th>ID</th>
    </tr>
    <tr>
        <td>Discord</td>
    </tr>
</table>

## Usage
In order to used it, you may need to check:

### Environment Variables
The template to the environment variables can be found in `.env.example`. Where:
* `CATALOG_URL`: The URL to Epic's backend catalog, where the application'll extract every information;
* `DATA_PATH`: The JSON file used to storage data. (if the file not exists, the program'll create a basic template).

The `DATA_PATH` is a json file with the below structure:
```json
{
    "webhooks": {
        "title": "Webhook1",
        "url": "https://webhook_url.com",
        "type": "Discord"
    }
}
```



### Running
To run the program, you may want to use Docker:
```sh
# Simple builded docker
docker build -t gsaiki26/epic-free-game-webhook .;
docker run --rm --env-file=.env -v $(pwd)/cache:/app/cache:rw --name epic-free-game-webhook gsaiki26/epic-free-game-webhook;

# Simple pulled image on Docker hub
docker run --rm --env-file=.env -v $(pwd)/cache:/app/cache:rw --name epic-free-game-webhook gsaiki26/epic-free-game-webhook:latest;

# Docker-compose
docker-compose up --build -f ./docker-compose.yaml;
```

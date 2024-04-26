### Rust TODO-API

### Starting the DB

```sh 
# Start postgresql server docker image:
docker run --rm --name pg -p 5432:5432  -e POSTGRES_PASSWORD=welcome  postgres:15
```

### DEV

``` sh 
    # Terminal  to run server
    cargo watch -q -c -w src/ -w .cargo/ -x "run src"

```
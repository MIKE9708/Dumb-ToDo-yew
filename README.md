# ToDo Yew
This is just a sample ToDo application using yew and a Redis Server as Backend.

## Prerequisite
In order to run this sample ToDo app an istance of a Redis Server running is necessary, the simplest way to get that up and running is by using docker.
To run a container istance of Redis do:

```console
docker run -d -p 6379:6379 redis:latest
```
## How to Run
This repo contain both the yew app and the simple-server that communicate with redis, to run the app first run the server:

```console
cargo run -p simple-server
```
Then on the crate that contains the yew app do:
```console
trunk serve --open
```

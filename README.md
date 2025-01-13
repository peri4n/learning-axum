# Learning Axum

This repository is about learning the Rust [Axum framework](https://docs.rs/axum/latest/axum/).

To begin with, I created two endpoints:

To issue an SQL fetch:

```
curl 'http://localhost:3000/people'
```

To issue an SQL insert:

```
curl -XPOST 'http://localhost:3000/people/gnu'

```

## TODO

- Add telemetrics
- Add more extractor examples
- Add authentication
- Add Open Policy Agent
- Add Docker compose


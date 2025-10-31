# darkicewolf50-actix-setup

Includes a health check function, and a logger

![Workflow](https://forge.ucalgarybaja.ca/darkicewolf50/darkicewolf50-actix-setup/badges/workflows/all_test.yaml/badge.svg?branch=master)
![Open Issues](https://forge.ucalgarybaja.ca/darkicewolf50/darkicewolf50-actix-setup/badges/issues/open.svg)
![GitHub stars](https://img.shields.io/github/stars/darkicewolf50/darkicewolf50-actix-setup?style=flat&compact=true)

# Features

log_incoming, logs the two strings to the terminal, is useful for knowing what was polled in seting and what was polled in the production

health_check, sends back a json stating that the server is alive

# Repository

[https://forge.ucalgarybaja.ca/darkicewolf50/darkicewolf50-actix-setup](https://forge.ucalgarybaja.ca/darkicewolf50/darkicewolf50-actix-setup)

# Install

Add one of these to your toml

Minimal Version

```toml
darkicewolf50-actix-setup = { version = "0.1.4", git = "https://forge.ucalgarybaja.ca/darkicewolf50/darkicewolf50-actix-setup.git" }
```

Version with debug swagger/open api docs only
```toml
darkicewolf50-actix-setup = { version = "0.1.4", git = "https://forge.ucalgarybaja.ca/darkicewolf50/darkicewolf50-actix-setup.git", features = ["debug"] }
```

Version with swagger/open api docs on always
```toml
darkicewolf50-actix-setup = { version = "0.1.4", git = "https://forge.ucalgarybaja.ca/darkicewolf50/darkicewolf50-actix-setup.git", features = ["full"]}
```
# rust-openai
Bindings to [OpenAI Gym/Universe](https://github.com/openai/universe) for Rust language

# Installation

Docker and Rust are required for this client to work

## Linux

###### Docker

[Official](https://www.digitalocean.com/community/tutorials/how-to-install-and-use-docker-on-ubuntu-16-04)

###### Rust

    curl https://sh.rustup.rs -sSf | sh

##  OSX

###### Docker

[Docker](https://docs.docker.com/docker-for-mac/)

###### Rust

    curl https://sh.rustup.rs -sSf | sh

###### Other dependencies

    xcode-select --install
    brew install openssl

You may also need to point to your systems openssl installation:

    export OPENSSL_INCLUDE_DIR=/usr/local/opt/openssl/include
    export DEP_OPENSSL_INCLUDE=/usr/local/opt/openssl/include
    source ~/.bashrc

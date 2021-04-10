# Aiba -- lil CLI password manager


## Overview

Aiba (shortened form of *aikotoba*, the Japanese word for password), is a very basic CLI password manager

It doesn't use encryption, hashing, or anything fancy, so it's not neccessarily secure (other than secuirty protecting your computer), but it works fine for my needs

It's also my first time using Rust, so it's probably written very poorly, haha. I also got tired of working on this so it's not very refactored, and lacks decent error handling

<br>

## Installation

```
$ git clone <THIS_REPO>
$ cd aiba
$ cargo build --release
$ cp target/release/aiba /somewhere/on/your/path
```

<br>

## Usage

Run the `aiba` executable, then check out the help menu by entering `h`
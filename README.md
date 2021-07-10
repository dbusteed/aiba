# Aiba -- Lil CLI Password Manager


## Overview

Aiba (shortened form of *aikotoba*, the Japanese word for password), is a very basic CLI password manager

It doesn't use encryption, hashing, or anything fancy, so it's not neccessarily secure (other than secuirty protecting your computer), but it works fine for my needs

I originally wrote this in Rust (just for the sake of playing with that language), but then rewrote it in Python after I had issues building it on a different system. Either way, it's not very advanced in terms of error handling, but works well for me

<br>

## Installation

```sh
$ git clone <THIS_REPO>
$ cd aiba
$ pip install pyperclip
$ sudo apt install xclip  # required if on Linux
$ chmod +x aiba
$ cp aiba /somewhere/on/your/path
```

<br>

## Usage

Run the `aiba` executable, then check out the help menu by entering `h`
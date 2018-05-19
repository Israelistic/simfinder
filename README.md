# Simfinder [![Build Status](https://travis-ci.org/fvumbaca/simfinder.svg?branch=master)](https://travis-ci.org/fvumbaca/simfinder)

An batch image similarity tool.

## Usage

Create a csv that looks like this:

| image1 | image2 |
|--------|--------|
| img1.png|img2.png|


Then pass the file to the tool with an output file:
```bash
simfinder input.csv output.csv
```

And the output file will look something like:

| image1 | image2 | similarity | elapsed |
|--------|--------|------------|---------|
|img1.png|img2.png|0.56|255|

> Similarity is judged by 0.00 being exactly the same and > 0.00 is different.
> Elapsed is time taken to compare the images in milliseconds.

In the case where image files are all located in a different root directory,
the image flag can be used:
```bash
$ simfinder -i path/to/imgs/ input.csv output.csv
```

## The Install

With a copy of rust, run:
```bash
$ cargo install
```
to build and install a release version of simfinder.

## Uninstall

When your done with all this tool has to offer, just run:
```bash
$ cargo uninstall simfinder
```

## Libraries Used
- **[Image](https://github.com/PistonDevelopers/image)** - The basic image library developed for and used in Piston
- **[Csv](https://github.com/BurntSushi/rust-csv)** - A csv parsing library
- **[Clap](https://github.com/kbknapp/clap-rs)** - Api for building a beautiful cli

## Comparison Algorithm

Suggested for this project was to use either [Image Magick]() or [Graphics Magick](). There are two main reasons I did not use them:

First, it is a pain to integrate into the code. Dealing with compiling source files and managing bindings is not fun - even if a crate can do it for you. It still requires you to install the whole external project, then use a tool to manage the source files. This install is not insignificant and took time and space to download, build, then build into the project - and end users would need to do the same.

Second, it seemed overkill for a project that did not center around the image comparison itself. If the project was particular about the accuracy or method of comparison then the above libraries would be the better choice. For speed of both development, compile time, and runtime anything more than a simple algorithm would be overkill.

Looking for a replacement library there were not too many that did not fall into the same compromise. After a little bit of searching however, I stumbled across [this](https://stackoverflow.com/questions/32680834/how-to-compare-images-with-go) and [this](https://codereview.stackexchange.com/questions/12700/comparing-two-images) (when I was initially intending to write this in go) questions/discussions. A comparison algorithm can be made by simply iterating over each pixel and taking the difference. Essentially thats all I am doing, with a little more math magic to normalize and clean up the numbers.



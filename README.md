# tesseract-sys

Rust bindings for [Tesseract](https://github.com/tesseract-ocr/tesseract). Requires version `4.1.0` or newer.


## Building

This links to the C libraries [leptonica](https://github.com/danbloomberg/leptonica) and tesseract.

The source code for tesseract is embedded in this repo; and it will link to `leptonica-sys`, so you should need to do nothing.

# asc

Utility to print the ASCII table or information about an individual character.

Written in Rust 2018.

## Usage

Install using `cargo install` or use with `cargo run`.

Without any command line argument, the program prints the ASCII table,
with character codes in decimal, hexadecimal, binary and octal, with the
representation or name of the character. The values are output with tab
separator, so that they can be easily parsed with `sed`, `awk` and the like.

Optionally takes one command line argument, which can be either:

* Decimal number from 0 to 127, like `65`
* Hexadecimal number from 0 to 127, with the `0x` prefix, like `0x41`
* Binary number from 0 to 127, with the `0b` prefix, like `0b01000001`
* Octal number from 0 to 127, with the `0o` prefix, like `0o101`

When invoked with one argument, prints the same information as in the table,
but only for the character that has the code specified, like this:

    % cargo run --quiet -- 65
     65	41	01000001	101	A

Returns a standard UNIX exit code to the invoking shell: zero if a valid argument
was supplied, 64 if there was a usage error.

## Why ASCII?

This utility only deals with ASCII characters. Yes, they are legacy. Yes, I know about Unicode.
You can extend it to handle any Unicode codepoint.

I was working with old file formats from pre-Unicode
times, and using `man ascii` was not suitable for a quick reference. Also,
[ascii](https://gitlab.com/esr/ascii) is handy, but I wanted to do something in Rust.

## License

MIT License.

# Rust HTML and XML Parser Comparison

I love Rust's features and performance, and it has a great ecosystem of awesome parsing libraries. However, they are generally focused on Context Free Grammars, rather than the complexities of XML and especially HTML.

There are a ton of XML parsers and fewer HTML ones. This is my attempt at comparing as many of them as possible.

## XML libraries

- [xml-rs](https://github.com/netvl/xml-rs)
- [xmlparser](https://github.com/RazrFalcon/xmlparser)
- [quick-xml](https://github.com/tafia/quick-xml)
- [xmltree](https://github.com/eminence/xmltree-rs)
- [roxmltree](https://github.com/RazrFalcon/roxmltree)
- [minidom](https://docs.rs/minidom/0.12.0/minidom/) (Source [here](https://gitlab.com/xmpp-rs/xmpp-rs))
- [xml5ever](https://github.com/servo/html5ever/tree/master/xml5ever)

There are a few relationships between this libs.

`xml-rs`, `xmlparser`, and `quick-xml` are low-level, [pull-based](https://stackoverflow.com/questions/15895124/what-is-push-approach-and-pull-approach-to-parsing) streaming XML tokenizers. The first two implement Iterator while the latter doesn't.

Both `quick-xml` and `xml-rs` can take input from `BufRead` and `Read`, respectively. Unfortunately, `xmlparser` only takes input from `str`s. Theoretically, it's `Stream` could be implemented to allow for real streaming from those traits, but it isn't for now.

`xml-rs` returns owned values of everything for all tokens. `xmlparser` returns tokens that use references to spans of strings. `quick-xml` uses a custom method to read new events, and pushes event data onto a user-provided buffer, and returns a CoW as the event.

`xmltree` and `roxmltree` are higher-level libaries that ouput an entire XML tree representation into memory.

`xmltree` uses `xml-rs`. `roxmltree` uses `xmlparser`. `minidom` uses `quick-xml`.

I'll refer you to a performance comparison from the `roxmltree` author here: https://github.com/RazrFalcon/roxmltree#performance

### HTML libraries

- [html5ever](https://github.com/servo/html5ever)
- [html-parser](https://github.com/mathiversen/html-parser)

## About the Comparison

In the examples folder, we basically copied the "getting started" example from each library.

In the source of the project, we implemented tests and benchmarks of each of the libraries.

## Current Results

As of 8/10/2020, these are the results for the "low-level" XML parsers:

```
Test Name          xml-rs           xml-parser        quick-xml
basic              SUCCEEDED        SUCCEEDED         SUCCEEDED
basic2             SUCCEEDED        FAILED            SUCCEEDED
invalid            FAILED           FAILED            FAILED
invalid2           FAILED           SUCCEEDED         SUCCEEDED
invalid3           FAILED           SUCCEEDED         FAILED
self_closed        SUCCEEDED        SUCCEEDED         SUCCEEDED
```

## Design choices

The first step I decided to take was to test the forgiveness of the various high-level parsers to somewhat malformatted HTML that could be found in the wild.

If a parser is found which has the features I need and can parse HTML, I would likely not delve any lower and try to write my own system based on tokens.

Also, reading existing comparisons, I had high hopes that `roxmltree` would work as its performance is good. However, a note about `xmlparser`, which it is based on

> quick-xml is faster than xmlparser because it's more forgiving for the input, while xmlparser is very strict and does a lot of checks, which are expensive. So performance difference is mainly due to validation.

## Glossary

"Malformatted HTML" - HTML which is technically valid according to the spec, but which must be error-corrected by the parser. All malformatted HTML is noncompliant XML?

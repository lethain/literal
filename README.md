# Literal

`literal` is a command-line tool to track state and render templates
using that state in text documents. The idea first came from idly wondering
how [LitRPG](https://en.wikipedia.org/wiki/LitRPG) authors maintain state
across the many chapters and novels, but it's intended to be general purpose.

## Build

Because this is in a very rough state, I'm not distributing binaries,
and you'll need to build it yourself.

Once you [have installed Rust](https://www.rust-lang.org/tools/install),
then building should be as easy as:

    git clone $REPO
    cargo build

And your binary will be located at `./target/debug/literal`.

## Usage

To use `literal`, you need two things: a text document that takes advantage of
literal's directives, and
templates (written in [Tera's subset of Django's template language](https://tera.netlify.com/docs/templates/#introduction))
that are used by those directives.

That's a bit abstract, so let's think of an example directory structure:

    book/
    book/text.md
    book/templates/status.txt

There are only two files, `text.md` and `status.txt`.
The first is your actual document, a simple version of which
might be:

    \init exp 0
    \init level 1

    # Hello

    This is the first line of your book.
    \render templates/status.txt

    This is a thing    
    \incr exp 50
    \incr level 1
    \render templates/status.txt

Then `status.txt` might look like

    | Key | Value |
    | --- | --- |
    | **Level**      | {{ level }}       |
    | **Experience** | {{ exp }}         |

Which you'd use `literal` to render:

    ./literal text.md

Resulting in:

    # Hello
    
    This is the first line of your book.

    | Key | Value |
    | --- | --- |
    | **Level**      | 1 |
    | **Experience** | 0   |
    
    
    This is a thing

    | Key | Value |
    | --- | --- |
    | **Level**      | 2 |
    | **Experience** | 50   |

Your full workflow would probably then pass
the output through markdown, maybe using
[pandoc](https://pandoc.org/):

    ./literal | pandoc -f gfm

Which would spit out a well-formatted HTML.
You could also publish to an epub document (readable on iBooks, etc)
via:

    ./literal | pandoc -f gfm -o mybook.epub

And yeah, from there you can do whatever you want.

## Directives

The full set of supported directives are:

* `\init {variable_name} {integer_value}`
* `\incr {variable_name} {integer_value}` - referenced variable must have been initialized previously
* `\render {path/to/template}` - path must be relative to where you're running the binary

## Errors

`literal` takes a fail fast, fail hard approach to errors:
whenever you make a mistake, it will fail hard with error output,
no standard output, and an error-appropriate exit code.
(You really, really don't want error messages in your book.)

## Assertions

You might also want to make assertions about state as you're writing,
to ensure that you don't make continuity errors:

    \init level 1
    \assert level 1
    \incr level 3
    \assert level 4

**Assertion functionality below here is not implemented yet**

If variables supported lists, which is planned functionality,
you could imagine this being more useful, and to be useful in the
case of more general books:

    \init covered []
    Explain how to install Rust
    \push covered "install"

    \assert_in covered "install"    
    Now that you've installed Rust, let's
    dig into how building binaries works.
    \push covered "build"

    \assert_in covered "install", "build"
    Now that you've learned to install and build
    Rust, let's talk about...

This is something that I wish I'd had when writing my book,
as it would have made it easier to know what I could or could
not reorder safely.


# Literal

`literal` is a command-line tool to track state and render templates
using that state in text documents. The idea first came from idly wondering
how [LitRPG](https://en.wikipedia.org/wiki/LitRPG) authors maintain state
across the many chapters and novels, but you could use it's intended to be general
purpose.

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

    | **Level**      | {{ level }}       |
    | **Experience** | {{ exp }}         |

Which you'd use `literal` to render:

    ./literal text.md

Resulting in:

    # Hello
    
    This is the first line of your book.
    | --- | --- |
    | **Level**      | 1 |
    | **Experience** | 0   |
    
    
    This is a thing
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

## Build

Because this is in a very rough state, I'm not distributing binaries,
and you'll need to build it yourself.

Once you [have installed Rust](https://www.rust-lang.org/tools/install),
then building should be as easy as:

    git clone $REPO
    cargo build

And your binary will be located at `./target/debug/literal`.
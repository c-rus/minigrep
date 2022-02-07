# minigrep

# A simple clone of the `grep` command-line tool implemented from [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) book.

```
USAGE:
    minigrep <pattern> <filename> [OPTIONS]

ARGUMENTS:
    <pattern>           Keywords to search for
    <filename>          File path to read from

OPTIONS:
    --any               Ignore case sensitivity

MORE:
    Setting an environment variable called "CASE_INSENSITIVE" will trigger 
    ignoring case-sensitivity in the search results. The --any flag overrides 
    this check.
```
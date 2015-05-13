# concat-rs

This library provides the Concat reader adaptor, which wraps around an
iterator of readers and exposes its items' contents sequentially. Thus,
the contents read from a Concat instance will be the concatenation of
the items' contents.

## Example usage

Assuming there's a variable `files: &mut [File]` in scope

```
let mut c = concat(files.iter_mut());

let mut buf = String::new();
c.read_to_string(&mut buf).unwrap();
```

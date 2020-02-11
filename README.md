# minimal-yaml

A minimalist, zero-copy parser for a strict subset of the YAML specification.

## Motivation

The full YAML specification is [quite complex](https://yaml.org/spec/1.2/spec.html) and includes features such as aliases, anchors, and type inference which are non-trivial to parse and resolve. For some use cases, those features are overkill and the associated overhead is undesirable. In its barest form, YAML (arguably) boils down to a clean way to represent a structure of sequences and mappings. This crate aims to support this simple use-case.

## Use

Supports YAML input consisting of only sequences and mappings. No type inference is performed, with all literal values being captured as string slices.
Sequences may be in flow style

```yaml
[such, as, this, sequence]
```

or in block style

```yaml
- laid
- out
- like
- this
```

Similarly, mappings may be in flow style

```yaml
{with : the, mapping : inline}
```

or in block style

```yaml
spread : across
multiple : lines
```

Of course, sequences and mappings may be nested, and any valid YAML element can be a key or value, so for example

```yaml
[this, is] :
  -
    - totally
    - valid
  - input
  - {to : the parser}
```

In accordance with the crate's goal of minimalism, the public API consists of one main structure: `enum Yaml<'a>` and one main function: `pub fn parse<'a>(input: &'a str) -> Result<Yaml<'a>>`

## Performance

***Caveat***:
*The following is based on fairly limited benchmarking performed on a single machine using inputs of various sizes*

A side benefit of keeping the parser as simple as possible is that *minimal-yaml* performs better than existing, full-featured parsers. In comparison with the fully spec compliant parser [*yaml-rust*](https://github.com/chyhå1990/yaml-rust), *minimal-yaml* consistently performs 3-5x better (i.e. parsing takes 1/3 to 1/5 of the time of *yaml-rust*).

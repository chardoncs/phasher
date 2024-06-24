# phasher

Simple CLI program hashing passwords into [PHC strings](https://github.com/P-H-C/phc-string-format/blob/master/phc-sf-spec.md).

## Usage

By default, phasher uses Argon2 to hash your data.

```bash
phasher "Hello world"
```

You can also choose different algorithms.

```bash
phasher "Hello world" -a scrypt
```

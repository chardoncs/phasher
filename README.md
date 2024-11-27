# phasher

Simple CLI program hashing passwords into [PHC strings](https://github.com/P-H-C/phc-string-format/blob/master/phc-sf-spec.md).

## Usage

### Basic

Provide one string:

```bash
phasher "Hello world"
```

Or multiple strings: (since v0.2)

```bash
phasher "Hello world" GoodMorning 'GoodNight'
```

### Algorithms

By default, phasher uses Argon2 to hash your data.
You can also choose different algorithms.

```bash
phasher "Hello world" -a scrypt
```

### stdin reading (since v0.2)

If there are no strings provided, phasher will read stdin instead:

```bash
phasher << EOF
Hello world
Welcome!
EOF
```

```bash
phasher < /path/to/a/file.txt
```

You can also use piping to pass data into phasher:

```bash
printf %s "Hello world" | phasher
```

```bash
cat /path/to/a/file.txt | phasher
```

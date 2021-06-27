# Cat With An Attidute (in rust)

A re-write of the [cwaa](https://github.com/zakarouf/cwaa) program in rust.

### Install

```c
cargo install cwaa-rs
```

### Usage 

From `cwaa-rs -h`

```
USAGE:
    cwaa-rs [OPTIONS] <INPUT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d <0 | 1>        Set Draw Mode
    -x <Float>        Takes in the Maximum Draw speed
    -m <Float>        Takes in the Minimum Draw speed

ARGS:
    <INPUT>    Input File/Folder to read
```

#### Example

* **Print Out Text File**
```sh
cwaa-rs foo.txt
```

* **Print Out All Text Files inside of a Directory**

```sh
cwaa-rs Foo
```
> Assuming Foo is a Directory

* **Type Out Text File character by character**

```sh
cwaa-rs foo.txt -d 1
```
> Use -x <> and -m <> to set out maximum and minimum typing/drawing speed.


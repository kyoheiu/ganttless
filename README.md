# ganttless
This cli app produces "gantt chart in ASCII" like followings:

```
        2022-01-01              2022-02-03
        ----------------------------------
taskA  |01-01 ->                  01-31
       |+++++++++++++++++++++++++++++++
taskB  |    01-05 -> 01-17
       |    +++++++++++++
taskC-1|                        01-25 -> 02-03
       |                        ++++++++++
```

```
        2022-01-01              2022-02-03
        ----------------------------------
taskA  |+++++++++++++++++++++++++++++++
taskB  |    +++++++++++++
taskC-1|                        ++++++++++
```

```
     1914                        1945
     --------------------------------
WWI |1914 -> 1918
    |+++++
WWII|                         1939 -> 1945
    |                         +++++++
```

## Installation
```
git clone https://github.com/kyoheiu/ganttless.git
cd ganttless
cargo install --path .
```

## Usage
```
USAGE:
    gnt [OPTIONS] [--] [INPUT_FILE]

ARGS:
    <INPUT_FILE>


OPTIONS:
    -d, --day
            Use day format

    -h, --help
            Print help information

    -i, --input <INPUT>...
            Input data as args: title=yyyy-m-d:yyyy-m-d or title=integer:integer

    -n, --number
            Use number format

    -s, --simple
            Print simple chart

    -V, --version
            Print version information
```

### Input as YAML
For example, see this yaml file:
```
# sample_number.yaml
fmt: Number
charts:
  WWII: 1939:1945
  WWI: 1914:1918
```

`gnt sample_number.yaml` produces this chart:
```
     1914                        1945
     --------------------------------
WWI |1914 -> 1918
    |+++++
WWII|                         1939 -> 1945
    |                         +++++++
```

And, `gnt -s sample_number.yaml` makes this:
```
     1914                        1945
     --------------------------------
WWI |+++++
WWII|                         +++++++
```
You can choose "verbose" or "simple" as you like.

Similarly, if you have this yaml:
```
# sample_day.yaml
fmt: Day
charts:
  taskA: 2022-1-1:2022-1-31
  taskB: 2022-1-5:2022-1-17
  taskC-1: 2022-1-25:2022-2-3
```
`gnt sample_day.yaml`The chart goes like this:
```
        2022-01-01              2022-02-03
        ----------------------------------
taskA  |01-01 ->                  01-31
       |+++++++++++++++++++++++++++++++
taskB  |    01-05 -> 01-17
       |    +++++++++++++
taskC-1|                        01-25 -> 02-03
       |                        ++++++++++
```

### Input as Args
yaml can handle long charts, but when it gonna be simple, you can give inputs as arguments easily.
`gnt -d -i taskP=2022-1-5:2022-1-14 taskQ=2022-1-12:2022-2-1` produces this chart:
```
      2022-01-05        2022-02-01
      ----------------------------
taskP|01-05 -> 01-14
     |++++++++++
taskQ|       01-12 ->        02-01
     |       +++++++++++++++++++++
```
How do you like this? Hope you enjoy.


## web app version
Also made a web app version as a trial, so take a look.
https://ganttless.app/
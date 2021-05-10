# lazer-rs

Console printer with a fluent API, written in Rust.

# lazer

Lazer is a utility for printing to the console using a fluent API. Ported to Rust from the original TypeScript package.

## Description
Lazer helps you build, format and print complex messages to the console using an expressive fluent API.

## Usage

### Simple Example

```typescript
import { lazer } from "https://deno.land/x/lazer/mod.ts"

lazer()
    .print("Hello,")
    .print_space()
    .print_green("Green World")
    .print_ln("!")
```

```bash
$ deno run example.ts
Hello, Green World!

```

### Complex Example

```typescript
import { lazer } from "https://deno.land/x/lazer/mod.ts"

const remoteAddr = "127.0.0.1";
const method = "GET";
const path = "/a/really/really/really/long/path/here";
const status = 200;
const time_ms = 20;
const size_bytes_string = "1.10kB";

lazer()
    .print('[').print_utc_time().print(']')
    .print_space().print("-").print_space()
    .print_pad_right(remoteAddr, 15, '_')
    .print_space(2)
    .print_pad_right(method, 4, '_')
    .print_space(2)
    .print_pad_right(path, 20, "_")
    .print_space(2)
    .if(status >= 200 && status < 300)
        .print_green(status)
    .elseif(status >= 300 && status < 400)
        .print_yellow(status)
    .elseif(status >= 400)
        .print_red(status)
    .end()
    .print_space(2)
    .print_pad_right(`${time_ms}ms`, 6, "_")
    .print_space(2)
    .print_ln(size_bytes_string);
```

```bash
$ deno run example.ts
[Fri, 01 Jan 2021 00:00:00 GMT] - 127.0.0.1______  GET_  /a/really/really/+42  200  20ms__  1.10kB 

```

### Buffering Example

```typescript
import { lazer } from "https://deno.land/x/lazer/mod.ts"

const getLinePrefix = () => 
{
    return lazer().buffer()
        .print_yellow('[').print_yellow("Line Prefix").print_yellow(']')
        .print_space().print("-").print_space()
        .print_yellow('[').set_color_yellow().print_utc_time().print_yellow(']')
        .print_space().print("-").print_space()
        .return();
}

lazer()
    .print(getLinePrefix())
    .print_yellow_ln("This is a prefixed line of text output");

lazer()
    .print(getLinePrefix())
    .print_yellow_ln("This is another prefixed line of text output");
```

```bash
$ deno run example.ts
[Line Prefix] - [Mon, 10 May 2021 16:31:29 GMT] - This is a prefixed line of text output
[Line Prefix] - [Mon, 10 May 2021 16:31:29 GMT] - This is another prefixed line of text output

```

### Buffer Aliasing Example

```typescript
import { lazer } from "https://deno.land/x/lazer/mod.ts"

lazer().buffer()
    .set_color_red().print_ln("Some red output to buffer")
    .store('i am an alias');

lazer().buffer()
    .load('i am an alias')
    .print_b();
```

```bash
$ deno run example.ts
Some red output to buffer

```

## Supported Platforms

### Deno
```typescript
import { lazer } from "https://deno.land/x/lazer/mod.ts"
```

### Node.js
```bash
npm i --save lazer-js
```
```javascript
const { lazer } = require('lazer-js');
```
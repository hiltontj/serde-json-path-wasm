# serde-json-path

## About

This crate is used to publish the JSONPath parsing and query functionality of the Rust [`serde_json_path`](https://crates.io/crates/serde_json_path) crate into WASM, to be used in a browser.

It is used at [serdejsonpath.live](https://serdejsonpath.live).

## Usage

```javascript
import { JsonPath } from "serde-json-path";

const obj = {
  "foo": [
    "bar",
    "baz"
  ]
};

const path = JsonPath.parse("$.foo.*");
const nodes = path.query(obj);
// -> ["bar", "baz"]
```

## Build

Build with `wasm-pack build`

```
wasm-pack build
```

Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## License

MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, shall be dual licensed as above,
without any additional terms or conditions.
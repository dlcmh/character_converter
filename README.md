# character_converter
##### v1.0.0
---

### About
Turn Traditional Chinese script to Simplified Chinese script and vice-versa. Check string script to determine if string is Traditional or Simplified Chinese Characters.

### Usage
```rust
extern crate character_converter;

use character_converter::{is_traditional, is_simplified, traditional_to_simplified, simplified_to_traditional};

let traditional_text = "歐洲";
let simplified_text = "欧洲";

// Check script
assert!(is_traditional(traditional_text));

assert!(!is_simplified(traditional_text));

// Convert script
let result_three = traditional_to_simplified(traditional_text);
assert_eq!(result_three, simplified_text);

let result_four = simplified_to_traditional(simplified_text);
assert_eq!(result_four, traditional_text);
```

### Benchmarks
Run benchmarks using the nightly bench feature:
```
cargo +nightly bench --features=bench
```

### License
[MIT](https://github.com/sotch-pr35mac/character_converter/blob/master/LICENSE)

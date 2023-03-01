/*!
# Usage

```text
use repr::{DIGIT, Pattern};
let re = DIGIT * 4 & "-" & DIGIT * 2 & "-" & DIGIT * 2;
assert!(re.is_match("2014-01-01"));
```

```text
let re = (D * 4)["year"] & "-" & (D * 2)["month"] & "-" & (D * 2)["day"];
let before = "2012-03-14, 2013-01-01 and 2014-07-05";
let after = re.replace_all(before, "$m/$d/$y");
assert_eq!(after, "03/14/2012, 01/01/2013 and 07/05/2014");
```

```text
use repr::{WORD, Pattern};
let wh = WORD | "-";
let re = (wh | ".") * 1.. & "@" & (wh * 1.. & ".") * 1.. & wh * 2..4;
```
*/

#![feature(pattern)]
#![feature(once_cell)]
#![feature(const_trait_impl)]
#![feature(box_syntax)]
#![feature(try_trait_v2)]

pub mod char;
pub mod consts;
pub mod escape;
pub mod macros;
pub mod pat;

pub use consts::{DIGIT, SPACE, WORD};
pub use pat::Pat;

// #[test]
// fn datetime() {
//     let year = DIGIT * 4;
//     let month = DIGIT * 2;
//     let day = DIGIT * 2;
//     let re = year & "-" & month & "-" & day;
//     let before = "2012-03-14, 2013-01-01 and 2014-07-05";
//     let after = re.replace_all(before, "$m/$d/$y");
//     assert_eq!(after, "03/14/2012, 01/01/2013 and 07/05/2014");
// }

// #[test]
// fn phone_number() {

// }

// #[test]
// fn email() {

// }
// #[test]
// fn url() {

// }

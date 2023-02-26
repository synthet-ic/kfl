#[allow(dead_code)]
pub fn hint_same_type<T>(_lhs: &T, _rhs: &T) {}

#[macro_export]
macro_rules! assert_decode {
    ($input:literal, $output:expr) => {
        let node = kfl::decode("<test>", $input).unwrap();
        let output = $output;
        common::hint_same_type(&node, &output);
        assert_eq!(node, output);
    }
}

#[macro_export]
macro_rules! assert_decode_error {
    ($ty:ty, $input:literal, $output:literal) => {
        let err = kfl::decode::<$ty>("<test>", $input).unwrap_err();
        let err = <kfl::Error as miette::Diagnostic>::related(&err).unwrap()
            .map(|e| e.to_owned()).collect::<Vec<_>>()
            .join("\n");
        assert_eq!(err, $output);
    }
}

#[macro_export]
macro_rules! assert_decode_children {
    ($input:literal, $output:expr) => {
        let node = kfl::decode_children("<test>", $input).unwrap();
        let output = $output;
        common::hint_same_type(&node, &output);
        assert_eq!(node, output);
    }
}

#[macro_export]
macro_rules! assert_decode_children_error {
    ($ty:ty, $input:literal, $output:literal) => {
        let err = kfl::decode_children::<$ty>("<test>", $input)
            .unwrap_err();
        let err = <kfl::Error as miette::Diagnostic>::related(&err).unwrap()
            .map(|e| e.to_owned()).collect::<Vec<_>>()
            .join("\n");
        assert_eq!(err, $output);
    }
}

#[macro_export]
macro_rules! assert_encode {
    ($input:expr, $output:literal) => {
        let input = kfl::encode("<test>", &$input).unwrap();
        let output = $output.to_owned();
        // common::hint_same_type(&input, &output);
        assert_eq!(input, output);
    }
}

// #[macro_export]
// macro_rules! assert_encode_error {
//     ($ty:ty, $input:literal, $output:literal) => {
//         let err = kfl::eecode::<$ty>("<test>", $input).unwrap_err();
//         let err = <kfl::Error as miette::Diagnostic>::related(&err).unwrap()
//             .map(|e| e.to_owned()).collect::<Vec<_>>()
//             .join("\n");
//         assert_eq!(err, $output);
//     }
// }

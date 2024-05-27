// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.

//

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}
/*
这段 Rust 代码中的问题在于 PositiveNonzeroInteger::new 函数。这个函数的目的是创建一个正的非零整数，但是它没有正确地处理负数和零的情况。

在你提供的代码中，PositiveNonzeroInteger::new 函数总是返回一个 Ok，即使输入的值是负数或零。这就是为什么测试 test_creation 失败的原因。
*/
impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm... Why is this always returning an Ok value?
        if value < 0 {
            return Err(CreationError::Negative);
        }
        if value == 0 {
            return Err(CreationError::Zero);
        }

        return Ok(PositiveNonzeroInteger(value as u64));
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}

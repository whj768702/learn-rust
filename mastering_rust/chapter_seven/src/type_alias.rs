#![allow(dead_code)]

// 类型别名
pub struct ParsedPayload<T> {
    inner: T,
}

pub struct ParseError<E> {
    inner: E,
}

// 函数签名太长，可以使用类型别名
pub fn parse_payload<T, E>(stream: &[u8]) -> Result<ParsedPayload<T>, ParseError<E>> {
    unimplemented!();
}

// 别名
type ParserResult<T, E> = Result<ParsedPayload<T>, ParseError<E>>;

// 使用类型别名
pub fn parse_payload_alias<T, E>(stream: &[u8]) -> ParserResult<T, E> {
    unimplemented!();
}

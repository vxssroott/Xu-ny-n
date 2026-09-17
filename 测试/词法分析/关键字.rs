use xuanyan::词法分析::词元::Token;

#[test]
fn 中文关键字应被识别() {
    assert_eq!(Token::函数, Token::函数);
}

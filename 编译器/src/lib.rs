pub mod 词法分析;
pub mod 语法分析;
pub mod 语法树;

#[cfg(test)]
mod 测试 {
    use crate::词法分析::词法分析器;
    use crate::语法分析::语法分析器;
    use crate::语法树::{表达式, 语句};

    #[test]
    fn 应该解析中文函数() {
        let 源码 = r#"
函数 主程序():
    让 数字 = 42
    返回 数字
"#;

        let mut 词法分析器 = 词法分析器::新建(源码);
        let 词元 = 词法分析器.分析();

        let mut 语法分析器 = 语法分析器::新建(词元);
        let 程序 = 语法分析器.解析().unwrap();

        assert_eq!(程序.函数.len(), 1);
        assert_eq!(程序.函数[0].名称, "主程序");
        assert_eq!(程序.函数[0].主体.len(), 2);

        match &程序.函数[0].主体[0] {
            语句::声明 { 名称, 值 } => {
                assert_eq!(名称, "数字");
                assert!(matches!(值, 表达式::数字(数字) if 数字 == "42"));
            }
            _ => panic!("不是变量声明"),
        }
    }
}

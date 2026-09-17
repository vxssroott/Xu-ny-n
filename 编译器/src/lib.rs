pub mod 词法分析;
pub mod 语法分析;
pub mod 语法树;
pub mod 类型系统;

#[cfg(test)]
mod 测试 {
    use crate::词法分析::词法分析器;
    use crate::语法分析::语法分析器;
    use crate::类型系统::{类型, 类型检查器};

    #[test]
    fn 应该推断整数类型() {
        let 源码 = r#"
函数 主程序():
    让 数字 = 42
    返回 数字
"#;

        let mut 词法分析器 = 词法分析器::新建(源码);
        let 词元 = 词法分析器.分析();

        let mut 语法分析器 = 语法分析器::新建(词元);
        let 程序 = 语法分析器.解析().unwrap();

        let mut 检查器 = 类型检查器::新建();
        assert!(检查器.检查(&程序).is_ok());

        assert_eq!(类型::整数.名称(), "整数");
    }

    #[test]
    fn 应该拒绝未定义变量() {
        let 源码 = r#"
函数 主程序():
    返回 未知变量
"#;

        let mut 词法分析器 = 词法分析器::新建(源码);
        let 词元 = 词法分析器.分析();

        let mut 语法分析器 = 语法分析器::新建(词元);
        let 程序 = 语法分析器.解析().unwrap();

        let mut 检查器 = 类型检查器::新建();
        assert!(检查器.检查(&程序).is_err());
    }
}

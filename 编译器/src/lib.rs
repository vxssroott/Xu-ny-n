pub mod 词法分析;
pub mod 语法分析;
pub mod 语法树;
pub mod 类型系统;
pub mod 所有权系统;
pub mod 中间表示;
pub mod 代码生成;

#[cfg(test)]
mod 测试 {
    use crate::中间表示::*;

    #[test]
    fn 应该创建中间模块() {
        let mut 模块 = 中间模块::新建();

        模块.函数.push(中间函数 {
            名称: "主程序".into(),
            参数: vec![],
            返回类型: 类型::整数,
            基本块: vec![
                基本块 {
                    名称: "入口".into(),
                    指令: vec![
                        指令::返回(
                            值::常量整数(42)
                        )
                    ],
                }
            ],
        });

        assert_eq!(
            模块.函数[0].名称,
            "主程序"
        );

        assert!(
            模块.转文本().contains("return 42")
        );
    }

    #[test]
    fn 应该生成后端代码() {
        let mut 模块 = 中间模块::新建();

        模块.函数.push(中间函数 {
            名称: "主程序".into(),
            参数: vec![],
            返回类型: 类型::整数,
            基本块: vec![
                基本块 {
                    名称: "入口".into(),
                    指令: vec![
                        指令::返回(
                            值::常量整数(42)
                        )
                    ],
                }
            ],
        });

        let 生成器 =
            crate::代码生成::代码生成器::新建();

        let 输出 = 生成器.生成(&模块);

        assert!(
            输出.contains("func @主程序")
        );

        assert!(
            输出.contains("return 42")
        );
    }
}

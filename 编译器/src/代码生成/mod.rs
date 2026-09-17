use super::中间表示::{
    中间函数,
    中间模块,
    类型,
    值,
    基本块,
    指令,
};

pub struct 代码生成器;

impl 代码生成器 {
    pub fn 新建() -> Self {
        Self
    }

    pub fn 生成(&self, 模块: &中间模块) -> String {
        let mut 输出 = String::new();

        输出.push_str("; 玄言生成代码\n");
        输出.push_str("; Phase 6 textual backend\n\n");

        for 函数 in &模块.函数 {
            输出.push_str(&self.生成函数(函数));
            输出.push('\n');
        }

        输出
    }

    fn 生成函数(&self, 函数: &中间函数) -> String {
        let mut 输出 = String::new();

        输出.push_str(&format!(
            "func @{}(",
            函数.名称
        ));

        for (索引, (名称, 类型)) in 函数.参数.iter().enumerate() {
            if 索引 > 0 {
                输出.push_str(", ");
            }

            输出.push_str(&format!(
                "%{}: {}",
                名称,
                类型名称(类型)
            ));
        }

        输出.push_str(&format!(
            ") -> {} {{\n",
            类型名称(&函数.返回类型)
        ));

        for 块 in &函数.基本块 {
            输出.push_str(&format!(
                "{}:\n",
                块.名称
            ));

            for 指令 in &块.指令 {
                输出.push_str("  ");
                输出.push_str(&指令文本(指令));
                输出.push('\n');
            }
        }

        输出.push('}');

        输出
    }
}

fn 类型名称(类型: &类型) -> &'static str {
    match 类型 {
        类型::整数 => "i64",
        类型::浮点数 => "f64",
        类型::字符串 => "str",
        类型::布尔 => "bool",
        类型::空值 => "void",
    }
}

fn 值文本(值: &值) -> String {
    match 值 {
        值::常量整数(数字) => 数字.to_string(),
        值::常量浮点数(数字) => 数字.to_string(),
        值::常量字符串(文本) => format!("\"{}\"", 文本),
        值::常量布尔(布尔) => 布尔.to_string(),
        值::变量(名称) => format!("%{}", 名称),
        值::空值 => "void".into(),
    }
}

fn 指令文本(指令: &指令) -> String {
    match 指令 {
        指令::参数 { 名称, 类型 } => {
            format!(
                "%{} : {}",
                名称,
                类型名称(类型)
            )
        }

        指令::分配 { 名称, 类型 } => {
            format!(
                "%{} = alloc {}",
                名称,
                类型名称(类型)
            )
        }

        指令::存储 { 目标, 值 } => {
            format!(
                "store %{}, {}",
                目标,
                值文本(值)
            )
        }

        指令::加载 { 目标, 来源 } => {
            format!(
                "%{} = load %{}",
                目标,
                来源
            )
        }

        指令::返回(值) => {
            format!(
                "return {}",
                值文本(值)
            )
        }

        指令::调用 {
            目标,
            函数,
            参数,
        } => {
            let 参数文本 = 参数
                .iter()
                .map(值文本)
                .collect::<Vec<_>>()
                .join(", ");

            match 目标 {
                Some(目标) => format!(
                    "%{} = call {}({})",
                    目标,
                    函数,
                    参数文本
                ),

                None => format!(
                    "call {}({})",
                    函数,
                    参数文本
                ),
            }
        }
    }
}

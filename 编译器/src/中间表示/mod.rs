#[derive(Debug, Clone, PartialEq)]
pub enum 类型 {
    整数,
    浮点数,
    字符串,
    布尔,
    空值,
}

#[derive(Debug, Clone, PartialEq)]
pub enum 值 {
    常量整数(i64),
    常量浮点数(f64),
    常量字符串(String),
    常量布尔(bool),
    变量(String),
    空值,
}

#[derive(Debug, Clone, PartialEq)]
pub enum 指令 {
    参数 {
        名称: String,
        类型: 类型,
    },

    分配 {
        名称: String,
        类型: 类型,
    },

    存储 {
        目标: String,
        值: 值,
    },

    加载 {
        目标: String,
        来源: String,
    },

    返回(值),

    调用 {
        目标: Option<String>,
        函数: String,
        参数: Vec<值>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct 基本块 {
    pub 名称: String,
    pub 指令: Vec<指令>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct 中间函数 {
    pub 名称: String,
    pub 参数: Vec<(String, 类型)>,
    pub 返回类型: 类型,
    pub 基本块: Vec<基本块>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct 中间模块 {
    pub 函数: Vec<中间函数>,
}

impl 中间模块 {
    pub fn 新建() -> Self {
        Self {
            函数: Vec::new(),
        }
    }

    pub fn 转文本(&self) -> String {
        let mut 输出 = String::new();

        for 函数 in &self.函数 {
            输出.push_str(&format!(
                "函数 {}(", 函数.名称
            ));

            for (索引, (名称, 类型)) in 函数.参数.iter().enumerate() {
                if 索引 > 0 {
                    输出.push_str(", ");
                }

                输出.push_str(&format!(
                    "{}: {}",
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
                    "  {}:\n",
                    块.名称
                ));

                for 指令 in &块.指令 {
                    输出.push_str(&format!(
                        "    {}\n",
                        指令文本(指令)
                    ));
                }
            }

            输出.push_str("}\n");
        }

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
            format!("%{} : {}", 名称, 类型名称(类型))
        }

        指令::分配 { 名称, 类型 } => {
            format!("%{} = alloc {}", 名称, 类型名称(类型))
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
            format!("return {}", 值文本(值))
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

use crate::语法树::{表达式, 函数, 程序, 语句};

#[derive(Debug, Clone, PartialEq)]
pub enum 类型 {
    整数,
    浮点数,
    字符串,
    布尔,
    空值,
    未知,
}

impl 类型 {
    pub fn 名称(&self) -> &'static str {
        match self {
            类型::整数 => "整数",
            类型::浮点数 => "浮点数",
            类型::字符串 => "字符串",
            类型::布尔 => "布尔",
            类型::空值 => "空值",
            类型::未知 => "未知",
        }
    }
}

#[derive(Debug, Clone)]
pub struct 符号 {
    pub 名称: String,
    pub 类型: 类型,
}

#[derive(Debug)]
pub struct 语义错误 {
    pub 消息: String,
}

pub struct 类型检查器 {
    符号表: Vec<符号>,
    错误: Vec<语义错误>,
    当前函数返回类型: 类型,
}

impl 类型检查器 {
    pub fn 新建() -> Self {
        Self {
            符号表: Vec::new(),
            错误: Vec::new(),
            当前函数返回类型: 类型::空值,
        }
    }

    pub fn 检查(&mut self, 程序: &程序) -> Result<(), Vec<语义错误>> {
        for 函数 in &程序.函数 {
            self.检查函数(函数);
        }

        if self.错误.is_empty() {
            Ok(())
        } else {
            Err(self.错误.drain(..).collect())
        }
    }

    fn 检查函数(&mut self, 函数: &函数) {
        self.符号表.clear();
        self.当前函数返回类型 = 类型::空值;

        for 参数 in &函数.参数 {
            self.符号表.push(符号 {
                名称: 参数.clone(),
                类型: 类型::未知,
            });
        }

        for 语句 in &函数.主体 {
            self.检查语句(语句);
        }
    }

    fn 检查语句(&mut self, 语句: &语句) {
        match 语句 {
            语句::表达式(表达式) => {
                self.检查表达式(表达式);
            }

            语句::返回(表达式) => {
                let 类型 = self.检查表达式(表达式);

                if self.当前函数返回类型 == 类型::空值 {
                    self.当前函数返回类型 = 类型;
                } else if self.当前函数返回类型 != 类型 {
                    self.错误.push(语义错误 {
                        消息: format!(
                            "返回类型不一致：期待「{}」，得到「{}」",
                            self.当前函数返回类型.名称(),
                            类型.名称()
                        ),
                    });
                }
            }

            语句::声明 { 名称, 值 } => {
                let 类型 = self.检查表达式(值);

                if self.查找符号(名称).is_some() {
                    self.错误.push(语义错误 {
                        消息: format!("变量「{}」重复声明", 名称),
                    });
                    return;
                }

                self.符号表.push(符号 {
                    名称: 名称.clone(),
                    类型,
                });
            }
        }
    }

    fn 检查表达式(&mut self, 表达式: &表达式) -> 类型 {
        match 表达式 {
            表达式::数字(_) => 类型::整数,

            表达式::字符串(_) => 类型::字符串,

            表达式::标识符(名称) => {
                match self.查找符号(名称) {
                    Some(符号) => 符号.类型.clone(),

                    None => {
                        self.错误.push(语义错误 {
                            消息: format!("未定义的变量「{}」", 名称),
                        });

                        类型::未知
                    }
                }
            }
        }
    }

    fn 查找符号(&self, 名称: &str) -> Option<&符号> {
        self.符号表
            .iter()
            .rev()
            .find(|符号| 符号.名称 == 名称)
    }
}

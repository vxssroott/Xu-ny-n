use crate::中间表示::{
    中间模块,
    中间函数,
    基本块,
    指令,
    值,
};

#[derive(Debug, Clone, PartialEq)]
pub enum 运行值 {
    整数(i64),
    浮点数(f64),
    字符串(String),
    布尔(bool),
    空值,
}

#[derive(Debug)]
pub enum 运行错误 {
    未知函数(String),
    未知变量(String),
    无效操作(String),
    缺少返回值,
}

pub struct 虚拟机 {
    栈: Vec<运行值>,
}

impl 虚拟机 {
    pub fn 新建() -> Self {
        Self {
            栈: Vec::new(),
        }
    }

    pub fn 执行(
        &mut self,
        模块: &中间模块,
    ) -> Result<运行值, 运行错误> {
        let 函数 = 模块
            .函数
            .iter()
            .find(|函数| 函数.名称 == "主程序")
            .ok_or_else(|| {
                运行错误::未知函数("主程序".into())
            })?;

        self.执行函数(函数)
    }

    fn 执行函数(
        &mut self,
        函数: &中间函数,
    ) -> Result<运行值, 运行错误> {
        let mut 变量:
            std::collections::HashMap<String, 运行值>
            = std::collections::HashMap::new();

        for 块 in &函数.基本块 {
            if let Some(结果) =
                self.执行基本块(块, &mut 变量)?
            {
                return Ok(结果);
            }
        }

        Err(运行错误::缺少返回值)
    }

    fn 执行基本块(
        &mut self,
        块: &基本块,
        变量: &mut std::collections::HashMap<String, 运行值>,
    ) -> Result<Option<运行值>, 运行错误> {
        for 指令 in &块.指令 {
            match 指令 {
                指令::参数 { .. } => {}

                指令::分配 { 名称, .. } => {
                    变量.insert(
                        名称.clone(),
                        运行值::空值,
                    );
                }

                指令::存储 { 目标, 值 } => {
                    let 运行值 = self.求值(值, 变量)?;
                    变量.insert(
                        目标.clone(),
                        运行值,
                    );
                }

                指令::加载 { 目标, 来源 } => {
                    let 值 = 变量
                        .get(来源)
                        .cloned()
                        .ok_or_else(|| {
                            运行错误::未知变量(
                                来源.clone()
                            )
                        })?;

                    变量.insert(
                        目标.clone(),
                        值,
                    );
                }

                指令::返回(值) => {
                    return Ok(Some(
                        self.求值(值, 变量)?
                    ));
                }

                指令::调用 {
                    目标,
                    函数,
                    参数,
                } => {
                    let 参数值 = 参数
                        .iter()
                        .map(|值| self.求值(值, 变量))
                        .collect::<Result<Vec<_>, _>>()?;

                    let 结果 =
                        self.执行内置函数(
                            函数,
                            参数值,
                        )?;

                    if let Some(目标) = 目标 {
                        变量.insert(
                            目标.clone(),
                            结果,
                        );
                    }
                }
            }
        }

        Ok(None)
    }

    fn 求值(
        &self,
        值: &值,
        变量: &std::collections::HashMap<String, 运行值>,
    ) -> Result<运行值, 运行错误> {
        match 值 {
            值::常量整数(数字) => {
                Ok(运行值::整数(*数字))
            }

            值::常量浮点数(数字) => {
                Ok(运行值::浮点数(*数字))
            }

            值::常量字符串(文本) => {
                Ok(运行值::字符串(文本.clone()))
            }

            值::常量布尔(布尔) => {
                Ok(运行值::布尔(*布尔))
            }

            值::变量(名称) => {
                变量
                    .get(名称)
                    .cloned()
                    .ok_or_else(|| {
                        运行错误::未知变量(
                            名称.clone()
                        )
                    })
            }

            值::空值 => Ok(运行值::空值),
        }
    }

    fn 执行内置函数(
        &mut self,
        函数: &str,
        参数: Vec<运行值>,
    ) -> Result<运行值, 运行错误> {
        match 函数 {
            "打印" => {
                for 参数 in 参数 {
                    println!("{:?}", 参数);
                }

                Ok(运行值::空值)
            }

            _ => Err(
                运行错误::未知函数(
                    函数.to_string()
                )
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum 所有权 {
    拥有,
    借用,
    可变借用,
    移动,
    已释放,
}

#[derive(Debug, Clone, PartialEq)]
pub struct 资源 {
    pub 名称: String,
    pub 类型: String,
    pub 所有权: 所有权,
    pub 生命周期: 生命周期,
}

#[derive(Debug, Clone, PartialEq)]
pub enum 生命周期 {
    临时,
    当前作用域,
    显式(String),
    静态,
}

#[derive(Debug, Clone, PartialEq)]
pub enum 借用方式 {
    共享,
    可变,
}

#[derive(Debug)]
pub enum 所有权错误 {
    重复移动(String),
    使用已移动资源(String),
    使用已释放资源(String),
    可变借用冲突(String),
    多重可变借用(String),
    生命周期不足(String),
}

pub struct 所有权检查器 {
    资源: Vec<资源>,
    错误: Vec<所有权错误>,
}

impl 所有权检查器 {
    pub fn 新建() -> Self {
        Self {
            资源: Vec::new(),
            错误: Vec::new(),
        }
    }

    pub fn 声明资源(
        &mut self,
        名称: &str,
        类型: &str,
        生命周期: 生命周期,
    ) {
        self.资源.push(资源 {
            名称: 名称.to_string(),
            类型: 类型.to_string(),
            所有权: 所有权::拥有,
            生命周期,
        });
    }

    pub fn 移动(&mut self, 名称: &str) {
        if let Some(资源) = self.查找资源(名称) {
            match 资源.所有权 {
                所有权::拥有 => {
                    资源.所有权 = 所有权::移动;
                }

                所有权::移动 => {
                    self.错误.push(
                        所有权错误::重复移动(名称.to_string())
                    );
                }

                所有权::已释放 => {
                    self.错误.push(
                        所有权错误::使用已释放资源(名称.to_string())
                    );
                }

                _ => {}
            }
        }
    }

    pub fn 借用(
        &mut self,
        名称: &str,
        方式: 借用方式,
    ) -> Result<(), 所有权错误> {
        if let Some(资源) = self.查找资源(名称) {
            match (&资源.所有权, &方式) {
                (所有权::拥有, 借用方式::共享) => {
                    资源.所有权 = 所有权::借用;
                    Ok(())
                }

                (所有权::拥有, 借用方式::可变) => {
                    资源.所有权 = 所有权::可变借用;
                    Ok(())
                }

                (所有权::借用, 借用方式::共享) => {
                    Ok(())
                }

                (所有权::借用, 借用方式::可变) => {
                    Err(所有权错误::可变借用冲突(
                        名称.to_string()
                    ))
                }

                (所有权::可变借用, _) => {
                    Err(所有权错误::多重可变借用(
                        名称.to_string()
                    ))
                }

                (所有权::移动, _) => {
                    Err(所有权错误::使用已移动资源(
                        名称.to_string()
                    ))
                }

                (所有权::已释放, _) => {
                    Err(所有权错误::使用已释放资源(
                        名称.to_string()
                    ))
                }
            }
        } else {
            Err(所有权错误::使用已释放资源(
                名称.to_string()
            ))
        }
    }

    pub fn 释放(&mut self, 名称: &str) {
        if let Some(资源) = self.查找资源(名称) {
            资源.所有权 = 所有权::已释放;
        }
    }

    pub fn 资源状态(&self, 名称: &str) -> Option<所有权> {
        self.资源
            .iter()
            .find(|资源| 资源.名称 == 名称)
            .map(|资源| 资源.所有权.clone())
    }

    pub fn 错误(&self) -> &[所有权错误] {
        &self.错误
    }

    fn 查找资源(&mut self, 名称: &str) -> Option<&mut 资源> {
        self.资源
            .iter_mut()
            .find(|资源| 资源.名称 == 名称)
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 资源应该默认拥有() {
        let mut 检查器 = 所有权检查器::新建();

        检查器.声明资源(
            "数据",
            "字符串",
            生命周期::当前作用域,
        );

        assert_eq!(
            检查器.资源状态("数据"),
            Some(所有权::拥有)
        );
    }

    #[test]
    fn 移动后资源不再拥有() {
        let mut 检查器 = 所有权检查器::新建();

        检查器.声明资源(
            "数据",
            "字符串",
            生命周期::当前作用域,
        );

        检查器.移动("数据");

        assert_eq!(
            检查器.资源状态("数据"),
            Some(所有权::移动)
        );
    }

    #[test]
    fn 共享借用可以共存() {
        let mut 检查器 = 所有权检查器::新建();

        检查器.声明资源(
            "数据",
            "字符串",
            生命周期::当前作用域,
        );

        assert!(检查器.借用(
            "数据",
            借用方式::共享
        ).is_ok());

        assert!(检查器.借用(
            "数据",
            借用方式::共享
        ).is_ok());
    }

    #[test]
    fn 可变借用不能与共享借用冲突() {
        let mut 检查器 = 所有权检查器::新建();

        检查器.声明资源(
            "数据",
            "字符串",
            生命周期::当前作用域,
        );

        assert!(检查器.借用(
            "数据",
            借用方式::共享
        ).is_ok());

        assert!(检查器.借用(
            "数据",
            借用方式::可变
        ).is_err());
    }
}

use crate::词法分析::词元::Token;
use crate::语法树::{表达式, 函数, 程序, 语句};

pub struct 语法分析器 {
    词元: Vec<Token>,
    位置: usize,
}

impl 语法分析器 {
    pub fn 新建(词元: Vec<Token>) -> Self {
        Self { 词元, 位置: 0 }
    }

    pub fn 解析(&mut self) -> Result<程序, String> {
        let mut 函数列表 = Vec::new();

        while !self.当前是(&Token::文件结束) {
            if self.当前是(&Token::换行) {
                self.前进();
                continue;
            }

            函数列表.push(self.解析函数()?);
        }

        Ok(程序 { 函数: 函数列表 })
    }

    fn 解析函数(&mut self) -> Result<函数, String> {
        self.消耗(Token::函数, "期待「函数」")?;

        let 名称 = match self.前进() {
            Some(Token::标识符(名称)) => 名称,
            _ => return Err("函数声明需要名称".into()),
        };

        self.消耗(Token::左括号, "期待「(」")?;

        let mut 参数 = Vec::new();

        while !self.当前是(&Token::右括号) {
            match self.前进() {
                Some(Token::标识符(名称)) => 参数.push(名称),
                _ => return Err("参数必须是标识符".into()),
            }

            if self.当前是(&Token::逗号) {
                self.前进();
            } else {
                break;
            }
        }

        self.消耗(Token::右括号, "期待「)」")?;
        self.消耗(Token::冒号, "期待「:」")?;

        let mut 主体 = Vec::new();

        while !self.当前是(&Token::函数)
            && !self.当前是(&Token::文件结束)
        {
            if self.当前是(&Token::换行) {
                self.前进();
                continue;
            }

            主体.push(self.解析语句()?);
        }

        Ok(函数 {
            名称,
            参数,
            主体,
        })
    }

    fn 解析语句(&mut self) -> Result<语句, String> {
        if self.当前是(&Token::返回) {
            self.前进();

            let 表达式 = self.解析表达式()?;

            return Ok(语句::返回(表达式));
        }

        if self.当前是(&Token::让) {
            self.前进();

            let 名称 = match self.前进() {
                Some(Token::标识符(名称)) => 名称,
                _ => return Err("变量声明需要名称".into()),
            };

            self.消耗(Token::等号, "期待「=」")?;

            let 值 = self.解析表达式()?;

            return Ok(语句::声明 { 名称, 值 });
        }

        Ok(语句::表达式(self.解析表达式()?))
    }

    fn 解析表达式(&mut self) -> Result<表达式, String> {
        match self.前进() {
            Some(Token::数字(数字)) => Ok(表达式::数字(数字)),
            Some(Token::字符串(字符串)) => Ok(表达式::字符串(字符串)),
            Some(Token::标识符(名称)) => Ok(表达式::标识符(名称)),
            _ => Err("无法解析表达式".into()),
        }
    }

    fn 当前是(&self, 目标: &Token) -> bool {
        self.词元.get(self.位置) == Some(目标)
    }

    fn 前进(&mut self) -> Option<Token> {
        let 当前 = self.词元.get(self.位置).cloned();
        if 当前.is_some() {
            self.位置 += 1;
        }
        当前
    }

    fn 消耗(&mut self, 目标: Token, 错误: &str) -> Result<(), String> {
        if self.当前是(&目标) {
            self.前进();
            Ok(())
        } else {
            Err(错误.into())
        }
    }
}

pub mod 词元;

use 词元::Token;

pub struct 词法分析器 {
    源码: Vec<char>,
    位置: usize,
}

impl 词法分析器 {
    pub fn 新建(源码: &str) -> Self {
        Self {
            源码: 源码.chars().collect(),
            位置: 0,
        }
    }

    pub fn 分析(&mut self) -> Vec<Token> {
        let mut 词元 = Vec::new();

        while self.位置 < self.源码.len() {
            let 当前 = self.源码[self.位置];

            if 当前.is_whitespace() {
                if 当前 == '\n' {
                    词元.push(Token::换行);
                }
                self.位置 += 1;
                continue;
            }

            if 当前.is_ascii_digit() {
                词元.push(self.读取数字());
                continue;
            }

            if 当前.is_alphabetic() || 当前 == '_' {
                词元.push(self.读取标识符());
                continue;
            }

            match 当前 {
                '(' => 词元.push(Token::左括号),
                ')' => 词元.push(Token::右括号),
                '{' => 词元.push(Token::左大括号),
                '}' => 词元.push(Token::右大括号),
                ':' => 词元.push(Token::冒号),
                ',' => 词元.push(Token::逗号),
                '+' => 词元.push(Token::加号),
                '-' => 词元.push(Token::减号),
                '*' => 词元.push(Token::乘号),
                '/' => 词元.push(Token::除号),
                '=' => 词元.push(Token::等号),
                _ => {}
            }

            self.位置 += 1;
        }

        词元.push(Token::文件结束);
        词元
    }

    fn 读取数字(&mut self) -> Token {
        let 开始 = self.位置;

        while self.位置 < self.源码.len()
            && self.源码[self.位置].is_ascii_digit()
        {
            self.位置 += 1;
        }

        Token::数字(self.源码[开始..self.位置].iter().collect())
    }

    fn 读取标识符(&mut self) -> Token {
        let 开始 = self.位置;

        while self.位置 < self.源码.len()
            && (self.源码[self.位置].is_alphanumeric()
                || self.源码[self.位置] == '_')
        {
            self.位置 += 1;
        }

        let 名称: String = self.源码[开始..self.位置].iter().collect();

        match 名称.as_str() {
            "函数" => Token::函数,
            "如果" => Token::如果,
            "否则" => Token::否则,
            "返回" => Token::返回,
            "让" => Token::让,
            "真" => Token::真,
            "假" => Token::假,
            _ => Token::标识符(名称),
        }
    }
}

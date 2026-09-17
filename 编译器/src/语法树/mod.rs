#[derive(Debug, Clone)]
pub enum 表达式 {
    数字(String),
    字符串(String),
    标识符(String),
}

#[derive(Debug, Clone)]
pub enum 语句 {
    表达式(表达式),
    返回(表达式),
    声明 {
        名称: String,
        值: 表达式,
    },
}

#[derive(Debug, Clone)]
pub struct 函数 {
    pub 名称: String,
    pub 参数: Vec<String>,
    pub 主体: Vec<语句>,
}

#[derive(Debug, Clone)]
pub struct 程序 {
    pub 函数: Vec<函数>,
}

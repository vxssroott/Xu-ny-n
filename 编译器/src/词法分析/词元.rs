#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    标识符(String),
    数字(String),
    字符串(String),

    函数,
    如果,
    否则,
    返回,
    让,
    真,
    假,

    左括号,
    右括号,
    左大括号,
    右大括号,
    冒号,
    逗号,
    加号,
    减号,
    乘号,
    除号,
    等号,
    相等,
    不等,

    换行,
    文件结束,
}

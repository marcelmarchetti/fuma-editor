use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Not,
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    BitXor,
    BitXorAssign,

    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
    Rem,
    RemAssign,

    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,

    Shl,
    ShlAssign,
    Shr,
    ShrAssign,

    Deref,
    RangeExclusive,
    RangeInclusive,
    RangeTo,
    RangeFrom,
    RangeFull,
    Question,
    At,


    And,
    Or,
    Assign,

    Borrow,
    BorrowMut,
    RawPtr,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Operator::Not => "!",
            Operator::BitAnd => "&",
            Operator::BitAndAssign => "&=",
            Operator::BitOr => "|",
            Operator::BitOrAssign => "|=",
            Operator::BitXor => "^",
            Operator::BitXorAssign => "^=",

            Operator::Add => "+",
            Operator::AddAssign => "+=",
            Operator::Sub => "-",
            Operator::SubAssign => "-=",
            Operator::Mul => "*",
            Operator::MulAssign => "*=",
            Operator::Div => "/",
            Operator::DivAssign => "/=",
            Operator::Rem => "%",
            Operator::RemAssign => "%=",

            Operator::Eq => "==",
            Operator::Ne => "!=",
            Operator::Lt => "<",
            Operator::Le => "<=",
            Operator::Gt => ">",
            Operator::Ge => ">=",

            Operator::Shl => "<<",
            Operator::ShlAssign => "<<=",
            Operator::Shr => ">>",
            Operator::ShrAssign => ">>=",

            Operator::Deref => "*",
            Operator::RangeExclusive => "..",
            Operator::RangeInclusive => "..=",
            Operator::RangeTo => "..",
            Operator::RangeFrom => "..",
            Operator::RangeFull => "..",
            Operator::Question => "?",
            Operator::At => "@",

            Operator::And => "&&",
            Operator::Or => "||",
            Operator::Assign => "=",

            Operator::Borrow => "&",
            Operator::BorrowMut => "&mut",
            Operator::RawPtr => "*",
        };
        write!(f, "{}", s)
    }
}

impl Operator {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "!" => Some(Operator::Not),
            "!=" => Some(Operator::Ne),
            "==" => Some(Operator::Eq),
            "=" => Some(Operator::Assign),
            "&" => Some(Operator::BitAnd),
            "&=" => Some(Operator::BitAndAssign),
            "&&" => Some(Operator::And),
            "|" => Some(Operator::BitOr),
            "|=" => Some(Operator::BitOrAssign),
            "||" => Some(Operator::Or),
            "+" => Some(Operator::Add),
            "+=" => Some(Operator::AddAssign),
            "-" => Some(Operator::Sub),
            "-=" => Some(Operator::SubAssign),
            "*" => Some(Operator::Mul),
            "*=" => Some(Operator::MulAssign),
            "/" => Some(Operator::Div),
            "/=" => Some(Operator::DivAssign),
            "%" => Some(Operator::Rem),
            "%=" => Some(Operator::RemAssign),
            "<" => Some(Operator::Lt),
            "<=" => Some(Operator::Le),
            ">" => Some(Operator::Gt),
            ">=" => Some(Operator::Ge),
            "<<" => Some(Operator::Shl),
            "<<=" => Some(Operator::ShlAssign),
            ">>" => Some(Operator::Shr),
            ">>=" => Some(Operator::ShrAssign),
            ".." => Some(Operator::RangeExclusive),
            "..=" => Some(Operator::RangeInclusive),
            "?" => Some(Operator::Question),
            "@" => Some(Operator::At),
            _ => None,
        }
    }
}
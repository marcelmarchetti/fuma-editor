use std::fmt;
use std::fmt::{Display, Formatter};
use crate::editor::tree_parser::rust::basic_tokens::SyntaxTokenType::{Comment, Identifier};
use crate::editor::tree_parser::rust::syntax_tokens::delimiter::Delimiter;
use crate::editor::tree_parser::rust::syntax_tokens::operator::Operator;
use crate::editor::tree_parser::rust::syntax_tokens::keyword::Keyword;
use crate::editor::tree_parser::rust::syntax_tokens::punctuation::Punctuation;
use crate::editor::tree_parser::rust::syntax_tokens::separator::Separator;
use crate::log_debug;
use crate::utils::tokenizer::{LexicToken, LexicTokenType};
use crate::values::globals::MAX_SYMBOL_CHARS;

#[derive(PartialEq)]
pub enum SyntaxTokenType {
    Keyword(Keyword),
    Punctuation(Punctuation),
    Identifier,
    //Literal,
    Operator(Operator),
    Delimiter(Delimiter),
    Separator(Separator),
    EndOfLine,
    Comment,
}

impl Display for SyntaxTokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxTokenType::Keyword(kw) => write!(f, "{}", kw),
            Identifier => write!(f, "identifier"),
            //SyntaxToken::Literal => write!(f, "literal"),
            SyntaxTokenType::Operator(op) => write!(f, "op:{}", op),
            SyntaxTokenType::Delimiter(delim) => write!(f, "{}", delim),
            SyntaxTokenType::Punctuation(punct) => write!(f, "{}", punct),
            SyntaxTokenType::Separator(sep) => write!(f, "{}", sep),
            SyntaxTokenType::EndOfLine => write!(f, "end of line"),
            Comment => write!(f, "Comment"),
        }
    }
}

pub struct SyntaxToken {
    pub lexic_token: LexicToken,
    pub token_type: SyntaxTokenType,
}

impl Display for SyntaxToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token {}:  {}  {}",
            self.lexic_token.id,
            self.lexic_token.value,
            self.token_type,

        )
    }
}



impl SyntaxToken {
    pub fn new(lexic_token: &LexicToken) -> Self {
        let token_type: SyntaxTokenType;
        
        if lexic_token.token_type == LexicTokenType::EndOfLine {
            token_type = SyntaxTokenType::EndOfLine;
        } else {
            token_type = Keyword::from_str(&lexic_token.value)
                .map(SyntaxTokenType::Keyword)
                .or_else(|| Punctuation::from_str(&lexic_token.value).map(SyntaxTokenType::Punctuation))
                .or_else(|| Operator::from_str(&lexic_token.value).map(SyntaxTokenType::Operator))
                .or_else(|| Delimiter::from_str(&lexic_token.value).map(SyntaxTokenType::Delimiter))
                .or_else(|| Separator::from_str(&lexic_token.value).map(SyntaxTokenType::Separator))
                .or_else(|| Self::parse_comment(&lexic_token.value))
                //.or_else(|| Self::parse_literal(&lexic_token.value))
                .unwrap_or_else(|| Identifier);
        }
        
        Self {
            lexic_token: lexic_token.clone(),
            token_type
        }
    }

    fn parse_comment(s: &str) -> Option<SyntaxTokenType> {
        if s.starts_with("//") || s.starts_with("/*") || s == "*/" {
            Some(Comment)
        } else {
            None
        }
    }
}



pub struct Import {
    pub privacy: SyntaxToken,
    pub types: SyntaxToken,
    pub path: String,
    pub is_global: bool,
    pub imports: Option<Vec<String>>,

}

pub fn parse_tokens(tokens: &Vec<LexicToken>, debug: bool) -> Vec<SyntaxToken> {
    let tokens = tokens.clone();
    let mut tokens_type:Vec<SyntaxToken> = Vec::new();
    let mut skip = 0;

    for (inx, token) in tokens.iter().enumerate() {
        if skip > 0 {
            skip -= 1;
            continue;
        }

        if token.token_type == LexicTokenType::Symbol {
            let mut found = false;
            for symbol_len in (2..=MAX_SYMBOL_CHARS).rev() {
                if inx + (symbol_len - 1) < tokens.len() {
                    let combined_symbol: String =  tokens[inx..inx + symbol_len].iter().map(|t|t.value.clone()).collect();
                    if is_valid_symbol(&combined_symbol) {
                        let mut combined_token = token.clone();
                        combined_token.col_end = tokens[inx + symbol_len - 1].col_end;
                        combined_token.row_end = tokens[inx + symbol_len - 1].row_end;
                        combined_token.value = combined_symbol;
                        tokens_type.push(SyntaxToken::new(&combined_token));
                        skip = symbol_len - 1;
                        found = true;
                        break;
                    }
                }
            }
            if found{ continue;}
        }

        tokens_type.push(SyntaxToken::new(&token));
    }
    if debug {
        for token in &tokens_type {
            log_debug!("{}",token);
        }
    }
    tokens_type
}

fn is_valid_symbol(s: &str) -> bool {
    matches!(s,
        "..=" | "<<=" | ">>=" |
        "==" | "!=" | "<=" | ">=" | "&&" | "||" |
        "+=" | "-=" | "*=" | "/=" | "%=" | "&=" | "|=" | "^=" |
        "<<" | ">>" | ".." | "->" | "=>" | "::"
    )
}

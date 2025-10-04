use crate::editor::tree_parser::rust::basic_tokens::{SyntaxToken, SyntaxTokenType};

enum Level1Expr {

    TypeAnnotation {
        name: SyntaxToken,
        colon: SyntaxToken,
        type_expr: Vec<SyntaxToken>,
    },

    Comment {
        start: SyntaxToken,
        comment: Vec<SyntaxToken>,
    },


    PathExpr(Vec<SyntaxToken>),


    BinaryExpr {
        left: SyntaxToken,
        operator: SyntaxToken,
        right: SyntaxToken,
    },

    ParenGroup(Vec<SyntaxToken>),
    BracketGroup(Vec<SyntaxToken>),
    BraceGroup(Vec<SyntaxToken>),

    // GENERICS
    GenericArgs {
        base: SyntaxToken,
        args: Vec<SyntaxToken>,
    },
}

fn parse_into_lv1(tokens: &Vec<SyntaxToken>) {
    let mut tokens_lvl_1: Vec<Level1Expr> = Vec::new();
    let mut i = 0;

    while i < tokens.len() {

        if tokens[i].token_type == SyntaxTokenType::Comment {

        }


        i += 1;
    }

}

fn parse_comment(tokens: &Vec<SyntaxToken>, inx: usize) {
    let i = inx;
    if tokens[inx].lexic_token.value == "//" || tokens[inx].lexic_token.value == "///" {
        while i < tokens.len() {
            if tokens[i].token_type == SyntaxTokenType::Comment {}
        }
    }
}
pub enum WikiTableTokenType {
    OpeningBrackets,
    ClosingBrackets,

    Bang,
    DoubleBang,
    Split,
    DoubleSplit,
    SplitWithDash, // |-
    //Dash — Дефис

    Other,
}

pub struct WikiTableToken {
    pub token_type: WikiTableTokenType,
    pub value: String,
}

pub fn scan(table: &str) -> Vec<WikiTableToken> {
    let mut tokens: Vec<WikiTableToken> = vec![];
    return tokens;
}
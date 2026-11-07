use crate::scan_infoboxes_lists::{{ILTokenType, ILToken}};

fn parse(tokens: Vec<InfoboxToken>) -> Vec<String> {
    let mut list_items: Vec<String> = vec![];
    let mut i = 0;

    while i < tokens.len() {
        if tokens[i].token_type == ILTokenType::STAR {
            if tokens[i-1].token_type == ILTokenType::NEWLINE {
                let mut item = String::from("");
                while i < tokens.len() && tokens[i].token_type != ILTokenType::NEWLINE {
                    item+= &tokens[i].value;
                    i+=1;
                }

                list_items.push(item);
            }
        }

        i+=1;
    }

    return list_items;
}
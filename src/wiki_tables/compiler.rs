use crate::{utils, wiki_tables::parser::{WTableLevel1Entity, WTableLevel1EntityType}};

pub fn compile(entities: Vec<WTableLevel1Entity>) -> String {
    let mut table = String::new();
    let mut i = 0;

    while i < entities.len() {
        if i == 0 && entities[i].entity_type == WTableLevel1EntityType::TableAttrs {
            table += &format!("<table {}>", entities[i].value[0]);
        }
        else if i == 0 {
            table += "<table>";
        }
        else if entities[i].entity_type == WTableLevel1EntityType::Head {
            table += "<thead><tr>";
            let mut j = 0;

            while j < entities[i].value.len() {
                if utils::attrs::consists_only_of_attributes(&entities[i].value[j]) {
                    if j + 1 < entities[i].value.len() {
                        table += &format!("<th {}>{}</th>", entities[i].value[j], entities[i].value[j+1]);
                    }
                    j+=1;
                }
                else {
                    table += &format!("<th>{}</th>", entities[i].value[j]);
                }

                j+=1;
            }

            table += "</tr></thead><tbody>";
        }

        else if entities[i].entity_type == WTableLevel1EntityType::BodyLine {
            table += "<tr>";
            let mut j = 0;

            while j < entities[i].value.len() {
                if utils::attrs::consists_only_of_attributes(&entities[i].value[j]) {
                    if j + 1 < entities[i].value.len() {
                        table += &format!("<td {}>{}</td>", entities[i].value[j], entities[i].value[j+1]);
                    }
                    j+=1;
                }
                else {
                    table += &format!("<td>{}</td>", entities[i].value[j]);
                }

                j+=1;
            }

            table += "</tr>";
        }

        i+=1;
    }

    table += "</tbody></table>";


    return table;
}

#[cfg(test)]
mod tests {
    use super::compile;
    use crate::wiki_tables::parser::{WTableLevel1Entity, WTableLevel1EntityType};

    #[test]
    fn closes_table_head_before_body() {
        let html = compile(vec![
            WTableLevel1Entity {
                entity_type: WTableLevel1EntityType::TableAttrs,
                value: vec!["class=\"wikitable\"".to_string()],
            },
            WTableLevel1Entity {
                entity_type: WTableLevel1EntityType::Head,
                value: vec!["colspan=\"6\"".to_string(), "Shopping List".to_string()],
            },
            WTableLevel1Entity {
                entity_type: WTableLevel1EntityType::BodyLine,
                value: vec!["Bread & Butter".to_string()],
            },
        ]);

        assert_eq!(
            html,
            "<table class=\"wikitable\"><thead><tr><th colspan=\"6\">Shopping List</th></tr></thead><tbody><tr><td>Bread & Butter</td></tr></tbody></table>"
        );
    }
}
use crate::{utils, wiki_tables::parser::{WTableLevel1Entity, WTableLevel1EntityType}};

fn compile(entities: Vec<WTableLevel1Entity>) -> String {
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

            table += "</tr><thead>";
        }

        i+=1;
    }


    return table;
}
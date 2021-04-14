use std::env;
use std::error::Error;
use std::net::ToSocketAddrs;

pub fn to_snake_name(name: &str) -> String {
    let chs = name.chars();
    let mut new_name = String::new();
    let mut index = 0;
    let chs_len = name.len();
    for x in chs {
        if x.is_uppercase() {
            if index != 0 && (index + 1) != chs_len {
                new_name.push_str("_");
            }
            new_name.push_str(x.to_lowercase().to_string().as_str());
        } else {
            new_name.push(x);
        }
        index += 1;
    }
    new_name
}

pub trait Entity {

    fn name() -> String{
        let type_name = std::any::type_name::<Self>();
        let mut name = type_name.to_string();
        let names: Vec<&str> = name.split("::").collect();
        name = names.get(names.len() - 1).unwrap_or(&"").to_string();
        let mut pre = env::var("TABLE_NAMESPACE").unwrap_or("t".to_string());
        name = to_snake_name(&name);
        pre + "_" + name.as_str()
    }

    fn id_name() -> String{
        "id".to_string()
    }


    // fn create() -> Result<String , >{
    //     let name = Self::name();
    //     Ok(name);
    // }


}
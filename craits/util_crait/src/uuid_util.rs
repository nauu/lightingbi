use uuid::Uuid;

pub fn get_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn get_short_uuid() -> String {
    let uuid = get_uuid();
    let vec : Vec<&str> = uuid.split("-").collect();
    vec[0].to_string()
}

#[cfg(test)]
mod tests {
    use crate::uuid_util::{get_uuid, get_short_uuid};

    #[test]
    pub fn test_uuid(){
        let uuid = get_uuid();
        println!("uuid is {}" , uuid);
        let short_uuid = get_short_uuid();
        println!("short uuid is {}" , short_uuid);
    }
}
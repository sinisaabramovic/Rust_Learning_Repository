use std::cmp::Ordering;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct FooModel {
    pub name: String,
    pub id_num: u32
}

impl FooModel {
    pub fn print(&self) {
        println!("Name: {}, id: {}", self.name, self.id_num);
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_id(&self) -> u32 {
        self.id_num
    }
}

impl Clone for FooModel {
    fn clone(&self) -> FooModel {
        FooModel { name: self.get_name(), id_num: self.get_id() }
    }
}

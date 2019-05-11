use std::cmp::Ordering;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct FooModel {
    pub name: String,
    pub id_num: usize
}

impl FooModel {
    pub fn print(&self) {
        println!("Name: {}, id: {}", self.name, self.id_num);
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_id(&self) -> usize {
        self.id_num
    }

    pub fn isEqual(&self, element: FooModel) -> bool {
        return self.name == element.name && self.id_num == element.id_num;
    }
}

impl Clone for FooModel {
    fn clone(&self) -> FooModel {
        FooModel { name: self.get_name(), id_num: self.get_id() }
    }
}

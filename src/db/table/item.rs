pub trait TableId
where
    Self: Clone + Copy + PartialEq + Eq,
{
}
impl<T> TableId for T where T: Clone + Copy + PartialEq + Eq {}

pub trait DbTableItem {
    type Id: TableId = i32;

    fn get_id(&self) -> Self::Id;
}

pub trait DbTableUpdateItem {
    type Id: TableId = i32;

    fn get_id(&self) -> Self::Id;
}

pub trait DbTableNewItem {}

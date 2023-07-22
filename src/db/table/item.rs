pub trait TableId
where
    Self: Clone + Copy + PartialEq,
{
}
impl<T> TableId for T where T: Clone + Copy + PartialEq {}

pub trait DbTableItem {
    type Id: TableId = i32;

    fn get_id(&self) -> Self::Id;
}

pub trait DbTableUpdateItem {
    type Id: TableId = i32;

    fn get_id(&self) -> Self::Id;
}

pub trait DbTableNewItem {}

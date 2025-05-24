#[derive(Debug)]
pub enum Statement {
    CreateTable(Box<CreateTable>),
    Insert(Box<Insert>),
    Select(Box<Select>),
}

#[derive(Debug)]
pub struct CreateTable {
    pub name: String,
    pub cols: Vec<ColumnDef>,
}

#[derive(Debug)]
pub struct ColumnDef {
    pub name: String,
    pub data_type: String,
}

#[derive(Debug)]
pub struct Insert {
    pub table: String,
    pub cols: Vec<String>,
    pub values: Vec<Value>,
}

#[derive(Debug)]
pub enum Projection {
    All,
    Columns(Vec<String>),
}

#[derive(Debug)]
pub struct Select {
    pub projection: Projection,
    pub table: String,
}

#[derive(Debug)]
pub enum Value {
    Int(i64),
    Text(String),
}


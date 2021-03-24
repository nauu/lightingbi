#[derive(Debug, Clone)]
pub struct QueryBuilder {
    rows: Vec<Dimension>,
    columns: Vec<Dimension>,
    measures: Vec<Measure>,
    orders: Vec<Order>,
    filters: Vec<Measure>,
    table: String,
}

impl QueryBuilder {
    pub fn new() -> QueryBuilder {
        QueryBuilder {
            rows: vec![],
            columns: vec![],
            measures: vec![],
            orders: vec![],
            filters: vec![],
            table: String::new(),
        }
    }

    pub fn table(mut self, table: String) -> Self {
        self.table = table;
        self
    }

    pub fn row(mut self, rows: &mut Vec<Dimension>) -> Self {
        self.rows.append(rows);
        self
    }

    pub fn col(mut self, columns: &mut Vec<Dimension>) -> Self {
        self.columns.append(columns);
        self
    }

    pub fn meas(mut self, measures: &mut Vec<Measure>) -> Self {
        self.measures.append(measures);
        self
    }

    pub fn get_meas(&self) -> &Vec<Measure> {
        &self.measures
    }

    pub fn get_rows(&self) -> &Vec<Dimension> {
        &self.rows
    }

    pub fn get_cols(&self) -> &Vec<Dimension> {
        &self.columns
    }

    pub fn get_table(&self) -> &String {
        &self.table
    }

    pub fn get_rows_and_cols(&mut self) -> Vec<Dimension> {
        let mut res = Vec::new();
        res.append(&mut self.rows);
        res.append(&mut self.columns);
        res
    }

    pub fn order(mut self, orders: &mut Vec<Order>) -> Self {
        self.orders.append(orders);
        self
    }

    pub fn filter(&self, filters: Vec<&str>) -> &Self {
        self
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub field_name: String,
    pub field_type: DataType,
    pub display_name: String,
}

impl Field {
    pub fn new(field_name: String, field_type: DataType, mut display_name: String) -> Self {
        if display_name.is_empty() {
            display_name = field_name.to_owned();
        }
        Field {
            field_name,
            field_type,
            display_name,
        }
    }
}

///维度
#[derive(Debug, Clone)]
pub struct Dimension {
    pub dimension_type: DimensionType,
    pub field: Field,
}

impl Dimension {
    pub fn new_row(field: Field) -> Dimension {
        Dimension {
            dimension_type: DimensionType::Row,
            field,
        }
    }

    pub fn new_col(field: Field) -> Dimension {
        Dimension {
            dimension_type: DimensionType::Column,
            field,
        }
    }
}

///度量
#[derive(Debug, Clone)]
pub struct Measure {
    pub field: Field,
    pub measure_type: MeasureFn,
}

impl Measure {
    pub fn new(field: Field, measure_type: MeasureFn) -> Measure {
        Measure {
            field,
            measure_type,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Order {
    field: Field,
    order_type: OrderType,
}

impl Order {
    pub fn new(field: Field) -> Self {
        Order {
            field,
            order_type: OrderType::ASC,
        }
    }

    fn new_with_order(field: Field, order_type: OrderType) -> Self {
        Order { field, order_type }
    }
}

pub struct Filter {
    field: Field,
}

impl Filter {
    //=
    fn eq(self, f1: Field, f2: Field) {}
    // !=
    fn ne(self, f1: Field, f2: Field) {}

    //>
    fn gt(self, f1: Field, f2: Field) {}
    //<
    fn lt(self, f1: Field, f2: Field) {}

    //>=
    fn ge(self, f1: Field, f2: Field) {}

    //<=
    fn le(self, f1: Field, f2: Field) {}

    fn and() {}

    fn or() {}
}

#[derive(Debug, Copy, Clone)]
pub enum DimensionType {
    Row,
    Column,
}

#[derive(Debug, Copy, Clone)]
pub enum DataType {
    Text,
    Number,
    Date,
}

#[derive(Debug, Copy, Clone)]
pub enum OrderType {
    ASC,
    DESC,
}

#[derive(Debug, Copy, Clone)]
pub enum MeasureFn {
    SUM,
    MAX,
    MIN,
    AVG,
    COUNT,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let f1 = Field::new(String::from("field1"), DataType::Text, String::from("单位"));
        let f2 = Field::new(String::from("field2"), DataType::Text, String::from("员工"));
        let f3 = Field::new(String::from("field3"), DataType::Date, String::from("时间"));
        let f4 = Field::new(
            String::from("field4"),
            DataType::Number,
            String::from("人数"),
        );
        let f5 = Field::new(
            String::from("field5"),
            DataType::Number,
            String::from("价格"),
        );
        let f6 = Field::new(
            String::from("field6"),
            DataType::Number,
            String::from("数量"),
        );

        let qb = QueryBuilder::new()
            .table(String::from("tb1"))
            .row(&mut vec![Dimension::new_row(f1), Dimension::new_row(f3)])
            .col(&mut vec![Dimension::new_col(f2), Dimension::new_col(f4)])
            .meas(&mut vec![
                Measure::new(f5, MeasureFn::SUM),
                Measure::new(f6.clone(), MeasureFn::MAX),
            ])
            .order(&mut vec![Order::new(f6)]);

        println!("{:?}", qb);
    }
}

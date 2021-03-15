#[derive(Debug, Clone)]
pub struct QueryBuilder {
    rows: Vec<Dimension>,
    columns: Vec<Dimension>,
    measures: Vec<Measure>,
}

impl QueryBuilder {
    pub fn new() -> QueryBuilder {
        QueryBuilder {
            rows: vec![],
            columns: vec![],
            measures: vec![],
        }
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

    pub fn order(&self, orders: Vec<&str>) -> &Self {
        self
    }

    pub fn filter(&self, filters: Vec<&str>) -> &Self {
        self
    }
}

///维度
#[derive(Debug, Clone)]
pub struct Dimension {
    dimension_type: DimensionType,
    field_name: String,
    field_type: DataType,
}

impl Dimension {
    pub fn new_row(field_name: String, field_type: DataType) -> Dimension {
        Dimension {
            dimension_type: DimensionType::Row,
            field_name,
            field_type,
        }
    }

    pub fn new_col(field_name: String, field_type: DataType) -> Dimension {
        Dimension {
            dimension_type: DimensionType::Column,
            field_name,
            field_type,
        }
    }
}

//度量
#[derive(Debug, Clone)]
pub struct Measure {
    measure_name: String,
    measure_type: DataType,
}

impl Measure {
    pub fn new(measure_name: String, measure_type: DataType) -> Measure {
        Measure {
            measure_name,
            measure_type,
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let qb = QueryBuilder::new()
            .row(&mut vec![
                Dimension::new_row(String::from("row1"), DataType::Text),
                Dimension::new_row(String::from("row2"), DataType::Date),
            ])
            .col(&mut vec![
                Dimension::new_col(String::from("col1"), DataType::Text),
                Dimension::new_col(String::from("col2"), DataType::Date),
                Dimension::new_col(String::from("col3"), DataType::Number),
            ])
            .meas(&mut vec![
                Measure::new(String::from("val1"), DataType::Number),
                Measure::new(String::from("val2"), DataType::Number),
                Measure::new(String::from("val3"), DataType::Number),
            ]);

        println!("{:?}", qb);
    }
}

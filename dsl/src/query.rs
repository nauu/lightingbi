#[derive(Debug, Copy, Clone)]
pub struct Query {
    //name: String,
    dimension: Dimension,
    measure: Measure,
}

impl Query {
    pub fn new(dimension: Dimension, measure: Measure) -> Query {
        Query {
            // name,
            dimension,
            measure,
        }
    }

    // fn select(&self) -> Query {
    //     Query {
    //         name,
    //         dimension,
    //         measure,
    //     }
    // }

    fn dim(&self) -> Query {
        Query {
            // name: self.name,
            dimension: self.dimension,
            measure: self.measure,
        }
    }

    fn meas(&self) -> Query {
        Query {
            dimension: self.dimension,
            measure: self.measure,
        }
    }

    fn order(&self) -> Query {
        Query {
            dimension: self.dimension,
            measure: self.measure,
        }
    }

    fn filter(&self) -> Query {
        Query {
            dimension: self.dimension,
            measure: self.measure,
        }
    }
}

///维度
#[derive(Debug, Copy, Clone)]
pub struct Dimension {
    // name: String,
}

//度量
#[derive(Debug, Copy, Clone)]
pub struct Measure {
    // name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let d = Dimension {
            //name: "".to_string(),
        };
        let m = Measure {
          //  name: "".to_string(),
        };

        let q = Query::new(d, m);
        q.dim().meas().order().filter();
        println!("{:?}", q);

        assert_eq!(2 + 2, 4);
    }
}

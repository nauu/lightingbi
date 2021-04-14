use std::collections::HashMap;

struct FormulaEngine {
    id: String,
    vars: String,
    pool: String,
}

impl FormulaEngine {
    ///创建一个随机id的新实例
    fn new() -> Self {
        //random id
        let id = String::from("11111");
        Self {
            id,
            vars: "".to_string(),
            pool: "".to_string(),
        }
    }

    ///根据id返回一个已经存在的实例
    fn form(id: String) -> Self {
        Self {
            id,
            vars: "".to_string(),
            pool: "".to_string(),
        }
    }

    ///执行计算
    fn run(&self, config: HashMap<String, String>) {}

    ///设置变量公式
    fn var(&mut self, formula: &str) {
        self.vars.push_str(formula);
        self.vars.push_str("##");
    }

    ///返回公式的树形依赖结构
    fn tree() {}

    ///检查公式是否有循环依赖
    fn check_cycle() {}

    ///打印公式信息
    fn print(&self) {
        println!("{}", self.vars);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashMap;

    // a=10##b=20##f=getvalue(1,2,3,4,'abc')+1##c=$a+$b##g=$c*$f##
    #[test]
    fn it_works() {
        let mut fe = FormulaEngine::new();
        // let mut fe = FormulaEngine::form(String::from("6666"));

        fe.var("a={?}");
        fe.var("b=20");
        fe.var("f=getvalue(1,2,3,4,'abc')+1");
        fe.var("c=[a]+[b]");
        fe.var("g=[c]*[f]");

        let mut map = HashMap::<String, String>::new();
        map.insert(String::from("a"), String::from("20"));

        fe.run(map);
        fe.print();

        println!("id:{} ", fe.id);
    }
}

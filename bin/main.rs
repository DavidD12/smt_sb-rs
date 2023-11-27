use smt_sb::*;
fn main() {
    // let mut smt = SmtBridge::new("z3", vec!["-in"], Some("log.smt".to_string())).unwrap();
    let mut smt = SmtBridge::default();
    smt.set_option("print-success", "false").unwrap();

    smt.declare_const("x", "Int").unwrap();
    smt.declare_const("y", "Int").unwrap();

    smt.assert("(> x y)").unwrap();
    smt.assert("(> y 10)").unwrap();

    let result = smt.check_sat().unwrap();
    println!("result = {}", result);

    if result == SatResult::Sat {
        let model = smt.get_model().unwrap();
        println!("model = {}", model);
        let x = smt.eval("x").unwrap();
        let y = smt.eval("y").unwrap();
        println!("x = {}", x);
        println!("y = {}", y);
    }
}

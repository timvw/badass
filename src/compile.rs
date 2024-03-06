use minijinja::{context, path_loader, Environment, Error};
use std::ffi::OsString;
use std::fs;

pub fn do_compile() {
    println!("compiling all the templates...");

    let files = fs::read_dir("./demo/models").unwrap();
    for fileResult in files {
        let file = fileResult.unwrap();
        //compile_sql_template(file.file_name())
        //file.file_type().unwrap().
    }
}

fn compile_sql_template(file_name: OsString) {}

fn builtin_ref(arg1: &str, arg2: Option<&str>, arg3: Option<&str>) -> Result<String, Error> {
    let (model, schema, version) = if arg3.is_some() {
        (arg2.unwrap(), arg1, arg3.unwrap())
    } else if arg2.is_some() {
        (arg2.unwrap(), arg1, "1")
    } else {
        (arg1, "default", "1")
    };

    let fqn = format!("{}.{}@{}", schema, model, version);
    Ok(fqn)
}

fn mainx() {
    let mut env = Environment::new();
    //env.add_function("ref", builtin_ref2);
    env.add_function("ref", builtin_ref);
    //env.set_loader(path_loader("demo/macros"));

    // load macros...

    /*
    let files = fs::read_dir("./demo/macros").unwrap();
    for fileResult in files {
        let file = fileResult.unwrap();
        env.add_template(file.file_name())
    }*/

    env.set_loader(path_loader("demo/models"));

    //env.te

    // let tmpl = env.get_template("order_payment_method_amounts.sql").unwrap();
    let tmpl = env.get_template("interactions.sql").unwrap();
    //tmpl.instructions_and_blocks()

    let ctx = context! {};
    let rendered = tmpl.render(ctx).unwrap();
    println!("rendered the template to {}", rendered);

    println!("Bye.")
}

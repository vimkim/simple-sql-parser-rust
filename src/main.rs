pub mod ast;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub sql); // synthesized by LALRPOP

fn main() {
    let inputs = [
        "CREATE TABLE tbl (id INT);",
        "INSERT INTO tbl(id) VALUES (5);",
        "SELECT * FROM tbl;",
    ];

    for &sql in &inputs {
        match sql::StatementParser::new().parse(sql) {
            Ok(stmt) => println!("Parsed: {:#?}", stmt),
            Err(e)   => eprintln!("Error parsing `{}`: {}", sql, e),
        }
    }
}

use std::env;
use std::time::Instant;
use oracle::{Connection, Result, Version};

fn main() -> Result<()> {
    println!("\nINFO  OraMeter meassures the execution time of an Oracle database statement (including network data transfer).");
   
    let connect = match env::var_os("ORA_CONNECT") {
       Some(v) => v.into_string().unwrap(),
       None => panic!("ERROR Environment variable ORA_CONNECT is not set!")
    };
    let username = match env::var_os("ORA_USERNAME") {
       Some(v) => v.into_string().unwrap(),
       None => panic!("ERROR Environment variable ORA_USERNAME is not set!")
    };
    let password = match env::var_os("ORA_PASSWORD") {
       Some(v) => v.into_string().unwrap(),
       None => panic!("ERROR Environment variable ORA_PASSWORD is not set!")
    };
    let statement = match env::var_os("ORA_STATEMENT") {
       Some(v) => v.into_string().unwrap(),
       None => panic!("ERROR Environment variable ORA_STATEMENT is not set!")
    };
    println!("INFO  ORA_CONNECT: '{}'", connect);
    println!("INFO  ORA_USERNAME: '{}'", username);
    println!("INFO  ORA_PASSWORD: ***is set***");
    println!("INFO  ORA_STATEMENT: '{}'", statement);

    let client_version = Version::client()?;
    println!("INFO  Database Client Version: '{}'", client_version);


    let now1 = Instant::now();
     
    let conn = Connection::connect(username, password, connect)?;
    let (server_version, banner) = conn.server_version()?;

    println!("INFO  Database Server Version: '{}'", server_version);
    println!("INFO  Server Banner: '{}'", banner);

    let now2 = Instant::now();
    println!("DEBUG building statement");
    let mut stmt = conn.statement(&statement).build()?;

    println!("DEBUG executing SQL query");
    let rows = stmt.query(&[])?;

    println!("DEBUG columns info");
    for col_info in rows.column_info() {
        println!("INFO  Column: '{}'", col_info.name())
    }

    println!("DEBUG reading rows");
    let now3 = Instant::now();
    let mut count = 0;
    let mut chars = 0;
    for row in rows {
        count += 1;
        for val in row?.sql_values() {
            let formatted = format!("{}", val);
            chars += formatted.len();
            //println!("DEBUG  size: {}", formatted.len());
        }
    }
    let elapsed2 = now2.elapsed();
    let elapsed3 = now3.elapsed();
    println!("INFO  results");
    println!("INFO    number of rows read                                              : {} rows", count);
    println!("INFO    data read/downloaded                                             : {} characters", chars);
    println!("INFO    duration for reading data                                        : {} ms", elapsed3.as_millis()); 
    println!("INFO    duration for building statement, executing query and reading data: {} ms", elapsed2.as_millis()); 
    conn.close()?;
    let elapsed1 = now1.elapsed();
    println!("INFO    duration of database connection                                  : {} ms", elapsed1.as_millis()); 
    println!("INFO  Program finished.");
    Ok(())
}

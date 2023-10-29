
use diesel::prelude::*;
use diesel::dsl::sql_query;
use diesel::pg::PgConnection;
use std::io::{self};


#[derive(QueryableByName)]
struct ColumnInfo {
    #[diesel(sql_type = diesel::sql_types::Text)]
    column_name: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    data_type: String,
}

pub fn establish_connection(database_url: &String) -> PgConnection {
    
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
const POSTGRES_TO_TYPESCRIPT: &[(&str, &str)] = &[
    ("TEXT", "string"),
    ("VARCHAR", "string"),
    ("UUID", "string"),
    ("CHARACTER VARYING", "string"),
    ("INTEGER", "number"),
    ("REAL", "number"),
    ("DOUBLE PRECISION", "number"),
    ("BOOLEAN", "boolean"),
    ("ARRAY", "Array"),
    ("ENUM", "Enum"),
    ("NULL", "Null"),
    ("JSON", "object"),
    ("JSONB", "object"),
    ("FUNCTION", "Function"),
    ("TIMESTAMP WITH TIME ZONE", "Date"),
    ("TIMESTAMP WITHOUT TIME ZONE", "Date"),
];

fn get_typescript_type(postgres_type: &str) -> &str {
    let upper_postgres_type = postgres_type.to_uppercase();
    for &(postgres, typescript) in POSTGRES_TO_TYPESCRIPT.iter() {
        if postgres == upper_postgres_type {
            return typescript; 
        }
    }
    "any"
}
fn main() -> io::Result<()>{
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Uso: {} <DATABASE_URL> <TABLE>", args[0]);
        return Ok(());
    }
    let database_url = &args[1];
    let table = &args[2];
    println!("{} {}",database_url, table);
    let mut conn = establish_connection(database_url);
    let query = sql_query(format!("SELECT column_name, data_type, is_nullable FROM information_schema.columns WHERE table_name = '{}'", table));

    // Ejecutar la consulta y cargar los resultados en un vector de instancias del modelo ColumnInfo
    let results = query.load::<ColumnInfo>(&mut conn).expect("Error loading column info");
    let mut type_map = Vec::new();
    // Imprimir los resultados
    for column in results {
        let data_types = get_typescript_type(&column.data_type);
        type_map.push(format!("    {}: {}", column.column_name, data_types));
    }
    let type_defs_str = type_map.join(",\n");
    let type_script_type = format!("type {}Type = {{\n{}\n}};", table, type_defs_str);
    println!("{}", type_script_type);
    Ok(())
}

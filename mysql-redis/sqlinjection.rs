use mysql::*;
use mysql::prelude::*;
use redis::Commands;

fn main() -> Result<()> {
    // Conexión a MySQL
    let url = "mysql://user:password@localhost:3306/database";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    // Consulta MySQL parametrizada
    let query = r"SELECT userId, host, created FROM connections WHERE status = ? AND created > ?";
    let params = (10, "2022-04-02");
    let results: Vec<(u32, String, String)> = conn.exec(query, params)?;

    // Conexión a Redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    // Arreglo de datos almacenado en Redis
    let block_host: Vec<String> = con.smembers("blockHost")?;

    // Verificación de resultados
    for (_, host, _) in results {
        if block_host.contains(&host) {
            println!("Block");
        } else {
            println!("Accept");
        }
    }

    Ok(())
}

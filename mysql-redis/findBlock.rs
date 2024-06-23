use mysql::*;
use mysql::prelude::*;
use redis::Commands;

fn main() -> Result<()> {
    // Conexión a MySQL
    let url = "mysql://user:password@localhost:3306/database";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    // Consulta MySQL
    let results: Vec<(u32, String, String)> = conn.query("SELECT userId, host, created FROM connections WHERE status = 10 AND created > '2022-04-02'")?;

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

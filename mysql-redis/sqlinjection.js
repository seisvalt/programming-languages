const mysql = require('mysql2/promise');
const redis = require('redis');
const { promisify } = require('util');

(async () => {
    try {
        // Conexión a MySQL
        const mysqlConnection = await mysql.createConnection({
            host: 'localhost',
            user: 'user',
            password: 'password',
            database: 'database'
        });

        // Consulta MySQL parametrizada
        const [rows] = await mysqlConnection.execute(
            "SELECT userId, host, created FROM connections WHERE status = ? AND created > ?",
            [10, "2022-04-02"]
        );

        // Conexión a Redis
        const redisClient = redis.createClient({
            host: 'localhost',
            port: 6379
        });

        const smembersAsync = promisify(redisClient.smembers).bind(redisClient);

        // Obtener datos del arreglo en Redis
        const blockHost = await smembersAsync('blockHost');

        // Verificación de resultados
        for (const row of rows) {
            const host = row.host;
            if (blockHost.includes(host)) {
                console.log("Block");
            } else {
                console.log("Accept");
            }
        }

        // Cerrar conexiones
        await mysqlConnection.end();
        redisClient.quit();

    } catch (error) {
        console.error(error);
    }
})();

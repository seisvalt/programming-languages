import mysql.connector
import redis

# Conexión a MySQL
mydb = mysql.connector.connect(
    host="localhost",
    user="user",
    password="password",
    database="database"
)

mycursor = mydb.cursor(prepared=True)
query = "SELECT userId, host, created FROM connections WHERE status = %s AND created > %s"
params = (10, "2022-04-02")
mycursor.execute(query, params)
results = mycursor.fetchall()

# Conexión a Redis
r = redis.Redis(host='localhost', port=6379, db=0)

block_host = r.smembers("blockHost")

# Verificación de resultados
for userId, host, created in results:
    if host.encode() in block_host:
        print("Block")
    else:
        print("Accept")

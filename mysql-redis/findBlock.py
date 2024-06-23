import mysql.connector
import redis

# Conexión a MySQL
mydb = mysql.connector.connect(
    host="localhost",
    user="user",
    password="password",
    database="database"
)

mycursor = mydb.cursor()
mycursor.execute("SELECT userId, host, created FROM connections WHERE status = 10 AND created > '2022-04-02'")
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

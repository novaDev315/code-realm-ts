# Reference solutions for Chapter 7


def create_user_table() -> str:
    """Return SQL to create users table with id, name, email, created_at"""
    return """CREATE TABLE users (
  id INT PRIMARY KEY AUTO_INCREMENT,
  name VARCHAR(255) NOT NULL,
  email VARCHAR(255) UNIQUE NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)"""


def select_active_users() -> str:
    """Return SQL to select all active users ordered by created_at"""
    return "SELECT * FROM users WHERE active = true ORDER BY created_at ASC"


def join_users_orders() -> str:
    """Return SQL to join users with their orders and count total orders"""
    return """SELECT users.id, users.name, COUNT(orders.id) as total_orders
FROM users
LEFT JOIN orders ON users.id = orders.user_id
GROUP BY users.id, users.name"""


def create_index() -> str:
    """Return SQL to create index on users.email"""
    return "CREATE INDEX idx_user_email ON users(email)"

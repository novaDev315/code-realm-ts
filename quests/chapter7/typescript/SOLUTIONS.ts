// Reference solutions for Chapter 7

export function createUserTable(): string {
  return `CREATE TABLE users (
  id INT PRIMARY KEY AUTO_INCREMENT,
  name VARCHAR(255) NOT NULL,
  email VARCHAR(255) UNIQUE NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)`;
}

export function selectActiveUsers(): string {
  return `SELECT * FROM users WHERE active = true ORDER BY created_at ASC`;
}

export function joinUsersOrders(): string {
  return `SELECT users.id, users.name, COUNT(orders.id) as total_orders
FROM users
LEFT JOIN orders ON users.id = orders.user_id
GROUP BY users.id, users.name`;
}

export function createIndex(): string {
  return `CREATE INDEX idx_user_email ON users(email)`;
}

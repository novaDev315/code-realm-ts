// Reference solutions for Chapter 7

function createUserTable() {
  return `CREATE TABLE users (
  id INT PRIMARY KEY AUTO_INCREMENT,
  name VARCHAR(255) NOT NULL,
  email VARCHAR(255) UNIQUE NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)`;
}

function selectActiveUsers() {
  return `SELECT * FROM users WHERE active = true ORDER BY created_at ASC`;
}

function joinUsersOrders() {
  return `SELECT users.id, users.name, COUNT(orders.id) as total_orders
FROM users
LEFT JOIN orders ON users.id = orders.user_id
GROUP BY users.id, users.name`;
}

function createIndex() {
  return `CREATE INDEX idx_user_email ON users(email)`;
}

module.exports = { createUserTable, selectActiveUsers, joinUsersOrders, createIndex };

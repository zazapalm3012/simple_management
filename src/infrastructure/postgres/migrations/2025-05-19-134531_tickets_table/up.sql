-- Your SQL goes here
CREATE TABLE tickets (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id),
  subject VARCHAR(255) NOT NULL,
  issue_type VARCHAR(30) NOT NULL,
  description TEXT,
  ticket_file VARCHAR(1000),
  status VARCHAR(20),
  priority VARCHAR(20) DEFAULT 'not_assigned',
  reject_message TEXT,
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  updated_at TIMESTAMP NOT NULL DEFAULT now()
);
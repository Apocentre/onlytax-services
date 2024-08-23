-- Your SQL goes here

CREATE TABLE accounts (
  address VARCHAR(44) PRIMARY KEY,
  soft_delete BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);

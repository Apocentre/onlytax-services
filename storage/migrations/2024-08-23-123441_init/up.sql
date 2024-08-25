-- Your SQL goes here

CREATE TABLE accounts (
  address VARCHAR(44) PRIMARY KEY,
  soft_delete BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);

CREATE TABLE tokens (
  address VARCHAR(44) PRIMARY KEY
);

CREATE TABLE collect_transactions (
  id SERIAL PRIMARY KEY,
  withdraw_withheld_authority VARCHAR(44) NOT NULL REFERENCES accounts(address),
  token VARCHAR(44) NOT NULL REFERENCES tokens(address),
  batch_size INT NOT NULL,
  tx_signature VARCHAR,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
  
  UNIQUE(tx_signature)
);

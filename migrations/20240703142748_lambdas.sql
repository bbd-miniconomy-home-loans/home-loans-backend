-- Add migration script here

CREATE TABLE lambdas (
  lambdas_id serial PRIMARY KEY,
  tax_id varchar,
  account_balance_cents integer NOT NULL,
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);
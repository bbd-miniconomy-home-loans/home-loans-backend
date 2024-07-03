-- Add migration script here
-- Create enum type for loan status
CREATE TYPE loan_status_enum AS ENUM ('pending', 'approved', 'rejected', 'completed');

-- Create persona table
CREATE TABLE persona (
  persona_id varchar PRIMARY KEY NOT NULL,
  is_active boolean NOT NULL DEFAULT true,
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create loan table
CREATE TABLE loan (
  loan_id serial PRIMARY KEY,
  persona_id varchar NOT NULL,
  loan_amount_cents integer NOT NULL,
  installment_amount_cents integer NOT NULL,
  loan_status loan_status_enum NOT NULL DEFAULT 'pending',
  interest_rate DECIMAL(5, 2) NOT NULL,
  approval_date timestamp,
  completion_date timestamp,
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_persona
    FOREIGN KEY(persona_id) 
    REFERENCES persona(persona_id)
    ON DELETE CASCADE
);

-- Create property table
CREATE TABLE property (
  property_id serial PRIMARY KEY,
  persona_id varchar NOT NULL,
  property_price integer NOT NULL,
  is_active boolean NOT NULL DEFAULT true,
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_persona
    FOREIGN KEY(persona_id) 
    REFERENCES persona(persona_id)
    ON DELETE CASCADE
);

CREATE TABLE profits (
    profit_id SERIAL PRIMARY KEY,
    amount_cents integer NOT NULL,
    profit_date timestamp
);

CREATE TABLE SARS (
  sars_id SERIAL PRIMARY KEY,
  taxable_amount_cents integer NOT NULL,
  tax_amount_cents integer NOT NULL
);

-- Index foreign keys for performance
CREATE INDEX idx_loan_persona_id ON loan(persona_id);
CREATE INDEX idx_property_persona_id ON property(persona_id);
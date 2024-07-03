-- Create enum type for loan status
CREATE TYPE loan_status_enum AS ENUM ('pending', 'approved', 'rejected', 'completed');

-- Create persona table
CREATE TABLE persona (
  persona_id serial PRIMARY KEY,
  persona_name varchar(250),
  is_active boolean NOT NULL DEFAULT true,
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create loan table
CREATE TABLE loan (
  loan_id serial PRIMARY KEY,
  persona_id integer NOT NULL,
  loan_amount decimal(10, 2) NOT NULL,
  loan_status loan_status_enum NOT NULL DEFAULT 'pending',
  interest_rate decimal(5, 2) NOT NULL,
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
  persona_id integer NOT NULL,
  property_price decimal(10, 2) NOT NULL,
  is_active boolean NOT NULL DEFAULT true,
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_persona
    FOREIGN KEY(persona_id) 
    REFERENCES persona(persona_id)
    ON DELETE CASCADE
);

-- Index foreign keys for performance
CREATE INDEX idx_loan_persona_id ON loan(persona_id);
CREATE INDEX idx_property_persona_id ON property(persona_id);
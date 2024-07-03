-- Add migration script here

-- Seed data

-- Insert personas
INSERT INTO persona (persona_id, is_active, created_at) VALUES 
('p1', true, CURRENT_TIMESTAMP),
('p2', true, CURRENT_TIMESTAMP),
('p3', false, CURRENT_TIMESTAMP);

-- Insert loans
INSERT INTO loan (persona_id, loan_amount_cents, installment_amount_cents, loan_status, interest_rate, approval_date, created_at) VALUES
('p1', 1000000, 100000, 'approved', 5.00, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('p2', 2000000, 200000, 'pending', 4.50, NULL, CURRENT_TIMESTAMP),
('p3', 1500000, 150000, 'rejected', 6.00, NULL, CURRENT_TIMESTAMP);

-- Insert properties
INSERT INTO property (persona_id, property_price, is_active, created_at) VALUES
('p1', 3000000, true, CURRENT_TIMESTAMP),
('p2', 4000000, true, CURRENT_TIMESTAMP),
('p3', 5000000, false, CURRENT_TIMESTAMP);

-- Insert profits
INSERT INTO profits (amount_cents, profit_date) VALUES
(100000, CURRENT_TIMESTAMP),
(200000, CURRENT_TIMESTAMP),
(300000, CURRENT_TIMESTAMP);

-- Insert SARS data
INSERT INTO SARS (taxable_amount_cents, tax_amount_cents) VALUES
(500000, 75000),
(1000000, 150000),
(1500000, 225000);
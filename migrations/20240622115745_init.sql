-- Add migration script here
--     Thinking that we can have a message table that will be able to implement Change Data Capture (Cough Cough design pattern)
create table if not exists messages
(
    message_id VARCHAR(100) primary key,
    name       varchar not null,
    message    varchar not null
);

create table if not exists messages_cdc
(
    cdc_id         SERIAL PRIMARY KEY,
    message_id     VARCHAR(100),
    operation_type VARCHAR(10),
    timestamp      TIMESTAMP,
    name_before    VARCHAR,
    message_before VARCHAR,
    name_after     VARCHAR,
    message_after  VARCHAR
);


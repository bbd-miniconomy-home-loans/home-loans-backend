-- Add migration script here
--     Thinking that we can have a message table that will be able to implement Change Data Capture (Cough Cough design pattern)
/*create table if not exists messages (
            message_id int primary key,
            name varchar not null,
            message varchar not null,
            last_updated date not null default current_date
);*/
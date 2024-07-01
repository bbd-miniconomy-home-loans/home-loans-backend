CREATE
    OR REPLACE FUNCTION capture_changes() RETURNS TRIGGER AS
$$
BEGIN
    IF (TG_OP = 'DELETE') THEN
        INSERT INTO messages_cdc (message_id, operation_type, timestamp, name_before, message_before)
        VALUES (OLD.message_id, 'DELETE', NOW(), OLD.name, OLD.message);
    ELSIF (TG_OP = 'UPDATE')
    THEN
        INSERT INTO messages_cdc (message_id, operation_type, timestamp, name_before, message_before, name_after,
                                  message_after)
        VALUES (NEW.message_id, 'UPDATE', NOW(), OLD.name, OLD.message, NEW.name, NEW.message);
    ELSIF (TG_OP = 'INSERT')
    THEN
        INSERT INTO messages_cdc (message_id, operation_type, timestamp, name_after, message_after)
        VALUES (NEW.message_id, 'INSERT', NOW(), NEW.name, NEW.message);
    END
        IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS messages_trigger ON MESSAGES;

CREATE TRIGGER messages_trigger
    AFTER INSERT OR UPDATE OR DELETE
    ON MESSAGES
    FOR EACH ROW
EXECUTE FUNCTION capture_changes();


-- Create role if not exists
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_roles WHERE rolname = 'r_indexer'
    ) THEN
        CREATE ROLE r_indexer;
    END IF;
END
$$;

-- Create user if not exists
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_roles WHERE rolname = 'indexer'
    ) THEN
        CREATE USER indexer;
    END IF;
END
$$;

-- Grant role
GRANT r_indexer TO indexer;

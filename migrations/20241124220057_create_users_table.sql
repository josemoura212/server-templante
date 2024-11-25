-- Add migration script here
CREATE TABLE users (
    user_id uuid PRIMARY KEY,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255),
    email VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    admin BOOLEAN NOT NULL DEFAULT FALSE,
    plan VARCHAR(255) NOT NULL DEFAULT 'free',
    photo TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('brt'::text, now()),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('brt'::text, now()),

    constraint unique_user_email unique(email)
);


CREATE INDEX idx_email_busca On users(email);
-- CREATE INDEX CONCURRENTLY IF NOT EXISTS IDX_EMAIL_BUSCA_TGRM ON users USING GIST (email GIST_TRGM_OPS(SIGLEN=64));

create or replace function update_updated_at_column() returns trigger language plpgsql as
$$
begin 
	new.updated_at = now();
	return new;
end;
$$;

create or replace trigger updated_at_users before UPDATE on users 
for each row execute procedure update_updated_at_column();
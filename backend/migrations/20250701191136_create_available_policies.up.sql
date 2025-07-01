CREATE TABLE available_policies (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    payout DOUBLE PRECISION NOT NULL,
    duration INTEGER NOT NULL,
    event_type VARCHAR(255) NOT NULL,
    threshold DOUBLE PRECISION NOT NULL
);
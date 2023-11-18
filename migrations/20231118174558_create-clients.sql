-- Table for storing OIDC clients
CREATE TABLE clients (
    id INTEGER PRIMARY KEY NOT NULL,
    uuid VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255),
    secret VARCHAR(255) NOT NULL,
    redirect_uris VARCHAR(255) NOT NULL,
    grant_types VARCHAR(255) NOT NULL,
    response_types VARCHAR(255) NOT NULL,
    scope VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

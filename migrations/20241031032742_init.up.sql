-- Enable UUID extension if not already enabled
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create the teams table with UUID
CREATE TABLE teams (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    azure_id VARCHAR,
    name VARCHAR NOT NULL,
    description VARCHAR
);

-- Create indexes for teams
CREATE INDEX idx_teams_name ON teams(name);
CREATE INDEX idx_teams_azure_id ON teams(azure_id);

-- Create the users table with UUID
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    azure_id VARCHAR,
    name VARCHAR,
    email VARCHAR,
    team_id UUID,
    FOREIGN KEY (team_id) REFERENCES teams(id)
);

-- Create indexes for users
CREATE INDEX idx_users_team_id ON users(team_id);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_azure_id ON users(azure_id);

-- Create the projects table with UUID
CREATE TABLE projects (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    azure_id VARCHAR,
    name VARCHAR,
    description VARCHAR,
    url VARCHAR,
    template VARCHAR,
    begin_date TIMESTAMP,
    end_date TIMESTAMP,
    team_id UUID,
    FOREIGN KEY (team_id) REFERENCES teams(id)
);

-- Create indexes for projects
CREATE INDEX idx_projects_team_id ON projects(team_id);
CREATE INDEX idx_projects_begin_date ON projects(begin_date);
CREATE INDEX idx_projects_end_date ON projects(end_date);
CREATE INDEX idx_projects_azure_id ON projects(azure_id);

-- Create the work_items table with UUID
CREATE TABLE work_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    azure_id VARCHAR,
    title VARCHAR NOT NULL,
    type VARCHAR NOT NULL,
    state VARCHAR NOT NULL,
    project UUID NOT NULL,
    assigned_to_id UUID,
    created_by_id UUID NOT NULL,
    created_date TIMESTAMP NOT NULL,
    changed_date TIMESTAMP,
    priority INTEGER,
    severity VARCHAR,
    description VARCHAR,
    area_path VARCHAR,
    iteration_path VARCHAR,
    parent_id UUID,
    tags VARCHAR[],
    url VARCHAR NOT NULL,
    FOREIGN KEY (assigned_to_id) REFERENCES users(id),
    FOREIGN KEY (created_by_id) REFERENCES users(id),
    FOREIGN KEY (project) REFERENCES projects(id),
    FOREIGN KEY (parent_id) REFERENCES work_items(id)
);

-- Create indexes for work_items
CREATE INDEX idx_work_items_assigned_to ON work_items(assigned_to_id);
CREATE INDEX idx_work_items_created_by ON work_items(created_by_id);
CREATE INDEX idx_work_items_project ON work_items(project);
CREATE INDEX idx_work_items_state ON work_items(state);
CREATE INDEX idx_work_items_type ON work_items(type);
CREATE INDEX idx_work_items_created_date ON work_items(created_date);
CREATE INDEX idx_work_items_azure_id ON work_items(azure_id);
CREATE INDEX idx_work_items_parent_id ON work_items(parent_id);

-- Create the team_users junction table with UUID
CREATE TABLE team_users (
    team_id UUID NOT NULL,
    user_id UUID NOT NULL,
    PRIMARY KEY (team_id, user_id),
    FOREIGN KEY (team_id) REFERENCES teams(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create indexes for the junction table
CREATE INDEX idx_team_users_team_id ON team_users(team_id);
CREATE INDEX idx_team_users_user_id ON team_users(user_id);

-- Keep the notification table with integer ID as specified
CREATE TABLE notification (
    id SERIAL PRIMARY KEY,
    subject VARCHAR,
    sender_id UUID NOT NULL,
    reciever_id UUID NOT NULL,
    message VARCHAR,
    creation_time TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    closed BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (sender_id) REFERENCES users(id),
    FOREIGN KEY (reciever_id) REFERENCES users(id)
);

-- Create indexes for notifications
CREATE INDEX idx_notification_sender ON notification(sender_id);
CREATE INDEX idx_notification_receiver ON notification(reciever_id);
CREATE INDEX idx_notification_creation_time ON notification(creation_time);
CREATE INDEX idx_notification_closed ON notification(closed);

-- Add comments to document the schema
COMMENT ON TABLE teams IS 'Teams table storing team information with UUID primary key and Azure ID reference';
COMMENT ON COLUMN teams.azure_id IS 'Reference ID from Azure DevOps';

COMMENT ON TABLE users IS 'Users table storing user information with UUID primary key and Azure ID reference';
COMMENT ON COLUMN users.azure_id IS 'Reference ID from Azure DevOps';

COMMENT ON TABLE projects IS 'Projects table storing project information with UUID primary key and Azure ID reference';
COMMENT ON COLUMN projects.azure_id IS 'Reference ID from Azure DevOps';

COMMENT ON TABLE work_items IS 'Work items table with UUID primary key and Azure ID reference';
COMMENT ON COLUMN work_items.azure_id IS 'Reference ID from Azure DevOps';

COMMENT ON TABLE team_users IS 'Junction table for teams-users many-to-many relationship using UUIDs';
COMMENT ON TABLE notification IS 'Notifications table for system messages between users';
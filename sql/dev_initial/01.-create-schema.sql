CREATE Table if not EXISTS users (
    id serial primary key,
    username varchar(80) not null unique,
    password varchar(80) not null,
    deleted_at Timestamptz default null,
    token text default null
);



create table if not EXISTS tasks (
    id serial primary key,
    priority varchar(4)  default null,
    title varchar(255) not null,
    completed_at Timestamptz default null,
    description Text default null,
    deleted_at Timestamptz default null,
    user_id  integer default null,
    is_default boolean default FALSE,
    Constraint fk_user FOREIGN KEY (user_id) references users(id)
);
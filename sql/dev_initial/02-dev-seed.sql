

Insert into users 
(username, password) values
('deleteuser', 'xyz12234');

Insert into tasks
(title, deleted_at, user_id) values
('my deleted task', now(), (select id from users where user name = 'deleteuser'));


Insert into tasks
(priority, title, description, is_default) values
('A', 'this is the task title', 'this is the description', true),
('B', 'se my details for by clickign me', 'this is description can changed', true)
;
SELECT users.id as id, created_at, role_name as "role: Role", username, updated_at
FROM users
         INNER JOIN user_roles ON users.id = user_roles.user_id
         INNER JOIN roles on user_roles.user_id = roles.id
where users.id = $1

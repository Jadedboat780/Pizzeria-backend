UPDATE Users
SET username   = COALESCE($1, username),
    email      = COALESCE($2, email),
    password   = COALESCE($3, password),
    address    = COALESCE($4, address),
    phone      = COALESCE($5, phone),
    avatar_url = COALESCE($6, avatar_url)
WHERE id = $7;
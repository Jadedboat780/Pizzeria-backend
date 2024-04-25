UPDATE Users
SET username   = $1,
    email      = $2,
    password   = $3,
    address    = $4,
    phone      = $5,
    avatar_url = $6
WHERE id = $7;
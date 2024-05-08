UPDATE Pizzas
SET title       = $1,
    ingredients = $2,
    price       = $3,
    image_url   = $4
WHERE id = $5;
UPDATE Pizzas
SET title       =  COALESCE($1,title),
    ingredients =  COALESCE($2, ingredients),
    price       =  COALESCE($3, price),
    image_url   =  COALESCE($4, image_url)
WHERE id = $5;
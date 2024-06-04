-- Example data from book: 
-- Insert item tags
INSERT INTO item_tags (name, inserted_at, updated_at) VALUES
('Vegetarian', NOW(), NOW()),
('Vegan', NOW(), NOW()),
('Gluten Free', NOW(), NOW());

-- Retrieve the inserted Vegetarian tag id
DO $$ 
DECLARE 
  vegetarian_id INTEGER;
BEGIN
  SELECT id INTO vegetarian_id FROM item_tags WHERE name = 'Vegetarian';

  -- Insert categories
  INSERT INTO categories (name, inserted_at, updated_at) VALUES
  ('Sandwiches', NOW(), NOW()),
  ('Sides', NOW(), NOW()),
  ('Beverages', NOW(), NOW());

  -- Retrieve the inserted category ids
  DECLARE
    sandwiches_id INTEGER;
    sides_id INTEGER;
    beverages_id INTEGER;
  BEGIN
    SELECT id INTO sandwiches_id FROM categories WHERE name = 'Sandwiches';
    SELECT id INTO sides_id FROM categories WHERE name = 'Sides';
    SELECT id INTO beverages_id FROM categories WHERE name = 'Beverages';

    -- Insert items for Sandwiches category
    INSERT INTO items (name, price, category_id, inserted_at, updated_at) VALUES
    ('Reuben', 4.50, sandwiches_id, NOW(), NOW()),
    ('Croque Monsieur', 5.50, sandwiches_id, NOW(), NOW()),
    ('Muffuletta', 5.50, sandwiches_id, NOW(), NOW()),
    ('Bánh mì', 4.50, sandwiches_id, NOW(), NOW());

    -- Insert Vada Pav with Vegetarian tag
    INSERT INTO items (name, price, category_id, inserted_at, updated_at) VALUES
    ('Vada Pav', 4.50, sandwiches_id, NOW(), NOW());

    -- Retrieve the inserted Vada Pav id
    DECLARE
      vada_pav_id INTEGER;
    BEGIN
      SELECT id INTO vada_pav_id FROM items WHERE name = 'Vada Pav';

      -- Insert taggings for Vada Pav
      INSERT INTO items_taggings (item_id, item_tag_id) VALUES
      (vada_pav_id, vegetarian_id);
    END;

    -- Insert items for Sides category
    INSERT INTO items (name, price, category_id, inserted_at, updated_at) VALUES
    ('French Fries', 2.50, sides_id, NOW(), NOW()),
    ('Papadum', 1.25, sides_id, NOW(), NOW()),
    ('Pasta Salad', 2.50, sides_id, NOW(), NOW());

    -- Insert items for Beverages category
    INSERT INTO items (name, price, category_id, inserted_at, updated_at) VALUES
    ('Water', 0, beverages_id, NOW(), NOW()),
    ('Soft Drink', 1.5, beverages_id, NOW(), NOW()),
    ('Lemonade', 1.25, beverages_id, NOW(), NOW()),
    ('Masala Chai', 1.5, beverages_id, NOW(), NOW()),
    ('Vanilla Milkshake', 3.0, beverages_id, NOW(), NOW()),
    ('Chocolate Milkshake', 3.0, beverages_id, NOW(), NOW());
  END;
END;
$$;
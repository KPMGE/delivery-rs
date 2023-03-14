CREATE TABLE IF NOT EXISTS routes ( 
  id serial PRIMARY KEY,
  title TEXT NOT NULL,
  start_position_id INTEGER UNIQUE REFERENCES positions(id),
  end_position_id INTEGER UNIQUE REFERENCES positions(id)
);

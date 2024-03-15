DROP TABLE IF EXISTS frame;

CREATE TABLE if not exists frame (
  id INT GENERATED ALWAYS AS IDENTITY,
  project_id INT,
  ship_id INT NOT NULL,
  index INT NOT NULL, 
  key TEXT NOT NULL,
  value FLOAT8 NOT NULL,
  CONSTRAINT frame_pk PRIMARY KEY (id),
  CONSTRAINT frame_index_unique UNIQUE (ship_id, index, key),
  CONSTRAINT frame_key_check CHECK(char_length(key) <= 50)
);

INSERT INTO frame
  (project_id, ship_id, index, key, value)
VALUES
  (NULL, 1, 0, 'x', 0),
  (NULL, 1, 0, 'delta_x', 5.87),
  (NULL, 1, 1, 'x', 5.87),
  (NULL, 1, 1, 'delta_x', 5.87);

SELECT * FROM frame WHERE ship_id=1;

TRUNCATE TABLE frame;
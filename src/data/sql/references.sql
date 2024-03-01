ALTER TABLE ship_parameters ADD FOREIGN KEY (ship_id) REFERENCES ship(ship_id);
ALTER TABLE center_waterline ADD FOREIGN KEY (ship_id) REFERENCES ship(ship_id);
ALTER TABLE rad_long ADD FOREIGN KEY (ship_id) REFERENCES ship(ship_id);
ALTER TABLE mean_draught ADD FOREIGN KEY (ship_id) REFERENCES ship(ship_id);
ALTER TABLE center_shift ADD FOREIGN KEY (ship_id) REFERENCES ship(ship_id);
ALTER TABLE frame ADD FOREIGN KEY (ship_id) REFERENCES ship(ship_id);
ALTER TABLE load_space ADD FOREIGN KEY (ship_id) REFERENCES ship(ship_id);
ALTER TABLE tank ADD FOREIGN KEY (ship_id) REFERENCES ship(ship_id);
ALTER TABLE frame_area ADD FOREIGN KEY (frame_id) REFERENCES frame(id);
ALTER TABLE tank_center ADD FOREIGN KEY (tank_id) REFERENCES tank(id);
ALTER TABLE tank_inertia ADD FOREIGN KEY (tank_id) REFERENCES tank(id);
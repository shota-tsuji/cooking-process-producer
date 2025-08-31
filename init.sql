insert into recipes (title, description) values("カレー", "野菜を煮込んで作る");
insert into recipes (title, description) values("きんぴらごぼう", "人参とごぼうを切って炒める");

insert into resources (name, amount) values("人手", 1);
insert into resources (name, amount) values("コンロ(なべ・フライパン)", 2);

insert into steps (id, recipe_id, description, resource_id, order_number, duration) values(UUID(), 1, "野菜を切る", 1, 0, 5);
insert into steps (id, recipe_id, description, resource_id, order_number, duration) values(UUID(), 1, "煮込む", 2, 1, 25);
insert into steps (id, recipe_id, description, resource_id, order_number, duration) values(UUID(), 2, "野菜を切る", 1, 0, 5);
insert into steps (id, recipe_id, description, resource_id, order_number, duration) values(UUID(), 2, "煮る", 2, 1, 10);

insert into processes (name) values("process-0");

insert into process_regsitrations (process_id, recipe_id) values(1, 1);
insert into process_regsitrations (process_id, recipe_id) values(1, 2);

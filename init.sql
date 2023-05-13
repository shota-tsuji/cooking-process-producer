insert into recipes (id, title, description) values("00000000-0000-0000-0000-000000000000", "カレー", "野菜を煮込んで作る");
insert into recipes (id, title, description) values("11111111-1111-1111-1111-111111111111", "きんぴらごぼう", "人参とごぼうを切って炒める");

insert into resources (name, amount) values("人手", 1);
insert into resources (name, amount) values("コンロ(なべ・フライパン)", 2);

insert into steps (id, recipe_id, description, resource_id, order_number, duration) values(UUID(), "00000000-0000-0000-0000-000000000000", "野菜を切る", 1, 0, 5);
insert into steps (id, recipe_id, description, resource_id, order_number, duration) values(UUID(), "00000000-0000-0000-0000-000000000000", "煮込む", 2, 1, 25);

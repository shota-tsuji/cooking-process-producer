insert into recipes (id, title, description) values(0, "カレー", "野菜を煮込んで作る");

insert into resources (id, name, amount) values(0, "人手", 1);
insert into resources (id, name, amount) values(1, "コンロ(なべ・フライパン)", 2);

insert into tasks (id, recipe_id, description, resource_id, duration) values(0, 0, "野菜を切る", 0, 5);
insert into tasks (id, recipe_id, description, resource_id, duration) values(1, 0, "煮込む", 1, 25);

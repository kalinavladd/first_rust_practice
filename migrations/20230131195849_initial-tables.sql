create table users (
  id bigserial primary key,
  first_name varchar(255),
  last_name varchar(255)
);

create table articles (
  id bigserial primary key,
  user_id bigint not null,
  title varchar(120) not null,
  content varchar(255),
  constraint fk_user foreign key (user_id) references users(id) on delete cascade
);
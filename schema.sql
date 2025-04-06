create table pages (
  id serial primary key
  , title varchar(512) not null
  , body text not null
  , create_time timestamp
  , update_time timestamp
);

create table page_revisions (
  id serial primary key
  , page_id int not null
  , body text not null
  , author varchar(256) not null
  , create_time timestamp
);

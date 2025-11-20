-- Your SQL goes here
create table tasks (
  id integer primary key autoincrement not null, 
  name char(50) not null, 
  description text not null, 
  priority integer not null, 
  planned_from char(10) not null,
  planned_duration integer not null,
  real_from char(10),
  real_duration integer
);

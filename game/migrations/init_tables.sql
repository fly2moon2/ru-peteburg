-- Add migration script here
CREATE TABLE IF NOT EXISTS uuser (
   id bigserial PRIMARY KEY,
   name VARCHAR (80) UNIQUE NOT NULL,
   display_name VARCHAR (80),
   isactive BOOLEAN DEFAULT TRUE
); 

CREATE TABLE IF NOT EXISTS ulogin (
   id bigserial PRIMARY KEY,
   uuser_id bigint NOT NULL,
   login VARCHAR (50) UNIQUE NOT NULL,
   start_dte TIMESTAMP DEFAULT NOW(),
   end_dte TIMESTAMP,
   frez_start_dte TIMESTAMP,
   frez_end_dte TIMESTAMP,
   constraint fk_uuser foreign key (uuser_id) references uuser(id) on delete cascade
); 

CREATE TABLE IF NOT EXISTS upasswd (
   id bigserial PRIMARY KEY,
   ulogin_id bigint NOT NULL,
   passwd VARCHAR (255) NOT NULL,
   start_dte TIMESTAMP DEFAULT NOW(),
   end_dte TIMESTAMP,
   constraint fk_ulogin foreign key (ulogin_id) references ulogin(id) on delete cascade
); 

-- drop
DROP TABLE uuser;
DROP TABLE ulogin;
DROP TABLE upasswd;


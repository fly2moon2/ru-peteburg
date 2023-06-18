-- Add migration script here
-- user
CREATE TABLE IF NOT EXISTS uuser (
   id bigserial PRIMARY KEY,
   name VARCHAR (80) UNIQUE NOT NULL,
   display_name VARCHAR (80),
   typ VARCHAR(10) NOT NULL,
   isactive BOOLEAN DEFAULT TRUE
); 

CREATE TABLE IF NOT EXISTS ulogin (
   id bigserial PRIMARY KEY,
   uuser_id bigint NOT NULL,
   --rel_uuser bigint REFERENCES uuser,
   login VARCHAR (50) UNIQUE NOT NULL,
   typ VARCHAR(10) NOT NULL,
   start_dte TIMESTAMP DEFAULT NOW(),
   end_dte TIMESTAMP,
   friz_start_dte TIMESTAMP,
   friz_end_dte TIMESTAMP,
   constraint fk_uuser foreign key (uuser_id) references uuser(id) on delete cascade
); 

CREATE TABLE IF NOT EXISTS upasswd (
   id bigserial PRIMARY KEY,
   ulogin_id bigint NOT NULL,
   pass VARCHAR (255) NOT NULL,
   start_dte TIMESTAMP DEFAULT NOW(),
   end_dte TIMESTAMP,
   constraint fk_ulogin foreign key (ulogin_id) references ulogin(id) on delete cascade
); 

-- user role
CREATE TABLE IF NOT EXISTS urole (
   id bigserial PRIMARY KEY,
   name VARCHAR (80) UNIQUE NOT NULL,
   isactive BOOLEAN DEFAULT TRUE,
   parent_urole_id bigint NOT NULL
); 

CREATE TABLE IF NOT EXISTS uuser_urole (
   id bigserial PRIMARY KEY,
   uuser_id bigint NOT NULL REFERENCES uuser,
   urole_id bigint NOT NULL REFERENCES urole,
   constraint unique_uuser_and_urole unique (uuser_id, urole_id)
); 

-- resource
CREATE TABLE IF NOT EXISTS resauce (
   id bigserial PRIMARY KEY,
   name VARCHAR (80) UNIQUE NOT NULL,
   isactive BOOLEAN DEFAULT TRUE
); 

-- user role rights
CREATE TABLE IF NOT EXISTS uright (
   id bigserial PRIMARY KEY,
   resauce_id bigint NOT NULL REFERENCES resauce,
   urole_id bigint NOT NULL REFERENCES urole,
   right_typ VARCHAR(50) NOT NULL,
   start_dte TIMESTAMP DEFAULT NOW(),
   end_dte TIMESTAMP,
   friz_start_dte TIMESTAMP,
   friz_end_dte TIMESTAMP,
   constraint fk_resauce foreign key (resauce_id) references resauce(id) on delete cascade
); 

-- drop
DROP TABLE uright;
DROP TABLE resauce;

DROP TABLE uuser_urole;
DROP TABLE urole;

DROP TABLE upasswd;
DROP TABLE ulogin;
DROP TABLE uuser;







